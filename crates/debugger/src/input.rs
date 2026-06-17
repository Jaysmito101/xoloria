use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{Debugger, JumpTarget};
use crate::state::*;

impl Debugger {
    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.ui.input_mode {
            InputMode::GotoMemory => self.handle_input_key(key),
            InputMode::Command => self.handle_command_key(key),
            InputMode::SearchSymbols => self.handle_search_symbols_key(key),
            InputMode::Normal => match self.screen {
                Screen::Setup => self.handle_setup_key(key),
                Screen::Debug => self.handle_debug_key(key),
            },
        }
    }

    fn handle_input_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.ui.set_input_mode(InputMode::Normal);
            }
            KeyCode::Enter => {
                let input = self.ui.input_buffer_take();
                self.submit_goto_memory(&input);
            }
            KeyCode::Backspace => {
                self.ui.input_buffer_pop();
            }
            KeyCode::Char(c) if c.is_ascii_hexdigit() || c == 'x' || c == 'X' => {
                self.ui.input_buffer_push(c);
            }
            _ => {}
        }
    }

    fn handle_command_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.ui.set_input_mode(InputMode::Normal);
            }
            KeyCode::Enter => self.execute_command(),
            KeyCode::Backspace => {
                if self.ui.input_buffer_is_empty() {
                    self.ui.set_input_mode(InputMode::Normal);
                } else {
                    self.ui.input_buffer_pop();
                }
            }
            KeyCode::Char(c) => self.ui.input_buffer_push(c),
            _ => {}
        }
    }

    fn handle_search_symbols_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Enter => {
                self.ui.set_input_mode(InputMode::Normal);
            }
            KeyCode::Backspace => {
                if self.ui.input_buffer_is_empty() {
                    self.ui.set_input_mode(InputMode::Normal);
                } else {
                    self.ui.input_buffer_pop();
                }
            }
            KeyCode::Char(c) => self.ui.input_buffer_push(c),
            _ => {}
        }
        self.ui.symbols_search = self.ui.input_buffer().to_string();
        self.ui.symbols_scroll = 0;
    }

    fn handle_setup_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.ui.setup_cursor = self.ui.setup_cursor.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max = self.total_setup_fields().saturating_sub(1);
                if self.ui.setup_cursor < max {
                    self.ui.setup_cursor += 1;
                }
            }
            KeyCode::Left | KeyCode::Char('h') => self.setup_adjust(-1),
            KeyCode::Right | KeyCode::Char('l') => self.setup_adjust(1),
            KeyCode::Char('d') => self.set_hart_mode_at_cursor(HartMode::Debug),
            KeyCode::Char('r') => self.set_hart_mode_at_cursor(HartMode::Running),
            KeyCode::Char('s') => self.set_hart_mode_at_cursor(HartMode::Stalled),
            KeyCode::Enter => {
                let config = MachineConfig {
                    harts: self.config_harts,
                    memory_size: self.memory_size(),
                };
                self.rebuild_machine(config);
                self.screen = Screen::Debug;
                self.ui.selected_hart = self
                    .hart_modes
                    .iter()
                    .position(|m| *m == HartMode::Debug)
                    .unwrap_or(0);
                self.tick_count = 0;
            }
            KeyCode::Char('q') => self.running = false,
            _ => {}
        }
    }

    fn setup_adjust(&mut self, delta: i32) {
        match self.ui.setup_cursor {
            0 => self.adjust_harts(delta),
            1 => self.adjust_memory(delta),
            n => {
                let idx = n - 2;
                if idx < self.hart_modes.len() {
                    self.hart_modes[idx] = if delta > 0 {
                        self.hart_modes[idx].next()
                    } else {
                        self.hart_modes[idx].prev()
                    };
                }
            }
        }
    }

    fn set_hart_mode_at_cursor(&mut self, mode: HartMode) {
        if self.ui.setup_cursor >= 2 {
            let idx = self.ui.setup_cursor - 2;
            if idx < self.hart_modes.len() {
                self.hart_modes[idx] = mode;
            }
        }
    }

    fn handle_debug_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.running = false,

            KeyCode::Char('n') | KeyCode::Char(' ') => {
                if self.hart_modes[self.ui.selected_hart] == HartMode::Debug {
                    self.step_hart(1);
                }
            }

            KeyCode::Char('c') => self.hart_modes[self.ui.selected_hart] = HartMode::Running,
            KeyCode::Char('p') => self.hart_modes[self.ui.selected_hart] = HartMode::Debug,
            KeyCode::Char('x') => self.hart_modes[self.ui.selected_hart] = HartMode::Stalled,

            KeyCode::Tab => self.select_hart_relative(1),
            KeyCode::BackTab => self.select_hart_relative(-1),
            KeyCode::Char(ch) if ch.is_ascii_digit() && ch != '0' => {
                let idx = (ch as usize) - ('1' as usize);
                if idx < self.hart_modes.len() {
                    self.ui.selected_hart = idx;
                    self.ui.disasm_cursor = 0;
                    self.disasm_cache = None;
                }
            }

            KeyCode::Left => self.ui.panel = self.ui.panel.nav(Direction::Left),
            KeyCode::Right => self.ui.panel = self.ui.panel.nav(Direction::Right),
            KeyCode::Up => self.ui.panel = self.ui.panel.nav(Direction::Up),
            KeyCode::Down => self.ui.panel = self.ui.panel.nav(Direction::Down),
            KeyCode::Char('f') => self.ui.panel = self.ui.panel.next(),

            KeyCode::Char('j') => self.scroll(1),
            KeyCode::Char('k') => self.scroll(-1),
            KeyCode::PageUp => self.scroll(-16),
            KeyCode::PageDown => self.scroll(16),

            KeyCode::Char('g') | KeyCode::Enter => {
                if self.ui.panel == Panel::Disassembly {
                    let entries = self.disassemble_around(200);
                    let hw_pc = self.machine.as_ref().map(|m| m.harts()[self.ui.selected_hart].registers().pc()).unwrap_or(0);
                    let center_addr = self.ui.view_center_addr.unwrap_or(hw_pc);
                    let center_idx = entries.iter().position(|e| e.addr == center_addr).unwrap_or(0) as i32;
                    let abs = (center_idx + self.ui.disasm_cursor).max(0) as usize;
                    let abs = abs.min(entries.len().saturating_sub(1));
                    if let Some(entry) = entries.get(abs) {
                        if let Some(JumpTarget::Known(target_addr)) = entry.jump_target {
                            self.ui.view_history.push(entry.addr); // Push current cursor addr before jumping
                            self.ui.view_center_addr = Some(target_addr);
                            self.ui.disasm_cursor = 0;
                            self.disasm_cache = None;
                        }
                    }
                } else if self.ui.panel == Panel::Symbols {
                    let search = self.ui.symbols_search.to_lowercase();
                    let filtered: Vec<_> = self.sorted_symbols.iter().filter(|(_, name)| {
                        search.is_empty() || name.to_lowercase().contains(&search)
                    }).collect();
                    
                    let target_addr = filtered.get(self.ui.symbols_cursor).map(|t| t.0);
                    if let Some(t_addr) = target_addr {
                        let entries = self.disassemble_around(200);
                        let hw_pc = self.machine.as_ref().map(|m| m.harts()[self.ui.selected_hart].registers().pc()).unwrap_or(0);
                        let center_addr = self.ui.view_center_addr.unwrap_or(hw_pc);
                        let center_idx = entries.iter().position(|e| e.addr == center_addr).unwrap_or(0) as i32;
                        let abs = (center_idx + self.ui.disasm_cursor).max(0) as usize;
                        let abs = abs.min(entries.len().saturating_sub(1));
                        if let Some(entry) = entries.get(abs) {
                            self.ui.view_history.push(entry.addr);
                        }
                        self.ui.view_center_addr = Some(t_addr);
                        self.ui.disasm_cursor = 0;
                        self.disasm_cache = None;
                        self.ui.panel = Panel::Disassembly;
                    }
                }
            }
            KeyCode::Backspace | KeyCode::Char('u') => {
                if self.ui.panel == Panel::Disassembly {
                    if let Some(prev) = self.ui.view_history.pop() {
                        self.ui.view_center_addr = Some(prev);
                        self.ui.disasm_cursor = 0;
                        self.disasm_cache = None;
                    } else {
                        self.ui.view_center_addr = None;
                        self.ui.disasm_cursor = 0;
                        self.disasm_cache = None;
                    }
                }
            }
            KeyCode::Char('b') => {
                let entries = self.disassemble_around(200);
                let hw_pc = self.machine.as_ref().map(|m| m.harts()[self.ui.selected_hart].registers().pc()).unwrap_or(0);
                let center_addr = self.ui.view_center_addr.unwrap_or(hw_pc);
                let center_idx = entries.iter().position(|e| e.addr == center_addr).unwrap_or(0) as i32;
                let abs = (center_idx + self.ui.disasm_cursor).max(0) as usize;
                let abs = abs.min(entries.len().saturating_sub(1));
                if let Some(entry) = entries.get(abs) {
                    let addr = entry.addr;
                    self.toggle_breakpoint_at(addr);
                }
            }
            KeyCode::Char('t') => {
                self.ui.show_targets = !self.ui.show_targets;
                self.set_info(if self.ui.show_targets {
                    "Jump targets: ON"
                } else {
                    "Jump targets: OFF"
                });
            }
            KeyCode::Char('v') => {
                self.ui.console_tab = self.ui.console_tab.next();
            }
            KeyCode::Char(':') => {
                self.ui.set_input_mode(InputMode::Command);
            }
            KeyCode::Char('m') => {
                self.ui.set_input_mode(InputMode::GotoMemory);
            }
            KeyCode::Char('/') => {
                if self.ui.panel == Panel::Symbols {
                    self.ui.set_input_mode(InputMode::SearchSymbols);
                }
            }

            KeyCode::Home => {
                if let Some(m) = self.machine.as_ref() {
                    let pc = m.harts()[self.ui.selected_hart].registers().pc();
                    if self.ui.panel == Panel::Memory {
                        self.ui.memory_addr = pc;
                    } else if self.ui.panel == Panel::Disassembly {
                        let entries = self.disassemble_around(200);
                        let center_addr = self.ui.view_center_addr.unwrap_or(pc);
                        let center_idx = entries.iter().position(|e| e.addr == center_addr).unwrap_or(0) as i32;
                        let abs = (center_idx + self.ui.disasm_cursor).max(0) as usize;
                        let abs = abs.min(entries.len().saturating_sub(1));
                        if let Some(entry) = entries.get(abs) {
                            self.ui.view_history.push(entry.addr);
                        }
                        self.ui.view_center_addr = None;
                        self.ui.disasm_cursor = 0;
                        self.disasm_cache = None;
                    } else {
                        self.ui.memory_addr = pc;
                        self.ui.panel = Panel::Memory;
                    }
                }
            }

            _ => {}
        }
    }

    fn select_hart_relative(&mut self, delta: i32) {
        let len = self.hart_modes.len();
        self.ui.selected_hart =
            ((self.ui.selected_hart as i32 + delta).rem_euclid(len as i32)) as usize;
        self.ui.disasm_cursor = 0;
        self.disasm_cache = None;
    }

    fn scroll(&mut self, delta: i32) {
        match self.ui.panel {
            Panel::Disassembly => {
                self.ui.disasm_cursor += delta;
            }
            Panel::Memory => {
                let byte_delta = delta as i64 * 16;
                self.ui.memory_addr = (self.ui.memory_addr as i64 + byte_delta).max(0) as u64;
            }
            Panel::Registers => {
                self.ui.reg_scroll = (self.ui.reg_scroll as i32 + delta).max(0) as usize;
            }
            Panel::Csr => {
                self.ui.csr_scroll = (self.ui.csr_scroll as i32 + delta).max(0) as usize;
            }
            Panel::Console => {
                self.ui.console_scroll = (self.ui.console_scroll as i32 + delta).max(0) as usize;
            }
            Panel::Symbols => {
                self.ui.symbols_cursor = (self.ui.symbols_cursor as i32 + delta).max(0) as usize;
            }
        }
    }

    pub fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent) {
        use crossterm::event::{MouseButton, MouseEventKind};
        if self.screen != Screen::Debug {
            return;
        }

        let delta = match mouse.kind {
            MouseEventKind::ScrollDown => 1,
            MouseEventKind::ScrollUp => -1,
            MouseEventKind::Down(MouseButton::Left) => 0,
            _ => return,
        };

        for (panel, rect) in &self.ui.panel_rects {
            if mouse.column >= rect.x
                && mouse.column < rect.x + rect.width
                && mouse.row >= rect.y
                && mouse.row < rect.y + rect.height
            {
                self.ui.panel = *panel;

                if delta != 0 {
                    match panel {
                        Panel::Disassembly => self.ui.disasm_cursor += delta,
                        Panel::Memory => {
                            let byte_delta = delta as i64 * 16;
                            self.ui.memory_addr = (self.ui.memory_addr as i64 + byte_delta).max(0) as u64;
                        }
                        Panel::Registers => self.ui.reg_scroll = (self.ui.reg_scroll as i32 + delta).max(0) as usize,
                        Panel::Csr => self.ui.csr_scroll = (self.ui.csr_scroll as i32 + delta).max(0) as usize,
                        Panel::Console => self.ui.console_scroll = (self.ui.console_scroll as i32 + delta).max(0) as usize,
                        Panel::Symbols => self.ui.symbols_cursor = (self.ui.symbols_cursor as i32 + delta).max(0) as usize,
                    }
                } else if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                    if *panel == Panel::Symbols && mouse.row > rect.y {
                        let row_idx = (mouse.row - rect.y - 1) as usize;
                        let search = self.ui.symbols_search.to_lowercase();
                        let filtered: Vec<_> = self.sorted_symbols.iter().filter(|(_, name)| {
                            search.is_empty() || name.to_lowercase().contains(&search)
                        }).collect();
                        
                        let symbol_idx = self.ui.symbols_scroll + row_idx;
                        let target_addr = filtered.get(symbol_idx).map(|t| t.0);
                        
                        if let Some(t_addr) = target_addr {
                            self.ui.symbols_cursor = symbol_idx;
                            let entries = self.disassemble_around(200);
                            let hw_pc = self.machine.as_ref().map(|m| m.harts()[self.ui.selected_hart].registers().pc()).unwrap_or(0);
                            let center_addr = self.ui.view_center_addr.unwrap_or(hw_pc);
                            let center_idx = entries.iter().position(|e| e.addr == center_addr).unwrap_or(0) as i32;
                            let abs = (center_idx + self.ui.disasm_cursor).max(0) as usize;
                            let abs = abs.min(entries.len().saturating_sub(1));
                            if let Some(entry) = entries.get(abs) {
                                self.ui.view_history.push(entry.addr);
                            }
                            self.ui.view_center_addr = Some(t_addr);
                            self.ui.disasm_cursor = 0;
                            self.disasm_cache = None;
                            self.ui.panel = Panel::Disassembly;
                        }
                    }
                }
                break;
            }
        }
    }
}

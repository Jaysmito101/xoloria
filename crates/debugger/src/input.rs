use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{Debugger, JumpTarget};
use crate::state::*;
use crate::ui_state::{DisasmTab, SymbolsTab};

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
            KeyCode::Up => self.ui.history_up(),
            KeyCode::Down => self.ui.history_down(),
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
        if self.ui.show_help {
            match key.code {
                KeyCode::Char('?') | KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                    self.ui.show_help = false;
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    self.ui.help_scroll = self.ui.help_scroll.saturating_add(1);
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.ui.help_scroll = self.ui.help_scroll.saturating_sub(1);
                }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Char('?') => {
                self.ui.show_help = true;
                self.ui.help_scroll = 0;
            }
            KeyCode::Esc => {
                if self.ui.panel_focused {
                    self.ui.panel_focused = false;
                }
            }
            KeyCode::Char('q') => self.running = false,

            KeyCode::Char('n') | KeyCode::Char(' ') => {
                if self.hart_modes[self.ui.selected_hart] == HartMode::Debug {
                    self.step_hart(1);
                    self.ui.view_center_addr = None;
                    self.ui.disasm_cursor = 0;
                    self.disasm_cache = None;
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

            KeyCode::Left => {
                if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) || !self.ui.panel_focused {
                    self.ui.panel = self.ui.panel.nav(Direction::Left);
                }
            }
            KeyCode::Right => {
                if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) || !self.ui.panel_focused {
                    self.ui.panel = self.ui.panel.nav(Direction::Right);
                }
            }
            KeyCode::Up => {
                if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) || !self.ui.panel_focused {
                    self.ui.panel = self.ui.panel.nav(Direction::Up);
                } else {
                    self.scroll(-1);
                }
            }
            KeyCode::Down => {
                if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) || !self.ui.panel_focused {
                    self.ui.panel = self.ui.panel.nav(Direction::Down);
                } else {
                    self.scroll(1);
                }
            }
            KeyCode::Char('f') => self.ui.panel = self.ui.panel.next(),

            KeyCode::Char('j') => self.scroll(1),
            KeyCode::Char('k') => self.scroll(-1),
            KeyCode::PageUp => self.scroll(-16),
            KeyCode::PageDown => self.scroll(16),

            KeyCode::Char('g') | KeyCode::Enter => {
                if key.code == KeyCode::Enter && !self.ui.panel_focused {
                    self.ui.panel_focused = true;
                } else {
                    if self.ui.panel == Panel::Disassembly {
                        let (_target_addr, target_entry, _entries) = self.resolve_cursor_target();
                        let _hw_pc = self.machine.as_ref().map(|m| m.harts()[self.ui.selected_hart].registers().pc()).unwrap_or(0);
                        if let Some(entry) = target_entry.as_ref()
                            && let Some(JumpTarget::Known(target_addr)) = entry.jump_target
                        {
                            self.ui.view_history.push(entry.addr);
                            self.ui.view_center_addr = Some(target_addr);
                            self.ui.disasm_cursor = 0;
                            self.disasm_cache = None;
                        }
                    } else if self.ui.panel == Panel::Symbols {
                        let target_addr = if self.ui.symbols_tab == SymbolsTab::Symbols {
                            let search = self.ui.symbols_search.to_lowercase();
                            let filtered: Vec<_> = self
                                .sorted_symbols
                                .iter()
                                .filter(|(_, name)| {
                                    search.is_empty() || name.to_lowercase().contains(&search)
                                })
                                .collect();
                            filtered.get(self.ui.symbols_cursor).map(|t| t.0)
                        } else {
                            let trace_len = self.ui.trace_stack.len();
                            let cursor = self.ui.trace_cursor.min(trace_len.saturating_sub(1));
                            let idx = trace_len.saturating_sub(1).saturating_sub(cursor);
                            self.ui.trace_stack.get(idx).copied()
                        };

                        if let Some(t_addr) = target_addr {
                            let (_target_addr, target_entry, _entries) = self.resolve_cursor_target();
                            let _hw_pc = self.machine.as_ref().map(|m| m.harts()[self.ui.selected_hart].registers().pc()).unwrap_or(0);
                            if let Some(entry) = target_entry.as_ref() {
                                self.ui.view_history.push(entry.addr);
                            }
                            self.ui.view_center_addr = Some(t_addr);
                            self.ui.disasm_cursor = 0;
                            self.disasm_cache = None;
                            self.ui.panel = Panel::Disassembly;
                        }
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
            KeyCode::Char('t') => {
                if let Some(addr) = self.ui.trace_stack.pop() {
                    let hw_pc = self
                        .machine
                        .as_ref()
                        .map(|m| m.harts()[self.ui.selected_hart].registers().pc())
                        .unwrap_or(0);
                    let center_addr = self.ui.view_center_addr.unwrap_or(hw_pc);
                    self.ui.trace_forward_stack.push(center_addr);

                    let (_target_addr, target_entry, _entries) = self.resolve_cursor_target();
                    if let Some(entry) = target_entry.as_ref() {
                        self.ui.view_history.push(entry.addr);
                    }
                    self.ui.view_center_addr = Some(addr);
                    self.ui.disasm_cursor = 0;
                    self.disasm_cache = None;
                    self.ui.panel = Panel::Disassembly;
                } else {
                    self.set_info("Trace stack is empty");
                }
            }
            KeyCode::Char('T') => {
                if let Some(addr) = self.ui.trace_forward_stack.pop() {
                    let hw_pc = self
                        .machine
                        .as_ref()
                        .map(|m| m.harts()[self.ui.selected_hart].registers().pc())
                        .unwrap_or(0);
                    let center_addr = self.ui.view_center_addr.unwrap_or(hw_pc);
                    self.ui.trace_stack.push(center_addr);

                    let (_target_addr, target_entry, _entries) = self.resolve_cursor_target();
                    if let Some(entry) = target_entry.as_ref() {
                        self.ui.view_history.push(entry.addr);
                    }
                    self.ui.view_center_addr = Some(addr);
                    self.ui.disasm_cursor = 0;
                    self.disasm_cache = None;
                    self.ui.panel = Panel::Disassembly;
                } else {
                    self.set_info("Forward trace stack is empty");
                }
            }
            KeyCode::Char('b') => {
                if self.ui.panel == Panel::Disassembly && self.ui.disasm_tab == DisasmTab::Source {
                    let (target_addr, _target_entry, entries) = self.resolve_cursor_target();
                    let hw_pc = self.machine.as_ref().map(|m| m.harts()[self.ui.selected_hart].registers().pc()).unwrap_or(0);
                    
                    let target_addr = target_addr;
                    if let Some((path, _)) = self.map_addr_to_source(target_addr, Some(&entries)) {
                        let target_line = (self.ui.source_cursor + 1) as u32;
                        if let Some(addr) = self.map_source_to_addr(&path, target_line, hw_pc) {
                            self.toggle_breakpoint_at(addr);
                        } else {
                            self.set_error("No mapping for this source line");
                        }
                    } else {
                        self.set_error("No source mapping available");
                    }
                } else {
                    let (_target_addr, target_entry, _entries) = self.resolve_cursor_target();
                    let _hw_pc = self.machine.as_ref().map(|m| m.harts()[self.ui.selected_hart].registers().pc()).unwrap_or(0);
                    if let Some(entry) = target_entry.as_ref() {
                        let addr = entry.addr;
                        self.toggle_breakpoint_at(addr);
                    }
                }
            }
            KeyCode::Char('D') => {
                let count = self.breakpoints.len();
                self.breakpoints.clear();
                self.set_info(format!("Cleared {} breakpoints", count));
                self.disasm_cache = None;
            }
            KeyCode::Char('l') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                self.load_breakpoints();
            }
            KeyCode::Char('J') => {
                self.ui.show_targets = !self.ui.show_targets;
                self.set_info(if self.ui.show_targets {
                    "Jump targets: ON"
                } else {
                    "Jump targets: OFF"
                });
            }
            KeyCode::Char('s') => {
                if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                    self.save_breakpoints();
                } else if self.ui.panel == Panel::Disassembly {
                    if self.ui.disasm_tab == DisasmTab::Assembly {
                        let (target_addr, _target_entry, entries) = self.resolve_cursor_target();
                        let hw_pc = self.machine.as_ref().map(|m| m.harts()[self.ui.selected_hart].registers().pc()).unwrap_or(0);

                        let target_addr = target_addr;
                        if let Some((_, target_line)) =
                            self.map_addr_to_source(target_addr, Some(&entries))
                        {
                            self.ui.source_cursor = target_line.saturating_sub(1) as usize;
                            self.ui.disasm_tab = DisasmTab::Source;
                        } else {
                            if let Some((_, target_line)) =
                                self.map_addr_to_source(hw_pc, Some(&entries))
                            {
                                self.ui.source_cursor = target_line.saturating_sub(1) as usize;
                                self.ui.disasm_tab = DisasmTab::Source;
                                self.set_info("Selected instruction has no source, jumped to PC instead.");
                            } else {
                                self.set_info("No source mapped to this instruction or the PC.");
                            }
                        }
                    } else {
                        let (target_addr, _target_entry, entries) = self.resolve_cursor_target();
                        let hw_pc = self.machine.as_ref().map(|m| m.harts()[self.ui.selected_hart].registers().pc()).unwrap_or(0);

                        let target_addr = target_addr;
                        if let Some((path, _)) =
                            self.map_addr_to_source(target_addr, Some(&entries))
                        {
                            let target_line = (self.ui.source_cursor + 1) as u32;
                            if let Some(addr) = self.map_source_to_addr(&path, target_line, hw_pc) {
                                self.ui.view_center_addr = Some(addr);
                                self.ui.disasm_cursor = 0;
                                self.disasm_cache = None;
                                self.ui.disasm_tab = DisasmTab::Assembly;
                            } else {
                                self.ui.view_center_addr = Some(hw_pc);
                                self.ui.disasm_cursor = 0;
                                self.disasm_cache = None;
                                self.ui.disasm_tab = DisasmTab::Assembly;
                                self.set_info("Selected source line has no assembly, jumped to PC instead.");
                            }
                        } else {
                            self.ui.view_center_addr = Some(hw_pc);
                            self.ui.disasm_cursor = 0;
                            self.disasm_cache = None;
                            self.ui.disasm_tab = DisasmTab::Assembly;
                            self.set_info("Selected source line has no assembly, jumped to PC instead.");
                        }
                    }
                } else if self.ui.panel == Panel::Symbols {
                    self.ui.symbols_tab = match self.ui.symbols_tab {
                        SymbolsTab::Symbols => SymbolsTab::Trace,
                        SymbolsTab::Trace => SymbolsTab::Symbols,
                    };
                }
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
                        if self.ui.disasm_tab == DisasmTab::Source {
                            let entries = self.disassemble_around(200);
                            if let Some((_, target_line)) =
                                self.map_addr_to_source(pc, Some(&entries))
                            {
                                self.ui.source_cursor = target_line.saturating_sub(1) as usize;
                                self.ui.view_center_addr = None;
                                self.ui.disasm_cursor = 0;
                                self.disasm_cache = None;
                            } else {
                                self.ui.disasm_tab = DisasmTab::Assembly;
                                self.ui.view_center_addr = None;
                                self.ui.disasm_cursor = 0;
                                self.disasm_cache = None;
                            }
                        } else {
                            let (_target_addr, target_entry, _entries) = self.resolve_cursor_target();
                            let _hw_pc = self.machine.as_ref().map(|m| m.harts()[self.ui.selected_hart].registers().pc()).unwrap_or(0);
                            if let Some(entry) = target_entry.as_ref() {
                                self.ui.view_history.push(entry.addr);
                            }
                            self.ui.view_center_addr = None;
                            self.ui.disasm_cursor = 0;
                            self.disasm_cache = None;
                        }
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
                if self.ui.disasm_tab == DisasmTab::Source {
                    self.ui.source_cursor = (self.ui.source_cursor as i32 + delta).max(0) as usize;
                } else {
                    self.ui.disasm_cursor += delta;
                }
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
                if self.ui.symbols_tab == SymbolsTab::Symbols {
                    self.ui.symbols_cursor = (self.ui.symbols_cursor as i32 + delta).max(0) as usize;
                } else {
                    self.ui.trace_cursor = (self.ui.trace_cursor as i32 + delta).max(0) as usize;
                }
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
                    self.scroll(delta);
                } else if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                    if *panel == Panel::Symbols && mouse.row > rect.y {
                        let row_idx = (mouse.row - rect.y - 1) as usize;
                        let search = self.ui.symbols_search.to_lowercase();
                        let filtered: Vec<_> = self
                            .sorted_symbols
                            .iter()
                            .filter(|(_, name)| {
                                search.is_empty() || name.to_lowercase().contains(&search)
                            })
                            .collect();

                        let symbol_idx = self.ui.symbols_scroll + row_idx;
                        let target_addr = filtered.get(symbol_idx).map(|t| t.0);

                        if let Some(t_addr) = target_addr {
                            self.ui.symbols_cursor = symbol_idx;
                            let (_target_addr, target_entry, _entries) = self.resolve_cursor_target();
                            let _hw_pc = self.machine.as_ref().map(|m| m.harts()[self.ui.selected_hart].registers().pc()).unwrap_or(0);
                            if let Some(entry) = target_entry.as_ref() {
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

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::Debugger;
use crate::disassembly::JumpTarget;
use crate::state::*;
use crate::ui_state::{DisasmTab, SymbolsTab};

impl Debugger {
    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.ui.input_mode {
            InputMode::GotoMemory | InputMode::GotoAddress => self.handle_input_key(key),
            InputMode::Command => self.handle_command_key(key),
            InputMode::Search => self.handle_search_key(key),
            InputMode::EditWatch(idx) => self.handle_edit_watch_key(key, idx),
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
                let mode = self.ui.input_mode;
                if mode == InputMode::GotoMemory {
                    if let Ok(addr) = crate::state::parse_addr(&input) {
                        self.execute_command(crate::command::DebugCommand::GotoMemory(addr));
                    } else {
                        self.set_error("Invalid hex address");
                    }
                    self.ui.set_input_mode(InputMode::Normal);
                } else if mode == InputMode::GotoAddress {
                    if let Ok(addr) = crate::state::parse_addr(&input) {
                        self.execute_command(crate::command::DebugCommand::GotoAddress(addr));
                    } else {
                        self.set_error("Invalid hex address");
                    }
                    self.ui.set_input_mode(InputMode::Normal);
                }
            }
            KeyCode::Backspace => {
                self.ui.input_buffer_pop();
            }
            KeyCode::Left => self.ui.input_cursor_left(),
            KeyCode::Right => self.ui.input_cursor_right(),
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
            KeyCode::Enter => self.execute_input_buffer_command(),
            KeyCode::Up => self.ui.history_up(),
            KeyCode::Down => self.ui.history_down(),
            KeyCode::Backspace => {
                if self.ui.input_buffer_is_empty() {
                    self.ui.set_input_mode(InputMode::Normal);
                } else {
                    self.ui.input_buffer_pop();
                }
            }
            KeyCode::Left => self.ui.input_cursor_left(),
            KeyCode::Right => self.ui.input_cursor_right(),
            KeyCode::Char(c) => self.ui.input_buffer_push(c),
            _ => {}
        }
    }

    fn handle_search_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.ui.set_input_mode(InputMode::Normal);
            }
            KeyCode::Enter => {
                let query = self.ui.input_buffer_take();
                self.ui.set_input_mode(InputMode::Normal);
                self.ui.search.query = query.clone();
                if !query.is_empty() {
                    if self.ui.search.history.last() != Some(&query) {
                        self.ui.search.history.push(query.clone());
                    }
                    self.ui.search.history_index = None;

                    match regex::RegexBuilder::new(&query)
                        .case_insensitive(true)
                        .build()
                    {
                        Ok(re) => {
                            self.ui.search.compiled_regex = Some(re);
                            self.ui.search.is_regex_error = false;
                        }
                        Err(_) => {
                            self.ui.search.compiled_regex = None;
                            self.ui.search.is_regex_error = true;
                        }
                    }
                } else {
                    self.ui.search.compiled_regex = None;
                    self.ui.search.is_regex_error = false;
                }

                if self.ui.panel == Panel::Symbols {
                    self.ui.symbols.scroll = 0;
                    self.ui.symbols.cursor = 0;
                } else if self.ui.panel == Panel::Disassembly
                    && self.ui.disasm.tab == DisasmTab::Source
                {
                    self.search_nearest();
                }
            }
            KeyCode::Up => self.ui.search_history_up(),
            KeyCode::Down => self.ui.search_history_down(),
            KeyCode::Backspace => {
                self.ui.input_buffer_pop();
            }
            KeyCode::Left => self.ui.input_cursor_left(),
            KeyCode::Right => self.ui.input_cursor_right(),
            KeyCode::Char(c) => self.ui.input_buffer_push(c),
            _ => {}
        }
    }

    fn handle_edit_watch_key(&mut self, key: KeyEvent, idx: usize) {
        match key.code {
            KeyCode::Esc => {
                self.ui.set_input_mode(InputMode::Normal);
            }
            KeyCode::Enter => {
                let input = self.ui.input_buffer_take();
                if let Ok(val) = crate::state::parse_expr(&input) {
                    if idx < self.watches.len() {
                        let watch = self.watches[idx].clone();
                        self.do_write_memory(watch.data_type, watch.address, val);
                    }
                } else {
                    self.set_error("Invalid value");
                }
                self.ui.set_input_mode(InputMode::Normal);
            }
            KeyCode::Backspace => {
                self.ui.input_buffer_pop();
            }
            KeyCode::Left => self.ui.input_cursor_left(),
            KeyCode::Right => self.ui.input_cursor_right(),
            KeyCode::Char(c) => {
                self.ui.input_buffer_push(c);
            }
            _ => {}
        }
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
                self.execute_command(crate::command::DebugCommand::Reset);
                self.screen = Screen::Debug;
                self.ui.selected_hart = self
                    .hart_modes
                    .iter()
                    .position(|m| *m == HartMode::Debug)
                    .unwrap_or(0);
            }
            KeyCode::Char('q') => self.execute_command(crate::command::DebugCommand::Quit),
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
        if self.ui.help.show {
            match key.code {
                KeyCode::Char('?') | KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                    self.ui.help.show = false;
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    self.ui.help.scroll = self.ui.help.scroll.saturating_add(1);
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.ui.help.scroll = self.ui.help.scroll.saturating_sub(1);
                }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Char('?') => {
                self.ui.help.show = true;
                self.ui.help.scroll = 0;
            }
            KeyCode::Esc => {
                if self.ui.panel_focused {
                    self.ui.panel_focused = false;
                }
            }
            KeyCode::Char('q') => self.execute_command(crate::command::DebugCommand::Quit),

            KeyCode::F(11) | KeyCode::Char(' ') => {
                if self.ui.panel == Panel::Csr
                    && self.ui.registers_tab == crate::ui_state::RegistersTab::Watch
                {
                    if !self.watches.is_empty() {
                        let cursor = self
                            .ui
                            .watch_cursor
                            .min(self.watches.len().saturating_sub(1));
                        self.execute_command(crate::command::DebugCommand::ToggleWatchBreakpoint(
                            cursor,
                        ));
                    }
                } else if self.hart_modes[self.ui.selected_hart] == HartMode::Debug {
                    self.execute_command(crate::command::DebugCommand::Step(1));
                }
            }

            KeyCode::Char('n') => {
                self.execute_command(crate::command::DebugCommand::SearchNext(1, false));
            }
            KeyCode::Char('N') => {
                self.execute_command(crate::command::DebugCommand::SearchNext(-1, false));
            }

            KeyCode::Char('c') => self.execute_command(crate::command::DebugCommand::Continue),
            KeyCode::Char('p') => self.execute_command(crate::command::DebugCommand::Pause),

            KeyCode::Char('e') => {
                if self.ui.panel == Panel::Csr
                    && self.ui.registers_tab == crate::ui_state::RegistersTab::Watch
                    && !self.watches.is_empty()
                {
                    let cursor = self
                        .ui
                        .watch_cursor
                        .min(self.watches.len().saturating_sub(1));
                    self.ui.set_input_mode(InputMode::EditWatch(cursor));
                }
            }
            KeyCode::Delete | KeyCode::Char('d') => {
                if self.ui.panel == Panel::Csr
                    && self.ui.registers_tab == crate::ui_state::RegistersTab::Watch
                    && !self.watches.is_empty()
                {
                    let cursor = self
                        .ui
                        .watch_cursor
                        .min(self.watches.len().saturating_sub(1));
                    self.execute_command(crate::command::DebugCommand::DeleteWatchIndex(cursor));
                }
            }

            KeyCode::Tab => {
                if self.ui.panel_focused {
                    self.toggle_active_panel_tab();
                } else {
                    self.select_hart_relative(1);
                }
            }
            KeyCode::BackTab => {
                if self.ui.panel_focused {
                    self.toggle_active_panel_tab();
                } else {
                    self.select_hart_relative(-1);
                }
            }
            KeyCode::Char(ch) if ch.is_ascii_digit() && ch != '0' => {
                let idx = (ch as usize) - ('1' as usize);
                if idx < self.hart_modes.len() {
                    self.ui.selected_hart = idx;
                    self.ui.disasm.cursor = 0;
                    self.disasm_cache = None;
                }
            }

            KeyCode::Left => {
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL)
                    || !self.ui.panel_focused
                {
                    self.ui.panel = self.ui.panel.nav(Direction::Left);
                }
            }
            KeyCode::Right => {
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL)
                    || !self.ui.panel_focused
                {
                    self.ui.panel = self.ui.panel.nav(Direction::Right);
                }
            }
            KeyCode::Up => {
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL)
                    || !self.ui.panel_focused
                {
                    self.ui.panel = self.ui.panel.nav(Direction::Up);
                } else {
                    self.scroll(-1);
                }
            }
            KeyCode::Down => {
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL)
                    || !self.ui.panel_focused
                {
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
                } else if self.ui.panel == Panel::Disassembly {
                    let (_target_addr, target_entry, _entries) = self.resolve_cursor_target();
                    let _hw_pc = self
                        .machine
                        .as_ref()
                        .map(|m| m.harts[self.ui.selected_hart].registers().pc())
                        .unwrap_or(0);
                    if let Some(entry) = target_entry.as_ref()
                        && let Some(JumpTarget::Known(target_addr)) = entry.jump_target
                    {
                        self.ui.disasm.view_history.push(entry.addr);
                        self.ui.disasm.view_center_addr = Some(target_addr);
                        self.ui.disasm.cursor = 0;
                        self.disasm_cache = None;
                    }
                } else if self.ui.panel == Panel::Symbols {
                    let target_addr = if self.ui.symbols.tab == SymbolsTab::Symbols {
                        let search = self.ui.search.query.to_lowercase();
                        let filtered: Vec<_> = self
                            .sorted_symbols
                            .iter()
                            .filter(|(_, name)| {
                                search.is_empty() || name.to_lowercase().contains(&search)
                            })
                            .collect();
                        filtered.get(self.ui.symbols.cursor).map(|t| t.0)
                    } else {
                        let filtered_trace: Vec<_> = if self.ui.trace.hide_non_symbols {
                            self.ui
                                .trace
                                .stack
                                .iter()
                                .rev()
                                .filter(|e| {
                                    self.sorted_symbols
                                        .binary_search_by_key(&e.pc, |(a, _)| *a)
                                        .is_ok()
                                })
                                .collect()
                        } else {
                            self.ui.trace.stack.iter().rev().collect()
                        };
                        filtered_trace.get(self.ui.trace.cursor).map(|e| e.pc)
                    };

                    if let Some(t_addr) = target_addr {
                        let (_target_addr, target_entry, _entries) = self.resolve_cursor_target();
                        let _hw_pc = self
                            .machine
                            .as_ref()
                            .map(|m| m.harts[self.ui.selected_hart].registers().pc())
                            .unwrap_or(0);
                        if self.ui.disasm.view_center_addr == Some(t_addr) {
                            self.ui.panel = Panel::Disassembly;
                        } else {
                            if let Some(entry) = target_entry.as_ref() {
                                self.ui.disasm.view_history.push(entry.addr);
                            }
                            self.ui.disasm.view_center_addr = Some(t_addr);
                            self.ui.disasm.cursor = 0;
                            self.disasm_cache = None;
                        }
                    }
                }
            }
            KeyCode::Backspace | KeyCode::Char('u') => {
                if self.ui.panel == Panel::Disassembly {
                    if let Some(prev) = self.ui.disasm.view_history.pop() {
                        self.ui.disasm.view_center_addr = Some(prev);
                        self.ui.disasm.cursor = 0;
                        self.disasm_cache = None;
                    } else {
                        self.ui.disasm.view_center_addr = None;
                        self.ui.disasm.cursor = 0;
                        self.disasm_cache = None;
                    }
                }
            }
            KeyCode::Char('t') => {
                if let Some(entry) = self.ui.trace.stack.pop() {
                    let addr = entry.pc;
                    let hw_pc = self
                        .machine
                        .as_ref()
                        .map(|m| m.harts[self.ui.selected_hart].registers().pc())
                        .unwrap_or(0);
                    let current_sp = self
                        .machine
                        .as_ref()
                        .map(|m| m.harts[self.ui.selected_hart].registers().x()[2])
                        .unwrap_or(0);
                    let center_addr = self.ui.disasm.view_center_addr.unwrap_or(hw_pc);
                    self.ui
                        .trace
                        .forward_stack
                        .push(crate::ui_state::TraceEntry::new(center_addr, current_sp));

                    let (_target_addr, target_entry, _entries) = self.resolve_cursor_target();
                    if let Some(entry) = target_entry.as_ref() {
                        self.ui.disasm.view_history.push(entry.addr);
                    }
                    self.ui.disasm.view_center_addr = Some(addr);
                    self.ui.disasm.cursor = 0;
                    self.disasm_cache = None;
                    self.ui.panel = Panel::Disassembly;
                } else {
                    self.set_info("Trace stack is empty");
                }
            }
            KeyCode::Char('T') => {
                if let Some(entry) = self.ui.trace.forward_stack.pop() {
                    let addr = entry.pc;
                    let hw_pc = self
                        .machine
                        .as_ref()
                        .map(|m| m.harts[self.ui.selected_hart].registers().pc())
                        .unwrap_or(0);
                    let current_sp = self
                        .machine
                        .as_ref()
                        .map(|m| m.harts[self.ui.selected_hart].registers().x()[2])
                        .unwrap_or(0);
                    let center_addr = self.ui.disasm.view_center_addr.unwrap_or(hw_pc);
                    self.ui
                        .trace
                        .stack
                        .push(crate::ui_state::TraceEntry::new(center_addr, current_sp));

                    let (_target_addr, target_entry, _entries) = self.resolve_cursor_target();
                    if let Some(entry) = target_entry.as_ref() {
                        self.ui.disasm.view_history.push(entry.addr);
                    }
                    self.ui.disasm.view_center_addr = Some(addr);
                    self.ui.disasm.cursor = 0;
                    self.disasm_cache = None;
                    self.ui.panel = Panel::Disassembly;
                } else {
                    self.set_info("Forward trace stack is empty");
                }
            }
            KeyCode::Char('b') => {
                if self.ui.panel == Panel::Disassembly && self.ui.disasm.tab == DisasmTab::Source {
                    let (target_addr, _target_entry, entries) = self.resolve_cursor_target();
                    let hw_pc = self
                        .machine
                        .as_ref()
                        .map(|m| m.harts[self.ui.selected_hart].registers().pc())
                        .unwrap_or(0);

                    if let Some((path, _)) = self.map_addr_to_source(target_addr, Some(&entries)) {
                        let target_line = (self.ui.disasm.source_cursor + 1) as u32;
                        if let Some(addr) = self.map_source_to_addr(&path, target_line, hw_pc) {
                            self.execute_command(crate::command::DebugCommand::Breakpoint(Some(
                                crate::command::BreakpointTarget::Address(addr),
                            )));
                        } else {
                            self.set_error("No mapping for this source line");
                        }
                    } else {
                        self.set_error("No source mapping available");
                    }
                } else {
                    let (_target_addr, target_entry, _entries) = self.resolve_cursor_target();
                    let _hw_pc = self
                        .machine
                        .as_ref()
                        .map(|m| m.harts[self.ui.selected_hart].registers().pc())
                        .unwrap_or(0);
                    if let Some(entry) = target_entry.as_ref() {
                        let addr = entry.addr;
                        self.execute_command(crate::command::DebugCommand::Breakpoint(Some(
                            crate::command::BreakpointTarget::Address(addr),
                        )));
                    }
                }
            }
            KeyCode::Char('D') => {
                self.execute_command(crate::command::DebugCommand::Delete(
                    crate::command::DeleteTarget::All,
                ));
            }
            KeyCode::Char('h') => {
                if self.ui.panel == Panel::Symbols && self.ui.symbols.tab == SymbolsTab::Trace {
                    self.ui.trace.hide_non_symbols = !self.ui.trace.hide_non_symbols;
                    self.ui.trace.cursor = 0;
                    self.ui.trace.scroll = 0;
                }
            }

            KeyCode::Char(':') => {
                self.ui.set_input_mode(InputMode::Command);
            }
            KeyCode::Char('m') => {
                if self.ui.panel == Panel::Disassembly {
                    self.ui.set_input_mode(InputMode::GotoAddress);
                } else {
                    self.ui.set_input_mode(InputMode::GotoMemory);
                }
            }
            KeyCode::Char('/') => {
                if self.ui.panel == Panel::Symbols
                    || (self.ui.panel == Panel::Disassembly
                        && self.ui.disasm.tab == DisasmTab::Source)
                {
                    self.ui.set_input_mode(InputMode::Search);
                }
            }

            KeyCode::Home => {
                if let Some(m) = self.machine.as_ref() {
                    let pc = m.harts[self.ui.selected_hart].registers().pc();
                    if self.ui.panel == Panel::Memory {
                        self.ui.memory_addr = pc;
                    } else if self.ui.panel == Panel::Disassembly {
                        if self.ui.disasm.tab == DisasmTab::Source {
                            let entries = self.disassemble_around(200);
                            if let Some((_, target_line)) =
                                self.map_addr_to_source(pc, Some(&entries))
                            {
                                self.ui.disasm.source_cursor =
                                    target_line.saturating_sub(1) as usize;
                                self.ui.disasm.view_center_addr = None;
                                self.ui.disasm.cursor = 0;
                                self.disasm_cache = None;
                            } else {
                                self.ui.disasm.tab = DisasmTab::Assembly;
                                self.ui.disasm.view_center_addr = None;
                                self.ui.disasm.cursor = 0;
                                self.disasm_cache = None;
                            }
                        } else {
                            let (_target_addr, target_entry, _entries) =
                                self.resolve_cursor_target();
                            let _hw_pc = self
                                .machine
                                .as_ref()
                                .map(|m| m.harts[self.ui.selected_hart].registers().pc())
                                .unwrap_or(0);
                            if let Some(entry) = target_entry.as_ref() {
                                self.ui.disasm.view_history.push(entry.addr);
                            }
                            self.ui.disasm.view_center_addr = None;
                            self.ui.disasm.cursor = 0;
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
        self.ui.disasm.cursor = 0;
        self.disasm_cache = None;
    }

    fn scroll(&mut self, delta: i32) {
        match self.ui.panel {
            Panel::Disassembly => {
                if self.ui.disasm.tab == DisasmTab::Source {
                    self.ui.disasm.source_cursor =
                        (self.ui.disasm.source_cursor as i32 + delta).max(0) as usize;
                } else {
                    self.ui.disasm.cursor += delta;
                }
            }
            Panel::Memory => {
                if self.ui.memory_tab == crate::ui_state::MemoryTab::Stack {
                    self.ui.stack_scroll = (self.ui.stack_scroll as i32 + delta).max(0) as usize;
                } else {
                    let byte_delta = delta as i64 * 16;
                    self.ui.memory_addr = (self.ui.memory_addr as i64 + byte_delta).max(0) as u64;
                }
            }
            Panel::Registers => {
                self.ui.reg_scroll = (self.ui.reg_scroll as i32 + delta).max(0) as usize;
            }
            Panel::Csr => {
                if self.ui.registers_tab == crate::ui_state::RegistersTab::Watch {
                    let new_cursor = (self.ui.watch_cursor as i32 + delta).max(0) as usize;
                    let max_cursor = self.watches.len().saturating_sub(1);
                    self.ui.watch_cursor = new_cursor.min(max_cursor);
                } else {
                    self.ui.csr_scroll = (self.ui.csr_scroll as i32 + delta).max(0) as usize;
                }
            }
            Panel::Console => {
                self.ui.console.scroll = (self.ui.console.scroll as i32 + delta).max(0) as usize;
            }
            Panel::Symbols => {
                if self.ui.symbols.tab == SymbolsTab::Symbols {
                    self.ui.symbols.cursor =
                        (self.ui.symbols.cursor as i32 + delta).max(0) as usize;
                } else {
                    self.ui.trace.cursor = (self.ui.trace.cursor as i32 + delta).max(0) as usize;
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
                } else if mouse.kind == MouseEventKind::Down(MouseButton::Left)
                    && *panel == Panel::Symbols
                    && mouse.row > rect.y
                {
                    let row_idx = (mouse.row - rect.y - 1) as usize;
                    let target_addr = if self.ui.symbols.tab == SymbolsTab::Symbols {
                        let search = self.ui.search.query.to_lowercase();
                        let filtered: Vec<_> = self
                            .sorted_symbols
                            .iter()
                            .filter(|(_, name)| {
                                search.is_empty() || name.to_lowercase().contains(&search)
                            })
                            .collect();

                        let symbol_idx = self.ui.symbols.scroll + row_idx;
                        if symbol_idx < filtered.len() {
                            self.ui.symbols.cursor = symbol_idx;
                            filtered.get(symbol_idx).map(|t| t.0)
                        } else {
                            None
                        }
                    } else {
                        let filtered_trace: Vec<_> = if self.ui.trace.hide_non_symbols {
                            self.ui
                                .trace
                                .stack
                                .iter()
                                .rev()
                                .filter(|e| {
                                    self.sorted_symbols
                                        .binary_search_by_key(&e.pc, |(a, _)| *a)
                                        .is_ok()
                                })
                                .collect()
                        } else {
                            self.ui.trace.stack.iter().rev().collect()
                        };
                        let trace_idx = self.ui.trace.scroll + row_idx;
                        if trace_idx < filtered_trace.len() {
                            self.ui.trace.cursor = trace_idx;
                            filtered_trace.get(trace_idx).map(|e| e.pc)
                        } else {
                            None
                        }
                    };

                    if let Some(t_addr) = target_addr {
                        let (_target_addr, target_entry, _entries) = self.resolve_cursor_target();
                        let _hw_pc = self
                            .machine
                            .as_ref()
                            .map(|m| m.harts[self.ui.selected_hart].registers().pc())
                            .unwrap_or(0);
                        if let Some(entry) = target_entry.as_ref() {
                            self.ui.disasm.view_history.push(entry.addr);
                        }
                        self.ui.disasm.view_center_addr = Some(t_addr);
                        self.ui.disasm.cursor = 0;
                        self.disasm_cache = None;
                        self.ui.panel = Panel::Disassembly;
                    }
                }
                break;
            }
        }
    }

    fn search_nearest(&mut self) {
        let query = self.ui.search.query.clone();
        if query.is_empty() {
            return;
        }
        let compiled_regex = self.ui.search.compiled_regex.clone();

        let is_match = |text: &str| -> bool {
            if let Some(re) = &compiled_regex {
                re.is_match(text)
            } else {
                text.to_lowercase().contains(&query.to_lowercase())
            }
        };

        if self.ui.panel == Panel::Disassembly && self.ui.disasm.tab == DisasmTab::Source {
            let (target_addr, _target_entry, entries) = self.resolve_cursor_target();
            if let Some((path, _)) = self.map_addr_to_source(target_addr, Some(&entries)) {
                let start_cursor = self.ui.disasm.source_cursor;
                let mut nearest_cursor = None;
                let mut min_distance = usize::MAX;

                if let Some(lines) = self.get_source_file(&path) {
                    for (idx, line) in lines.iter().enumerate() {
                        let line_text: String = line.iter().map(|(t, _)| t.as_str()).collect();
                        if is_match(&line_text) {
                            let distance = (idx as isize - start_cursor as isize).unsigned_abs();
                            if distance < min_distance {
                                min_distance = distance;
                                nearest_cursor = Some(idx);
                            }
                        }
                    }
                }
                if let Some(c) = nearest_cursor {
                    self.ui.disasm.source_cursor = c;
                }
            }
        }
    }

    pub(crate) fn search_next(&mut self, direction: i32, inclusive: bool) {
        let query = self.ui.search.query.clone();
        if query.is_empty() {
            return;
        }
        let compiled_regex = self.ui.search.compiled_regex.clone();

        let is_match = |text: &str| -> bool {
            if let Some(re) = &compiled_regex {
                re.is_match(text)
            } else {
                text.to_lowercase().contains(&query.to_lowercase())
            }
        };

        if self.ui.panel == Panel::Disassembly && self.ui.disasm.tab == DisasmTab::Source {
            let (target_addr, _target_entry, entries) = self.resolve_cursor_target();
            if let Some((path, _)) = self.map_addr_to_source(target_addr, Some(&entries)) {
                let start_cursor = self.ui.disasm.source_cursor;
                let mut next_cursor = None;
                if let Some(lines) = self.get_source_file(&path) {
                    let mut curr = start_cursor;
                    let mut found = false;

                    if inclusive && curr < lines.len() {
                        let line_text: String =
                            lines[curr].iter().map(|(t, _)| t.as_str()).collect();
                        if is_match(&line_text) {
                            found = true;
                        }
                    }

                    if !found {
                        for _ in 0..lines.len() {
                            curr = if direction > 0 {
                                (curr + 1) % lines.len()
                            } else {
                                (curr + lines.len() - 1) % lines.len()
                            };

                            let line_text: String =
                                lines[curr].iter().map(|(t, _)| t.as_str()).collect();
                            if is_match(&line_text) {
                                found = true;
                                break;
                            }
                        }
                    }
                    if found {
                        next_cursor = Some(curr);
                    }
                }
                if let Some(c) = next_cursor {
                    self.ui.disasm.source_cursor = c;
                }
            }
        }
    }

    fn toggle_active_panel_tab(&mut self) {
        match self.ui.panel {
            Panel::Disassembly => {
                self.ui.disasm.tab = match self.ui.disasm.tab {
                    DisasmTab::Assembly => {
                        let (target_addr, _target_entry, entries) = self.resolve_cursor_target();
                        let hw_pc = self
                            .machine
                            .as_ref()
                            .map(|m| m.harts[self.ui.selected_hart].registers().pc())
                            .unwrap_or(0);
                        if let Some((_, target_line)) =
                            self.map_addr_to_source(target_addr, Some(&entries))
                        {
                            self.ui.disasm.source_cursor = target_line.saturating_sub(1) as usize;
                            DisasmTab::Source
                        } else if let Some((_, target_line)) =
                            self.map_addr_to_source(hw_pc, Some(&entries))
                        {
                            self.ui.disasm.source_cursor = target_line.saturating_sub(1) as usize;
                            self.set_info(
                                "Selected instruction has no source, jumped to PC instead.",
                            );
                            DisasmTab::Source
                        } else {
                            self.set_info("No source mapped to this instruction or the PC.");
                            DisasmTab::Assembly
                        }
                    }
                    DisasmTab::Source => {
                        let (target_addr, _target_entry, entries) = self.resolve_cursor_target();
                        let hw_pc = self
                            .machine
                            .as_ref()
                            .map(|m| m.harts[self.ui.selected_hart].registers().pc())
                            .unwrap_or(0);
                        if let Some((path, _)) =
                            self.map_addr_to_source(target_addr, Some(&entries))
                        {
                            let target_line = (self.ui.disasm.source_cursor + 1) as u32;
                            if let Some(addr) = self.map_source_to_addr(&path, target_line, hw_pc) {
                                self.ui.disasm.view_center_addr = Some(addr);
                            } else {
                                self.ui.disasm.view_center_addr = Some(hw_pc);
                                self.set_info(
                                    "Selected source line has no assembly, jumped to PC instead.",
                                );
                            }
                        } else {
                            self.ui.disasm.view_center_addr = Some(hw_pc);
                            self.set_info(
                                "Selected source line has no assembly, jumped to PC instead.",
                            );
                        }
                        self.ui.disasm.cursor = 0;
                        self.disasm_cache = None;
                        DisasmTab::Assembly
                    }
                };
            }
            Panel::Csr => {
                self.ui.registers_tab = match self.ui.registers_tab {
                    crate::ui_state::RegistersTab::Csr => crate::ui_state::RegistersTab::Watch,
                    crate::ui_state::RegistersTab::Watch => crate::ui_state::RegistersTab::Csr,
                };
            }
            Panel::Memory => {
                self.ui.memory_tab = match self.ui.memory_tab {
                    crate::ui_state::MemoryTab::Hex => crate::ui_state::MemoryTab::Stack,
                    crate::ui_state::MemoryTab::Stack => crate::ui_state::MemoryTab::Hex,
                };
            }
            Panel::Symbols => {
                self.ui.symbols.tab = match self.ui.symbols.tab {
                    SymbolsTab::Symbols => SymbolsTab::Trace,
                    SymbolsTab::Trace => SymbolsTab::Symbols,
                };
            }
            Panel::Console => {
                self.ui.console.tab = self.ui.console.tab.next();
            }
            Panel::Registers => {}
        }
    }
}

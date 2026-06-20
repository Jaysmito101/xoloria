use ratatui::layout::Rect;
use std::collections::HashMap;

use crate::state::*;

#[derive(Default)]
pub struct DisasmState {
    pub cursor: i32,
    pub tab: DisasmTab,
    pub source_scroll: usize,
    pub source_cursor: usize,
    pub show_targets: bool,
    pub view_center_addr: Option<u64>,
    pub view_history: Vec<u64>,
    pub last_target_addr: Option<u64>,
}

#[derive(Default)]
pub struct ConsoleState {
    pub scroll: usize,
    pub tab: ConsoleTab,
    pub command_history: Vec<String>,
    pub history_index: Option<usize>,
}

#[derive(Default)]
pub struct SearchState {
    pub query: String,
    pub history: Vec<String>,
    pub history_index: Option<usize>,
    pub compiled_regex: Option<regex::Regex>,
    pub is_regex_error: bool,
}

#[derive(Default)]
pub struct SymbolsState {
    pub scroll: usize,
    pub cursor: usize,
    pub tab: SymbolsTab,
}

#[derive(Default)]
pub struct TraceState {
    pub stack: Vec<u64>,
    pub forward_stack: Vec<u64>,
    pub scroll: usize,
    pub cursor: usize,
}

#[derive(Default)]
pub struct HelpState {
    pub show: bool,
    pub scroll: usize,
}

pub struct UiState {
    pub input_mode: InputMode,
    input_buffer: String,
    pub input_cursor: usize,
    pub setup_cursor: usize,
    pub panel: Panel,

    pub disasm: DisasmState,
    pub console: ConsoleState,
    pub symbols: SymbolsState,
    pub trace: TraceState,
    pub help: HelpState,
    pub search: SearchState,

    pub reg_scroll: usize,
    pub registers_tab: RegistersTab,
    pub csr_scroll: usize,
    pub watch_scroll: usize,
    pub watch_cursor: usize,
    pub memory_addr: u64,
    pub memory_tab: MemoryTab,
    pub stack_scroll: usize,
    pub selected_hart: usize,
    pub panel_rects: HashMap<Panel, Rect>,
    pub panel_focused: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DisasmTab {
    #[default]
    Assembly,
    Source,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SymbolsTab {
    #[default]
    Trace,
    Symbols,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum MemoryTab {
    #[default]
    Hex,
    Stack,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum RegistersTab {
    #[default]
    Csr,
    Watch,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            input_cursor: 0,
            setup_cursor: 0,
            panel: Panel::Disassembly,

            disasm: DisasmState::default(),
            console: ConsoleState::default(),
            symbols: SymbolsState::default(),
            trace: TraceState::default(),
            help: HelpState::default(),
            search: SearchState::default(),

            reg_scroll: 0,
            registers_tab: RegistersTab::default(),
            csr_scroll: 0,
            watch_scroll: 0,
            watch_cursor: 0,
            memory_addr: 0x8000_0000,
            memory_tab: MemoryTab::default(),
            stack_scroll: 0,
            selected_hart: 0,
            panel_rects: HashMap::new(),
            panel_focused: true,
        }
    }

    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
        self.input_buffer.clear();
        self.input_cursor = 0;
        self.console.history_index = None;
    }

    pub fn push_command_history(&mut self, cmd: String) {
        if cmd.is_empty() {
            return;
        }
        if self.console.command_history.last() != Some(&cmd) {
            self.console.command_history.push(cmd);
        }
        self.console.history_index = None;
    }

    pub fn history_up(&mut self) {
        if self.console.command_history.is_empty() {
            return;
        }
        let new_idx = match self.console.history_index {
            None => self.console.command_history.len().saturating_sub(1),
            Some(idx) => idx.saturating_sub(1),
        };
        self.console.history_index = Some(new_idx);
        self.input_buffer = self.console.command_history[new_idx].clone();
        self.input_cursor = self.input_buffer.chars().count();
    }

    pub fn history_down(&mut self) {
        if let Some(idx) = self.console.history_index {
            let new_idx = idx + 1;
            if new_idx >= self.console.command_history.len() {
                self.console.history_index = None;
                self.input_buffer.clear();
                self.input_cursor = 0;
            } else {
                self.console.history_index = Some(new_idx);
                self.input_buffer = self.console.command_history[new_idx].clone();
                self.input_cursor = self.input_buffer.chars().count();
            }
        }
    }

    pub fn search_history_up(&mut self) {
        if self.search.history.is_empty() {
            return;
        }
        let new_idx = match self.search.history_index {
            None => self.search.history.len().saturating_sub(1),
            Some(idx) => idx.saturating_sub(1),
        };
        self.search.history_index = Some(new_idx);
        self.input_buffer = self.search.history[new_idx].clone();
        self.input_cursor = self.input_buffer.chars().count();
    }

    pub fn search_history_down(&mut self) {
        if let Some(idx) = self.search.history_index {
            let new_idx = idx + 1;
            if new_idx >= self.search.history.len() {
                self.search.history_index = None;
                self.input_buffer.clear();
                self.input_cursor = 0;
            } else {
                self.search.history_index = Some(new_idx);
                self.input_buffer = self.search.history[new_idx].clone();
                self.input_cursor = self.input_buffer.chars().count();
            }
        }
    }

    pub fn input_buffer(&self) -> &str {
        &self.input_buffer
    }

    pub fn input_buffer_push(&mut self, c: char) {
        if self.input_cursor >= self.input_buffer.chars().count() {
            self.input_buffer.push(c);
            self.input_cursor = self.input_buffer.chars().count();
        } else {
            let mut chars: Vec<char> = self.input_buffer.chars().collect();
            chars.insert(self.input_cursor, c);
            self.input_buffer = chars.into_iter().collect();
            self.input_cursor += 1;
        }
    }

    pub fn input_buffer_pop(&mut self) -> Option<char> {
        if self.input_cursor == 0 {
            return None;
        }
        let mut chars: Vec<char> = self.input_buffer.chars().collect();
        let c = chars.remove(self.input_cursor - 1);
        self.input_buffer = chars.into_iter().collect();
        self.input_cursor -= 1;
        Some(c)
    }

    pub fn input_cursor_left(&mut self) {
        self.input_cursor = self.input_cursor.saturating_sub(1);
    }

    pub fn input_cursor_right(&mut self) {
        if self.input_cursor < self.input_buffer.chars().count() {
            self.input_cursor += 1;
        }
    }

    pub fn input_buffer_is_empty(&self) -> bool {
        self.input_buffer.is_empty()
    }

    pub fn input_buffer_take(&mut self) -> String {
        self.input_cursor = 0;
        std::mem::take(&mut self.input_buffer)
    }
}

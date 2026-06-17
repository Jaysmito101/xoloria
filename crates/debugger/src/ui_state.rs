use ratatui::layout::Rect;
use std::collections::HashMap;

use crate::state::*;

pub struct UiState {
    pub input_mode: InputMode,
    input_buffer: String,
    pub setup_cursor: usize,
    pub panel: Panel,
    pub disasm_cursor: i32,
    pub reg_scroll: usize,
    pub csr_scroll: usize,
    pub console_scroll: usize,
    pub console_tab: ConsoleTab,
    pub memory_addr: u64,
    pub show_targets: bool,
    pub selected_hart: usize,
    pub panel_rects: HashMap<Panel, Rect>,
    pub view_center_addr: Option<u64>,
    pub view_history: Vec<u64>,
    pub symbols_scroll: usize,
    pub symbols_cursor: usize,
    pub symbols_search: String,
    pub show_help: bool,
    pub help_scroll: usize,
    pub disasm_tab: DisasmTab,
    pub source_scroll: usize,
    pub source_cursor: usize,
    pub command_history: Vec<String>,
    pub history_index: Option<usize>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DisasmTab {
    Assembly,
    Source,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            setup_cursor: 0,
            panel: Panel::Registers,
            disasm_cursor: 0,
            reg_scroll: 0,
            csr_scroll: 0,
            console_scroll: 0,
            console_tab: ConsoleTab::Debugger,
            memory_addr: 0x80000000,
            show_targets: true,
            selected_hart: 0,
            panel_rects: HashMap::new(),
            view_center_addr: None,
            view_history: Vec::new(),
            symbols_scroll: 0,
            symbols_cursor: 0,
            symbols_search: String::new(),
            show_help: false,
            help_scroll: 0,
            disasm_tab: DisasmTab::Assembly,
            source_scroll: 0,
            source_cursor: 0,
            command_history: Vec::new(),
            history_index: None,
        }
    }

    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
        self.input_buffer.clear();
        self.history_index = None;
    }

    pub fn push_command_history(&mut self, cmd: String) {
        if cmd.is_empty() { return; }
        if self.command_history.last() != Some(&cmd) {
            self.command_history.push(cmd);
        }
        self.history_index = None;
    }

    pub fn history_up(&mut self) {
        if self.command_history.is_empty() { return; }
        let new_idx = match self.history_index {
            None => self.command_history.len().saturating_sub(1),
            Some(idx) => idx.saturating_sub(1),
        };
        self.history_index = Some(new_idx);
        self.input_buffer = self.command_history[new_idx].clone();
    }

    pub fn history_down(&mut self) {
        if let Some(idx) = self.history_index {
            let new_idx = idx + 1;
            if new_idx >= self.command_history.len() {
                self.history_index = None;
                self.input_buffer.clear();
            } else {
                self.history_index = Some(new_idx);
                self.input_buffer = self.command_history[new_idx].clone();
            }
        }
    }

    pub fn input_buffer(&self) -> &str {
        &self.input_buffer
    }

    pub fn input_buffer_push(&mut self, c: char) {
        self.input_buffer.push(c);
    }

    pub fn input_buffer_pop(&mut self) -> Option<char> {
        self.input_buffer.pop()
    }

    pub fn input_buffer_is_empty(&self) -> bool {
        self.input_buffer.is_empty()
    }

    pub fn input_buffer_take(&mut self) -> String {
        std::mem::take(&mut self.input_buffer)
    }
}

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
        }
    }

    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
        self.input_buffer.clear();
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

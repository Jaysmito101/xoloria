use crate::app::DisasmEntry;
use emulator::registers::{ControlRegisterName, GeneralRegisterName};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Tabs, Wrap},
};

use crate::app::{Debugger, JumpTarget};
use crate::state::*;
use crate::ui_state::{DisasmTab, SymbolsTab};

impl Debugger {
    pub fn render(&mut self, frame: &mut Frame) {
        match self.screen {
            Screen::Setup => self.render_setup(frame),
            Screen::Debug => self.render_debug(frame),
        }
    }

    fn render_setup(&self, frame: &mut Frame) {
        let area = frame.area();
        let outer = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(6),
                Constraint::Length(6),
            ])
            .split(area);

        let title = Paragraph::new(Line::from(vec![
            Span::styled(
                "XOLORIA",
                Style::default()
                    .fg(self.theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Debugger", Style::default().fg(Color::White)),
        ]))
        .alignment(Alignment::Center);
        frame.render_widget(title, outer[0]);

        let mut lines: Vec<Line> = vec![Line::from("")];

        let harts_label = format!("◄ {} ►", self.config_harts);
        lines.push(self.setup_field_row(" Harts", &harts_label, self.ui.setup_cursor == 0));

        let mem_label = format!("◄ {} ►", self.memory_label());
        lines.push(self.setup_field_row(" Memory", &mem_label, self.ui.setup_cursor == 1));

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            " Hart Assignment",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));

        for i in 0..self.config_harts {
            let selected = self.ui.setup_cursor == i + 2;
            let mode = self.hart_modes[i];
            let mode_color = self.theme.mode_color(mode);
            let marker = if selected { " ► " } else { " " };

            lines.push(Line::from(vec![
                Span::styled(
                    marker,
                    if selected {
                        Style::default()
                            .fg(self.theme.accent)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(self.theme.dim)
                    },
                ),
                Span::styled(format!("Hart {} ", i), Style::default().fg(Color::White)),
                Span::styled(
                    format!("◄ {:8} ►", mode),
                    Style::default().fg(mode_color).add_modifier(if selected {
                        Modifier::BOLD
                    } else {
                        Modifier::empty()
                    }),
                ),
                Span::styled(
                    if selected { " d/r/s" } else { "" },
                    Style::default().fg(self.theme.dim),
                ),
            ]));
        }

        let config = Paragraph::new(lines)
            .block(self.panel_block("Machine Configuration", true))
            .wrap(Wrap { trim: false });
        frame.render_widget(config, outer[1]);

        let help = Paragraph::new(vec![
            Line::from(vec![
                Span::styled(
                    " ↑/↓ ",
                    Style::default()
                        .fg(self.theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("Navigate "),
                Span::styled(
                    " ←/→ ",
                    Style::default()
                        .fg(self.theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("Adjust "),
                Span::styled(
                    " Enter ",
                    Style::default()
                        .fg(self.theme.highlight)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("Start"),
            ]),
            Line::from(vec![
                Span::styled(" d ", Style::default().fg(self.theme.accent)),
                Span::raw("Debug "),
                Span::styled(" r ", Style::default().fg(self.theme.running)),
                Span::raw("Running "),
                Span::styled(" s ", Style::default().fg(self.theme.stalled)),
                Span::raw("Stalled "),
                Span::styled(" q ", Style::default().fg(self.theme.error)),
                Span::raw("Quit"),
            ]),
        ])
        .block(self.panel_block("Controls", false))
        .wrap(Wrap { trim: false });
        frame.render_widget(help, outer[2]);
    }

    fn render_debug(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(8),
                Constraint::Length(6),
                Constraint::Length(1),
            ])
            .split(area);

        self.render_hart_tabs(frame, layout[0]);

        let mid = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(layout[1]);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(mid[0]);

        let lower_mid = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[2]);

        self.render_registers(frame, left_chunks[0]);
        self.render_registers_tab(frame, left_chunks[1]);
        self.render_disassembly(frame, mid[1]);
        self.render_memory(frame, lower_mid[0]);
        self.render_symbols(frame, lower_mid[1]);
        self.render_console(frame, layout[3]);
        self.render_bottom_bar(frame, layout[4]);

        if self.ui.help.show {
            self.render_help_modal(frame);
        }
    }

    fn render_help_modal(&self, frame: &mut Frame) {
        let area = frame.area();
        let modal_width = 80;
        let modal_height = 35;
        let x = area.x + (area.width.saturating_sub(modal_width)) / 2;
        let y = area.y + (area.height.saturating_sub(modal_height)) / 2;
        let modal_area = Rect::new(
            x,
            y,
            modal_width.min(area.width),
            modal_height.min(area.height),
        );

        frame.render_widget(Clear, modal_area);

        let help_text = vec![
            "=== General Navigation ===",
            " Tab / Shift+Tab : Switch active Hart",
            " 1, 2, 3... : Switch active Hart (direct)",
            " Arrows : Navigate panels (unfocused) or inside panel (focused)",
            " Ctrl+Arrows : Navigate panels (always)",
            " Enter : Focus panel",
            " Esc : Unfocus panel",
            " f : Next panel",
            " ? : Toggle this help",
            " q : Quit",
            "",
            "=== Execution ===",
            " c : Continue (Running mode)",
            " p : Pause (Debug mode)",
            " F11 / Space : Step Instruction",
            " x : Stalled mode",
            "",
            "=== Disassembly ===",
            " b : Toggle breakpoint at cursor",
            " Shift+D : Clear all breakpoints",
            " g / Enter : Follow jump / Jump to selected symbol",
            " u / Backspace : Go back in history",
            " j : Toggle jump target labels",
            " j / k : Scroll down/up",
            " PageUp/PageDown : Scroll page down/up",
            " Home : Center on current PC",
            " Click on jump : Jump to target",
            "",
            "=== Memory ===",
            " m : Goto memory address (Enter to confirm)",
            " j / k : Scroll memory by 16 bytes",
            "",
            "=== Symbols ===",
            " / : Search source/symbols",
            " n / N : Search next / previous match",
            " j / k : Navigate symbol list",
            " Enter / Click : Jump to symbol in disassembly",
            "",
            "=== Console ===",
            " v : Switch console tab (Logs vs Tracing)",
            " : : Command prompt",
            " (e.g. :bp main, :save bp, :load bp)",
            " (e.g. :read u32 0x1000)",
            " (e.g. :write u32 0x1000 = 0xff)",
        ];

        let visible_lines: Vec<Line> = help_text
            .into_iter()
            .skip(self.ui.help.scroll)
            .take((modal_area.height.saturating_sub(2)) as usize)
            .map(Line::from)
            .collect();

        let block = Block::default()
            .title(" Help (Scroll: j/k, Close: ?, Esc) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.accent));

        let paragraph = Paragraph::new(visible_lines).block(block);
        frame.render_widget(paragraph, modal_area);
    }

    fn render_hart_tabs(&self, frame: &mut Frame, area: Rect) {
        let titles: Vec<Line> = self
            .hart_modes
            .iter()
            .enumerate()
            .map(|(i, mode)| {
                let color = self.theme.mode_color(*mode);
                let priv_mode = self
                    .machine
                    .as_ref()
                    .map(|m| m.harts()[i].privilage_mode())
                    .map(|p| format!(" {}", p))
                    .unwrap_or_default();
                Line::from(vec![
                    Span::styled(format!("Hart {} ", i), Style::default().fg(Color::White)),
                    Span::styled(format!("[{}]", mode), Style::default().fg(color)),
                    Span::styled(priv_mode, Style::default().fg(self.theme.dim)),
                ])
            })
            .collect();

        let tabs = Tabs::new(titles)
            .block(self.panel_block("Harts", false))
            .select(self.ui.selected_hart)
            .highlight_style(
                Style::default()
                    .fg(self.theme.accent)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            );
        frame.render_widget(tabs, area);
    }

    fn render_registers(&mut self, frame: &mut Frame, area: Rect) {
        self.ui.panel_rects.insert(Panel::Registers, area);
        let focused = self.ui.panel == Panel::Registers;
        let Some(machine) = self.machine.as_ref() else {
            let block = self.panel_block("Registers", focused);
            frame.render_widget(block, area);
            return;
        };
        let regs = machine.harts()[self.ui.selected_hart].registers();

        let mut all_rows: Vec<Row> = Vec::with_capacity(33);

        all_rows.push(Row::new(vec![
            Cell::from(Span::styled(
                "pc",
                Style::default().fg(self.theme.highlight),
            )),
            Cell::from(Span::styled(
                format!("{:#018x}", regs.pc()),
                Style::default().fg(Color::White),
            )),
            Cell::from(Span::styled(
                format!("{}", regs.pc()),
                Style::default().fg(self.theme.dim),
            )),
        ]));

        for (i, &val) in regs.x().iter().enumerate() {
            let name = GeneralRegisterName::try_from(i as u8)
                .map(|n| format!("{}", n))
                .unwrap_or_else(|_| format!("x{}", i));
            let nz = val != 0;
            all_rows.push(Row::new(vec![
                Cell::from(Span::styled(
                    name,
                    Style::default().fg(if nz {
                        self.theme.accent
                    } else {
                        self.theme.dim
                    }),
                )),
                Cell::from(Span::styled(
                    format!("{:#018x}", val),
                    Style::default().fg(if nz { Color::White } else { self.theme.dim }),
                )),
                Cell::from(Span::styled(
                    format!("{}", val as i64),
                    Style::default().fg(self.theme.dim),
                )),
            ]));
        }

        let visible_height = area.height.saturating_sub(3) as usize;
        let max_scroll = all_rows.len().saturating_sub(visible_height);
        let scroll = self.ui.reg_scroll.min(max_scroll);
        let visible_rows: Vec<Row> = all_rows
            .into_iter()
            .skip(scroll)
            .take(visible_height)
            .collect();

        let title = if scroll > 0 {
            format!("Registers [{}/{}]", scroll, max_scroll)
        } else {
            "Registers".into()
        };

        let table = Table::new(
            visible_rows,
            [
                Constraint::Length(10),
                Constraint::Length(20),
                Constraint::Min(10),
            ],
        )
        .block(self.panel_block(&title, focused))
        .header(
            Row::new(vec!["Name", "Hex", "Decimal"]).style(
                Style::default()
                    .fg(self.theme.dim)
                    .add_modifier(Modifier::BOLD),
            ),
        );
        frame.render_widget(table, area);
    }

    fn render_tabbed_panel(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        panel: Panel,
        panel_title: &str,
        tabs: &[&str],
        active_tab: usize,
        focused: bool,
    ) -> Option<Rect> {
        self.ui.panel_rects.insert(panel, area);

        let block = self.panel_block(panel_title, focused);
        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        if inner_area.height == 0 {
            return None;
        }

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(inner_area);

        let lines: Vec<Line> = tabs
            .iter()
            .enumerate()
            .map(|(i, &t)| {
                if i == active_tab {
                    Line::from(Span::styled(
                        format!(" {} ", t),
                        Style::default()
                            .bg(self.theme.accent)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    ))
                } else {
                    Line::from(Span::styled(
                        format!(" {} ", t),
                        Style::default().fg(self.theme.dim),
                    ))
                }
            })
            .collect();

        let tabs_widget = Tabs::new(lines)
            .divider(Span::styled("│", Style::default().fg(self.theme.dim)))
            .select(active_tab)
            .highlight_style(Style::default());

        frame.render_widget(tabs_widget, layout[0]);

        if layout[1].height > 0 {
            Some(layout[1])
        } else {
            None
        }
    }

    fn render_registers_tab(&mut self, frame: &mut Frame, area: Rect) {
        let focused = self.ui.panel == Panel::Csr;
        let active_tab = match self.ui.registers_tab {
            crate::ui_state::RegistersTab::Csr => 0,
            crate::ui_state::RegistersTab::Watch => 1,
        };

        if let Some(content_area) = self.render_tabbed_panel(
            frame,
            area,
            Panel::Csr,
            "Registers",
            &["CSR", "Watch List"],
            active_tab,
            focused,
        ) {
            match self.ui.registers_tab {
                crate::ui_state::RegistersTab::Csr => self.render_csr(frame, content_area, focused),
                crate::ui_state::RegistersTab::Watch => {
                    self.render_watch_list(frame, content_area, focused)
                }
            }
        }
    }

    fn render_csr(&mut self, frame: &mut Frame, area: Rect, _focused: bool) {
        let Some(machine) = self.machine.as_ref() else {
            return;
        };
        let regs = machine.harts()[self.ui.selected_hart].registers();
        let panel_width = area.width.saturating_sub(2) as usize;

        let all_csrs: Vec<(ControlRegisterName, u64)> = regs.csrs();

        let name_col = 10usize;
        let hex_col = 20usize;
        let bin_max = panel_width.saturating_sub(name_col + hex_col + 6);

        let all_rows: Vec<Row> = all_csrs
            .iter()
            .map(|(name, val)| {
                let nz = *val != 0;
                let bin = format_binary_grouped(*val);
                let bin_display = if bin.len() > bin_max && bin_max > 3 {
                    format!("...{}", &bin[bin.len() - (bin_max - 1)..])
                } else {
                    bin
                };
                Row::new(vec![
                    Cell::from(Span::styled(
                        format!("{}", name),
                        Style::default().fg(if nz {
                            self.theme.highlight
                        } else {
                            self.theme.dim
                        }),
                    )),
                    Cell::from(Span::styled(
                        format!("{:#018x}", val),
                        Style::default().fg(if nz { Color::White } else { self.theme.dim }),
                    )),
                    Cell::from(Span::styled(
                        bin_display,
                        Style::default().fg(if nz {
                            self.theme.accent
                        } else {
                            self.theme.dim
                        }),
                    )),
                ])
            })
            .collect();

        let visible_height = area.height.saturating_sub(3) as usize;
        let max_scroll = all_rows.len().saturating_sub(visible_height);
        let scroll = self.ui.csr_scroll.min(max_scroll);
        let visible_rows: Vec<Row> = all_rows
            .into_iter()
            .skip(scroll)
            .take(visible_height)
            .collect();

        let table = Table::new(
            visible_rows,
            [
                Constraint::Length(name_col as u16),
                Constraint::Length(hex_col as u16),
                Constraint::Min(12),
            ],
        )
        .header(
            Row::new(vec!["Register", "Hex", "Binary"]).style(
                Style::default()
                    .fg(self.theme.dim)
                    .add_modifier(Modifier::BOLD),
            ),
        );
        frame.render_widget(table, area);
    }

    fn render_watch_list(&mut self, frame: &mut Frame, area: Rect, _focused: bool) {
        if self.watches.is_empty() {
            let empty =
                Paragraph::new("No watchpoints set.\nUse `watch <name> <addr> <type>` to add one.")
                    .style(Style::default().fg(self.theme.dim))
                    .alignment(Alignment::Center);
            frame.render_widget(empty, area);
            return;
        }

        let (inner, is_editing) = if let InputMode::EditWatch(idx) = self.ui.input_mode {
            let splits = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(1)])
                .split(area);

            let name = self
                .watches
                .get(idx)
                .map(|w| w.name.as_str())
                .unwrap_or("?");
            let mut spans = vec![Span::styled(
                format!(
                    " Edit {} ({:?}): ",
                    name,
                    self.watches
                        .get(idx)
                        .map(|w| &w.data_type)
                        .unwrap_or(&crate::state::DataType::U32)
                ),
                Style::default().fg(self.theme.accent),
            )];
            spans.extend(
                self.render_input_spans(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                    self.theme.accent,
                ),
            );
            spans.push(Span::styled(
                " (Enter=save, Esc=cancel)",
                Style::default().fg(self.theme.dim),
            ));
            let input_line = Line::from(spans);
            frame.render_widget(
                Paragraph::new(input_line).style(Style::default().bg(Color::Rgb(40, 40, 60))),
                splits[1],
            );
            (splits[0], true)
        } else {
            (area, false)
        };

        let visible_height = inner.height.saturating_sub(3) as usize;
        let max_scroll = self.watches.len().saturating_sub(visible_height);
        let scroll = self.ui.watch_scroll.min(max_scroll);

        let mut rows = Vec::new();
        for (i, watch) in self
            .watches
            .iter()
            .enumerate()
            .skip(scroll)
            .take(visible_height)
        {
            let is_selected = _focused && i == self.ui.watch_cursor && !is_editing;

            let mut val_bytes = vec![0u8; watch.data_type.size_bytes() as usize];
            if let Some(machine) = self.machine.as_ref() {
                let bus = machine.bus();
                val_bytes = watch.read_value(bus.as_ref());
            }

            let val_str = match watch.data_type {
                crate::state::DataType::U8 => format!("{:#04x}", val_bytes[0]),
                crate::state::DataType::U16 => format!(
                    "{:#06x}",
                    u16::from_le_bytes(val_bytes.try_into().unwrap_or([0; 2]))
                ),
                crate::state::DataType::U32 => format!(
                    "{:#010x}",
                    u32::from_le_bytes(val_bytes.try_into().unwrap_or([0; 4]))
                ),
                crate::state::DataType::U64 => format!(
                    "{:#018x}",
                    u64::from_le_bytes(val_bytes.try_into().unwrap_or([0; 8]))
                ),
                crate::state::DataType::I8 => format!("{}", val_bytes[0] as i8),
                crate::state::DataType::I16 => format!(
                    "{}",
                    i16::from_le_bytes(val_bytes.try_into().unwrap_or([0; 2]))
                ),
                crate::state::DataType::I32 => format!(
                    "{}",
                    i32::from_le_bytes(val_bytes.try_into().unwrap_or([0; 4]))
                ),
                crate::state::DataType::I64 => format!(
                    "{}",
                    i64::from_le_bytes(val_bytes.try_into().unwrap_or([0; 8]))
                ),
            };

            let break_str = if watch.break_on_change { "[x]" } else { "[ ]" };
            let style = if is_selected {
                Style::default().bg(Color::Rgb(60, 60, 80)).fg(Color::White)
            } else {
                Style::default().fg(Color::White)
            };

            rows.push(
                Row::new(vec![
                    Cell::from(Span::styled(
                        format!("{:#010x}", watch.address),
                        style.fg(self.theme.highlight),
                    )),
                    Cell::from(Span::styled(
                        watch.name.clone(),
                        style.fg(self.theme.accent),
                    )),
                    Cell::from(Span::styled(
                        format!("{}", watch.data_type),
                        style.fg(Color::Yellow),
                    )),
                    Cell::from(Span::styled(val_str, style)),
                    Cell::from(Span::styled(
                        break_str,
                        style.fg(if watch.break_on_change {
                            Color::Red
                        } else {
                            self.theme.dim
                        }),
                    )),
                ])
                .style(style),
            );
        }

        let table = Table::new(
            rows,
            [
                Constraint::Length(10),
                Constraint::Length(15),
                Constraint::Length(5),
                Constraint::Min(10),
                Constraint::Length(5),
            ],
        )
        .header(
            Row::new(vec!["Address", "Name", "Type", "Value", "Brk"]).style(
                Style::default()
                    .fg(self.theme.dim)
                    .add_modifier(Modifier::BOLD),
            ),
        );
        frame.render_widget(table, inner);
    }

    fn render_disassembly(&mut self, frame: &mut Frame, area: Rect) {
        self.ui.panel_rects.insert(Panel::Disassembly, area);
        let focused = self.ui.panel == Panel::Disassembly;
        let all_entries = self.disassemble_around(200);

        let hw_pc = self
            .machine
            .as_ref()
            .map(|m| m.harts()[self.ui.selected_hart].registers().pc())
            .unwrap_or(0);
        let center_addr = self.ui.disasm.view_center_addr.unwrap_or(hw_pc);

        let center_idx = all_entries
            .iter()
            .position(|e| e.addr == center_addr)
            .unwrap_or(0) as i32;
        let abs_cursor = (center_idx + self.ui.disasm.cursor).max(0) as usize;
        let abs_cursor = abs_cursor.min(all_entries.len().saturating_sub(1));

        let cursor_target_addr = if self.ui.disasm.show_targets {
            all_entries
                .get(abs_cursor)
                .and_then(|e| match &e.jump_target {
                    Some(JumpTarget::Known(a)) => Some(*a),
                    _ => None,
                })
        } else {
            None
        };

        let target_abs_idx =
            cursor_target_addr.and_then(|addr| all_entries.iter().position(|e| e.addr == addr));

        let target_addr = all_entries
            .get(abs_cursor)
            .map(|e| e.addr)
            .unwrap_or(center_addr);

        let mut title = if self.breakpoints.is_empty() {
            "Disassembly".to_string()
        } else {
            format!("Disassembly ({} bp)", self.breakpoints.len())
        };

        if self.ui.disasm.tab == DisasmTab::Source
            && let Some((path, _)) = self.map_addr_to_source(target_addr, Some(&all_entries))
        {
            let short_path: &str = path.rsplit(['/', '\\']).next().unwrap_or(&path);
            title = format!("{} [{}]", title, short_path);
        }

        if let Some(content_area) = self.render_tabbed_panel(
            frame,
            area,
            Panel::Disassembly,
            &title,
            &["Assembly", "Source"],
            match self.ui.disasm.tab {
                DisasmTab::Assembly => 0,
                DisasmTab::Source => 1,
            },
            focused,
        ) {
            let (inner, _) = if self.ui.input_mode == InputMode::GotoAddress {
                let splits = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(3), Constraint::Length(1)])
                    .split(content_area);

                let mut spans = vec![Span::styled(
                    " Goto Address: 0x",
                    Style::default().fg(self.theme.accent),
                )];
                spans.extend(
                    self.render_input_spans(
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                        self.theme.accent,
                    ),
                );
                spans.push(Span::styled(
                    " (Enter=go, Esc=cancel)",
                    Style::default().fg(self.theme.dim),
                ));
                let input_line = Line::from(spans);
                frame.render_widget(
                    Paragraph::new(input_line).style(Style::default().bg(Color::Rgb(40, 40, 60))),
                    splits[1],
                );
                (splits[0], true)
            } else {
                (content_area, false)
            };

            if self.ui.disasm.tab == DisasmTab::Source {
                self.render_source_view(frame, inner, target_addr, &all_entries, hw_pc);
            } else {
                self.render_assembly_view(
                    frame,
                    inner,
                    &all_entries,
                    focused,
                    abs_cursor,
                    target_abs_idx,
                );
            }
        }
    }

    fn render_source_view(
        &mut self,
        frame: &mut Frame,
        content_area: Rect,
        target_addr: u64,
        all_entries: &[DisasmEntry],
        hw_pc: u64,
    ) {
        let visible_height = content_area.height as usize;
        let source_loc = self.map_addr_to_source(target_addr, Some(all_entries));

        if let Some((path, target_line_from_loc)) = source_loc {
            if Some(target_addr) != self.ui.disasm.last_target_addr {
                self.ui.disasm.source_cursor = target_line_from_loc.saturating_sub(1) as usize;
                self.ui.disasm.last_target_addr = Some(target_addr);
            }

            let lines_len = self.get_source_file(&path).map(|l| l.len()).unwrap_or(0);
            if lines_len > 0 {
                self.ui.disasm.source_cursor = self
                    .ui
                    .disasm
                    .source_cursor
                    .min(lines_len.saturating_sub(1));
                let target_line = self.ui.disasm.source_cursor + 1;

                let half = visible_height / 2;
                self.ui.disasm.source_scroll = self.ui.disasm.source_cursor.saturating_sub(half);

                let max_scroll = lines_len.saturating_sub(visible_height);
                self.ui.disasm.source_scroll = self.ui.disasm.source_scroll.min(max_scroll);
                let scroll = self.ui.disasm.source_scroll;

                let mut source_lines = Vec::new();
                let theme_dim = self.theme.dim;
                let theme_accent = self.theme.accent;
                let theme_breakpoint = self.theme.breakpoint;
                let mapped_addr = self.map_source_to_addr(&path, target_line as u32, hw_pc);
                let hw_pc_line = self.get_hw_pc_line(&path, hw_pc).map(|l| l as usize);

                let mut bp_lines = std::collections::HashSet::new();
                for &bp in &self.breakpoints {
                    if let Some((p, l)) = self.source_locations.get(&bp)
                        && p == &path
                    {
                        bp_lines.insert(*l as usize);
                    }
                }

                let highlight_color = self.theme.highlight;
                let query = self.ui.search.query.clone();
                let compiled_regex = self.ui.search.compiled_regex.clone();

                let lines = self.get_source_file(&path).unwrap();

                for (i, line_tokens) in lines.iter().enumerate().skip(scroll).take(visible_height) {
                    let is_target = i + 1 == target_line;
                    let is_pc = Some(i + 1) == hw_pc_line;
                    let has_bp = bp_lines.contains(&(i + 1));
                    let line_num = format!("{:4} | ", i + 1);

                    let mut spans = vec![Span::styled(line_num, Style::default().fg(theme_dim))];

                    let marker = if is_pc {
                        Span::styled(
                            "► ",
                            Style::default()
                                .fg(theme_accent)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else if has_bp {
                        Span::styled(
                            "● ",
                            Style::default()
                                .fg(theme_breakpoint)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else {
                        Span::raw(" ")
                    };
                    spans.push(marker);

                    if is_target && let Some(addr) = mapped_addr {
                        spans.push(Span::styled(
                            format!("[{:#010x}] ", addr),
                            Style::default().fg(theme_dim),
                        ));
                    }

                    let full_line: String = line_tokens.iter().map(|(t, _)| t.as_str()).collect();
                    let mut match_ranges = Vec::new();
                    if !query.is_empty() {
                        if let Some(re) = &compiled_regex {
                            for mat in re.find_iter(&full_line) {
                                match_ranges.push((mat.start(), mat.end()));
                            }
                        } else {
                            let mut start = 0;
                            let lower_line = full_line.to_lowercase();
                            let lower_query = query.to_lowercase();
                            while let Some(idx) = lower_line[start..].find(&lower_query) {
                                let actual_idx = start + idx;
                                match_ranges.push((actual_idx, actual_idx + lower_query.len()));
                                start = actual_idx + lower_query.len();
                            }
                        }
                    }

                    let mut byte_offset = 0;
                    for (text, token_style) in line_tokens {
                        let mut style = *token_style;
                        if is_target && is_pc {
                            style = style
                                .add_modifier(Modifier::BOLD)
                                .add_modifier(Modifier::REVERSED);
                        } else if is_target {
                            style = style.add_modifier(Modifier::REVERSED);
                        } else if is_pc {
                            style = style.add_modifier(Modifier::BOLD);
                        }

                        let token_start = byte_offset;
                        let token_end = byte_offset + text.len();
                        byte_offset = token_end;

                        if match_ranges.is_empty() {
                            spans.push(Span::styled(text.clone(), style));
                            continue;
                        }

                        let mut last = 0;
                        for &(m_start, m_end) in &match_ranges {
                            if m_end <= token_start || m_start >= token_end {
                                continue;
                            }
                            let overlap_start = m_start.max(token_start) - token_start;
                            let overlap_end = m_end.min(token_end) - token_start;

                            if overlap_start > last {
                                spans.push(Span::styled(
                                    text[last..overlap_start].to_string(),
                                    style,
                                ));
                            }
                            spans.push(Span::styled(
                                text[overlap_start..overlap_end].to_string(),
                                style.fg(Color::Black).bg(highlight_color),
                            ));
                            last = overlap_end;
                        }
                        if last < text.len() {
                            spans.push(Span::styled(text[last..].to_string(), style));
                        }
                    }

                    source_lines.push(Line::from(spans));
                }

                let paragraph = Paragraph::new(source_lines);
                frame.render_widget(paragraph, content_area);
            } else {
                let text = format!("Could not load source file: {}", path);
                frame.render_widget(
                    Paragraph::new(text).style(Style::default().fg(self.theme.error)),
                    content_area,
                );
            }
        } else {
            let text = "No source information available for current address.";
            frame.render_widget(
                Paragraph::new(text).style(Style::default().fg(self.theme.dim)),
                content_area,
            );
        }
    }

    fn render_assembly_view(
        &mut self,
        frame: &mut Frame,
        content_area: Rect,
        all_entries: &[DisasmEntry],
        focused: bool,
        abs_cursor: usize,
        target_abs_idx: Option<usize>,
    ) {
        let x_regs = {
            if let Some(machine) = self.machine.as_ref() {
                *machine.harts()[self.ui.selected_hart].registers().x()
            } else {
                [0; 32]
            }
        };

        let mut active_jump = None;
        let mut active_unknown = None;
        let mut active_offscreen = None;
        for (i, e) in all_entries.iter().enumerate() {
            if focused && abs_cursor == i {
                match &e.jump_target {
                    Some(JumpTarget::Known(addr)) => {
                        if let Some(dst_idx) = all_entries.iter().position(|t| t.addr == *addr) {
                            active_jump = Some((i, dst_idx));
                        } else {
                            active_offscreen = Some(i);
                        }
                    }
                    Some(JumpTarget::Unknown) => {
                        active_unknown = Some(i);
                    }
                    None => {}
                }
            } else if focused
                && let Some(JumpTarget::Known(addr)) = &e.jump_target
                && let Some(dst_idx) = all_entries.iter().position(|t| t.addr == *addr)
                && abs_cursor == dst_idx
            {
                active_jump = Some((i, dst_idx));
            }
        }

        let mut all_lines: Vec<Line> = Vec::new();
        let mut cursor_line_idx = 0;

        for (i, e) in all_entries.iter().enumerate() {
            let is_cursor = focused && i == abs_cursor;
            let is_target_line = target_abs_idx == Some(i);

            if let Some(sym) = &e.symbol {
                all_lines.push(Line::from(Span::styled(
                    format!("<{}>:", sym),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )));
            }

            if i == abs_cursor {
                cursor_line_idx = all_lines.len();
            }

            let marker = if e.is_pc {
                Span::styled(
                    "►",
                    Style::default()
                        .fg(self.theme.accent)
                        .add_modifier(Modifier::BOLD),
                )
            } else if e.is_bp {
                Span::styled(
                    "●",
                    Style::default()
                        .fg(self.theme.breakpoint)
                        .add_modifier(Modifier::BOLD),
                )
            } else if is_target_line {
                Span::styled(
                    "◄",
                    Style::default()
                        .fg(self.theme.target)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw(" ")
            };

            let text_color = if e.is_pc {
                self.theme.accent
            } else if is_target_line && self.ui.disasm.show_targets {
                self.theme.target
            } else {
                self.theme.instruction_color(&e.text)
            };

            let base_style = if e.is_pc {
                Style::default().fg(text_color).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(text_color)
            };

            let addr_style = if e.is_pc && e.is_bp {
                Style::default()
                    .fg(self.theme.breakpoint)
                    .add_modifier(Modifier::BOLD)
            } else if e.is_pc {
                base_style
            } else {
                Style::default().fg(self.theme.dim)
            };

            let bg = if is_cursor {
                self.theme.cursor_bg
            } else {
                Color::Reset
            };

            let compressed_marker = if e.is_compressed {
                Span::styled(" [C] ", Style::default().fg(self.theme.dim).bg(bg))
            } else {
                Span::styled(" ", Style::default().bg(bg))
            };

            let arrow_prefix = if let Some((src, dst)) = active_jump {
                let start = src.min(dst);
                let end = src.max(dst);

                if i == src && i == dst {
                    "⟲ "
                } else if i == src {
                    if dst > src { "╭─" } else { "╰─" }
                } else if i == dst {
                    if src < dst { "╰>" } else { "╭>" }
                } else if i > start && i < end {
                    "│ "
                } else {
                    " "
                }
            } else if active_offscreen == Some(i) {
                "─>"
            } else if active_unknown == Some(i) {
                "─?"
            } else {
                " "
            };

            let arrow_color = if active_jump.is_some()
                || active_unknown.is_some()
                || active_offscreen.is_some()
            {
                self.theme.target
            } else {
                Color::Reset
            };
            let arrow_span = Span::styled(arrow_prefix, Style::default().fg(arrow_color).bg(bg));

            let mut spans = vec![
                marker,
                Span::styled(" ", Style::default().bg(bg)),
                arrow_span,
                Span::styled(format!(" {:#010x} ", e.addr), addr_style.bg(bg)),
                compressed_marker,
            ];

            spans.push(Span::styled(e.text.clone(), base_style.bg(bg)));

            match &e.jump_target {
                Some(JumpTarget::Known(addr)) => {
                    let sym_name = self
                        .sorted_symbols
                        .iter()
                        .find(|(a, _)| a == addr)
                        .map(|(_, n)| n.as_str());
                    let target_str = if let Some(sym) = sym_name {
                        format!(" → {:#x} <{}>", addr, sym)
                    } else {
                        format!(" → {:#x}", addr)
                    };
                    let target_color = if is_cursor {
                        self.theme.target
                    } else {
                        self.theme.dim
                    };
                    spans.push(Span::styled(
                        target_str,
                        Style::default().fg(target_color).bg(bg),
                    ));
                }
                Some(JumpTarget::Unknown) => {
                    let target_color = if is_cursor {
                        self.theme.target
                    } else {
                        self.theme.dim
                    };
                    spans.push(Span::styled(
                        " → ???",
                        Style::default().fg(target_color).bg(bg),
                    ));
                }
                None => {}
            }

            if e.is_pc {
                let extracted = parse_registers(&e.text, &x_regs);
                if !extracted.is_empty() {
                    let mut reg_str = String::from(" // ");
                    for (idx, (name, val)) in extracted.iter().enumerate() {
                        reg_str.push_str(&format!("{}={:#x} ({})", name, val, *val as i64));
                        if idx < extracted.len() - 1 {
                            reg_str.push_str(", ");
                        }
                    }
                    spans.push(Span::styled(
                        reg_str,
                        Style::default().fg(self.theme.dim).bg(bg),
                    ));
                }
            }

            if let Some(loc) = self.source_lines.get(&e.addr) {
                spans.push(Span::styled(
                    format!(" @ {}", loc),
                    Style::default().fg(Color::Rgb(130, 130, 180)).bg(bg),
                ));
            }

            all_lines.push(Line::from(spans));
        }

        let visible_height = content_area.height as usize;
        let half = visible_height / 2;
        let view_start = if cursor_line_idx < half {
            0
        } else if cursor_line_idx + half >= all_lines.len() {
            all_lines.len().saturating_sub(visible_height)
        } else {
            cursor_line_idx.saturating_sub(half)
        };
        let view_end = (view_start + visible_height).min(all_lines.len());
        let visible_lines = all_lines[view_start..view_end].to_vec();

        let paragraph = Paragraph::new(visible_lines);
        frame.render_widget(paragraph, content_area);
    }

    fn render_memory(&mut self, frame: &mut Frame, area: Rect) {
        let focused = self.ui.panel == Panel::Memory;
        let active_tab = match self.ui.memory_tab {
            crate::ui_state::MemoryTab::Hex => 0,
            crate::ui_state::MemoryTab::Stack => 1,
        };

        if let Some(content_area) = self.render_tabbed_panel(
            frame,
            area,
            Panel::Memory,
            "Memory",
            &["Hex", "Stack"],
            active_tab,
            focused,
        ) {
            match self.ui.memory_tab {
                crate::ui_state::MemoryTab::Hex => self.render_memory_hex(frame, content_area),
                crate::ui_state::MemoryTab::Stack => self.render_memory_stack(frame, content_area),
            }
        }
    }

    fn render_memory_hex(&mut self, frame: &mut Frame, area: Rect) {
        let (inner, _) = if self.ui.input_mode == InputMode::GotoMemory {
            let splits = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(1)])
                .split(area);

            let mut spans = vec![Span::styled(
                " Goto: 0x",
                Style::default().fg(self.theme.accent),
            )];
            spans.extend(
                self.render_input_spans(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                    self.theme.accent,
                ),
            );
            spans.push(Span::styled(
                " (Enter=go, Esc=cancel)",
                Style::default().fg(self.theme.dim),
            ));
            let input_line = Line::from(spans);
            frame.render_widget(
                Paragraph::new(input_line).style(Style::default().bg(Color::Rgb(40, 40, 60))),
                splits[1],
            );
            (splits[0], true)
        } else {
            (area, false)
        };

        let rows_available = inner.height.saturating_sub(2) as usize;
        let bytes_per_row = 16usize;
        let data = self.read_memory_block(self.ui.memory_addr, rows_available * bytes_per_row);

        let lines: Vec<Line> = data
            .chunks(bytes_per_row)
            .enumerate()
            .map(|(row_idx, chunk)| {
                let addr = self.ui.memory_addr + (row_idx * bytes_per_row) as u64;
                let mut spans = vec![Span::styled(
                    format!("{:#010x} ", addr),
                    Style::default().fg(self.theme.highlight),
                )];

                for &byte in chunk.iter() {
                    let color = if byte == 0 {
                        self.theme.dim
                    } else {
                        Color::White
                    };
                    let sep = " ";
                    spans.push(Span::styled(
                        format!("{:02x}{}", byte, sep),
                        Style::default().fg(color),
                    ));
                }

                spans.push(Span::raw(" │ "));
                let ascii: String = chunk
                    .iter()
                    .map(|&b| {
                        if b.is_ascii_graphic() || b == b' ' {
                            b as char
                        } else {
                            '·'
                        }
                    })
                    .collect();
                spans.push(Span::styled(ascii, Style::default().fg(self.theme.dim)));
                Line::from(spans)
            })
            .collect();

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner);
    }

    fn render_memory_stack(&mut self, frame: &mut Frame, area: Rect) {
        let hart_idx = self.ui.selected_hart;
        if hart_idx >= self.stack_analyzers.len() {
            return;
        }

        let analyzer = &self.stack_analyzers[hart_idx];
        let mut lines = Vec::new();

        if let Some(frame_info) = &analyzer.current_frame {
            lines.push(Line::from(vec![
                Span::styled("Stack Allocation: ", Style::default().fg(self.theme.dim)),
                Span::styled(
                    format!("{} bytes", frame_info.size),
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            let sp = self
                .machine
                .as_ref()
                .map(|m| {
                    m.harts()[hart_idx].registers().x()
                        [emulator::registers::GeneralRegisterName::Sp as usize]
                })
                .unwrap_or(0);

            for push in &frame_info.pushes {
                let is_ra = push.reg == emulator::registers::GeneralRegisterName::Ra;
                let color = if is_ra {
                    Color::LightRed
                } else {
                    self.theme.accent
                };
                let reg_name = if is_ra {
                    "ra (ret)".to_string()
                } else {
                    format!("{:?}", push.reg).to_lowercase()
                };

                let addr = sp.wrapping_add(push.offset as u64);
                let val_bytes = self.read_memory_block(addr, push.data_type.size_bytes() as usize);
                let val = match push.data_type {
                    crate::state::DataType::U8 | crate::state::DataType::I8 => {
                        if !val_bytes.is_empty() {
                            format!("{:#04x} ({})", val_bytes[0], val_bytes[0] as i8)
                        } else {
                            "?".into()
                        }
                    }
                    crate::state::DataType::U16 | crate::state::DataType::I16 => {
                        if val_bytes.len() >= 2 {
                            let num = u16::from_le_bytes([val_bytes[0], val_bytes[1]]);
                            format!("{:#06x} ({})", num, num as i16)
                        } else {
                            "?".into()
                        }
                    }
                    crate::state::DataType::U32 | crate::state::DataType::I32 => {
                        if val_bytes.len() >= 4 {
                            let num = u32::from_le_bytes([
                                val_bytes[0],
                                val_bytes[1],
                                val_bytes[2],
                                val_bytes[3],
                            ]);
                            format!("{:#010x} ({})", num, num as i32)
                        } else {
                            "?".into()
                        }
                    }
                    crate::state::DataType::U64 | crate::state::DataType::I64 => {
                        if val_bytes.len() >= 8 {
                            let num = u64::from_le_bytes([
                                val_bytes[0],
                                val_bytes[1],
                                val_bytes[2],
                                val_bytes[3],
                                val_bytes[4],
                                val_bytes[5],
                                val_bytes[6],
                                val_bytes[7],
                            ]);
                            format!("{:#018x} ({})", num, num as i64)
                        } else {
                            "?".into()
                        }
                    }
                };

                lines.push(Line::from(vec![
                    Span::styled(
                        format!("[sp + {:>3}] ", push.offset),
                        Style::default().fg(self.theme.dim),
                    ),
                    Span::styled(
                        format!("{:>4} ", push.data_type),
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::styled(
                        format!("{:<10} ", reg_name),
                        Style::default().fg(color).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(format!("= {}", val), Style::default().fg(Color::White)),
                ]));
            }
        } else {
            lines.push(Line::from(Span::styled(
                "No stack frame detected.",
                Style::default().fg(self.theme.dim),
            )));
        }

        let paragraph = Paragraph::new(lines).scroll((self.ui.stack_scroll as u16, 0));
        frame.render_widget(paragraph, area);
    }

    fn render_symbols(&mut self, frame: &mut Frame, area: Rect) {
        let focused = self.ui.panel == Panel::Symbols;
        let active_tab = match self.ui.symbols.tab {
            SymbolsTab::Trace => 0,
            SymbolsTab::Symbols => 1,
        };

        if let Some(content_area) = self.render_tabbed_panel(
            frame,
            area,
            Panel::Symbols,
            "Symbols / Trace",
            &["Trace", "Symbols"],
            active_tab,
            focused,
        ) {
            match self.ui.symbols.tab {
                SymbolsTab::Trace => self.render_trace(frame, content_area),
                SymbolsTab::Symbols => self.render_symbols_list(frame, content_area),
            }
        }
    }

    fn render_trace(&mut self, frame: &mut Frame, content_area: Rect) {
        let filtered_trace: Vec<(usize, u64)> = if self.ui.trace.hide_non_symbols {
            self.ui.trace.stack.iter().rev().enumerate().filter(|&(_, &addr)| {
                self.sorted_symbols.binary_search_by_key(&addr, |(a, _)| *a).is_ok()
            }).map(|(i, &addr)| (i, addr)).collect()
        } else {
            self.ui.trace.stack.iter().rev().enumerate().map(|(i, &addr)| (i, addr)).collect()
        };

        let trace_len = filtered_trace.len();
        self.ui.trace.cursor = self.ui.trace.cursor.min(trace_len.saturating_sub(1));
        let focused = self.ui.panel == Panel::Symbols;

        let visible_height = content_area.height as usize;

        if self.ui.trace.cursor < self.ui.trace.scroll {
            self.ui.trace.scroll = self.ui.trace.cursor;
        } else if self.ui.trace.cursor >= self.ui.trace.scroll + visible_height {
            self.ui.trace.scroll = self
                .ui
                .trace
                .cursor
                .saturating_sub(visible_height.saturating_sub(1));
        }

        let max_scroll = trace_len.saturating_sub(visible_height);
        let scroll = self.ui.trace.scroll.min(max_scroll);

        let lines: Vec<Line> = filtered_trace
            .into_iter()
            .skip(scroll)
            .take(visible_height)
            .enumerate()
            .map(|(ui_idx, (_real_idx, addr))| {
                let mut spans = vec![];
                let selected = focused && scroll + ui_idx == self.ui.trace.cursor;

                if selected {
                    spans.push(Span::styled(
                        " ► ",
                        Style::default()
                            .fg(self.theme.accent)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    spans.push(Span::raw(" "));
                }

                spans.push(Span::styled(
                    format!("{:#010x} ", addr),
                    Style::default().fg(if selected {
                        self.theme.accent
                    } else {
                        self.theme.highlight
                    }),
                ));

                if let Some(inst) = self.disassemble_instruction_at(addr) {
                    spans.push(Span::styled(
                        format!("{{{}}} ", inst),
                        Style::default().fg(self.theme.dim),
                    ));
                }

                let sym_name = self
                    .sorted_symbols
                    .iter()
                    .find(|(a, _)| a == &addr)
                    .map(|(_, n)| n.as_str());
                if let Some(sym) = sym_name {
                    spans.push(Span::styled(sym, Style::default().fg(Color::Cyan)));
                } else if let Some((path, line)) = self.source_locations.get(&addr) {
                    let short: &str = path.rsplit(['/', '\\']).next().unwrap_or(path);
                    spans.push(Span::styled(
                        format!("{}:{}", short, line),
                        Style::default().fg(self.theme.dim),
                    ));
                }

                Line::from(spans)
            })
            .collect();

        if lines.is_empty() {
            frame.render_widget(
                Paragraph::new(" No trace available").style(Style::default().fg(self.theme.dim)),
                content_area,
            );
        } else {
            frame.render_widget(Paragraph::new(lines), content_area);
        }
    }

    fn render_symbols_list(&mut self, frame: &mut Frame, inner: Rect) {
        let focused = self.ui.panel == Panel::Symbols;
        let query = &self.ui.search.query;
        let is_match = |text: &str| -> bool {
            if query.is_empty() {
                return true;
            }
            if let Some(re) = &self.ui.search.compiled_regex {
                re.is_match(text)
            } else {
                text.to_lowercase().contains(&query.to_lowercase())
            }
        };

        let filtered: Vec<_> = self
            .sorted_symbols
            .iter()
            .filter(|(_, name)| is_match(name))
            .collect();

        self.ui.symbols.cursor = self.ui.symbols.cursor.min(filtered.len().saturating_sub(1));

        let visible_height = inner.height.saturating_sub(2) as usize;

        if self.ui.symbols.cursor < self.ui.symbols.scroll {
            self.ui.symbols.scroll = self.ui.symbols.cursor;
        } else if self.ui.symbols.cursor >= self.ui.symbols.scroll + visible_height {
            self.ui.symbols.scroll = self
                .ui
                .symbols
                .cursor
                .saturating_sub(visible_height.saturating_sub(1));
        }

        let max_scroll = filtered.len().saturating_sub(visible_height);
        let scroll = self.ui.symbols.scroll.min(max_scroll);

        let lines: Vec<Line> = filtered
            .into_iter()
            .enumerate()
            .skip(scroll)
            .take(visible_height)
            .map(|(i, (addr, name))| {
                let mut spans = vec![];
                let selected = focused && i == self.ui.symbols.cursor;

                if selected {
                    spans.push(Span::styled(
                        " ► ",
                        Style::default()
                            .fg(self.theme.accent)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    spans.push(Span::raw(" "));
                }

                spans.push(Span::styled(
                    format!("{:#010x} ", addr),
                    Style::default().fg(if selected {
                        self.theme.accent
                    } else {
                        self.theme.highlight
                    }),
                ));
                if !query.is_empty() {
                    let base_fg = if selected { Color::White } else { Color::Cyan };
                    if let Some(re) = &self.ui.search.compiled_regex {
                        if let Some(mat) = re.find(name) {
                            spans.push(Span::styled(
                                &name[..mat.start()],
                                Style::default().fg(base_fg),
                            ));
                            spans.push(Span::styled(
                                &name[mat.start()..mat.end()],
                                Style::default().fg(Color::Black).bg(self.theme.highlight),
                            ));
                            spans.push(Span::styled(
                                &name[mat.end()..],
                                Style::default().fg(base_fg),
                            ));
                        } else {
                            spans.push(Span::styled(name.clone(), Style::default().fg(base_fg)));
                        }
                    } else if let Some(idx) = name.to_lowercase().find(&query.to_lowercase()) {
                        spans.push(Span::styled(&name[..idx], Style::default().fg(base_fg)));
                        spans.push(Span::styled(
                            &name[idx..idx + query.len()],
                            Style::default().fg(Color::Black).bg(self.theme.highlight),
                        ));
                        spans.push(Span::styled(
                            &name[idx + query.len()..],
                            Style::default().fg(base_fg),
                        ));
                    } else {
                        spans.push(Span::styled(name.clone(), Style::default().fg(base_fg)));
                    }
                } else {
                    spans.push(Span::styled(
                        name.clone(),
                        Style::default().fg(if selected { Color::White } else { Color::Cyan }),
                    ));
                }
                Line::from(spans)
            })
            .collect();

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner);
    }

    fn render_console(&mut self, frame: &mut Frame, area: Rect) {
        let focused = self.ui.panel == Panel::Console;
        let active_tab = match self.ui.console.tab {
            ConsoleTab::Debugger => 0,
            ConsoleTab::Tracing => 1,
        };

        if let Some(content_area) = self.render_tabbed_panel(
            frame,
            area,
            Panel::Console,
            "Console",
            &["Debugger", "Tracing"],
            active_tab,
            focused,
        ) {
            let visible_height = content_area.height as usize;
            let lines = match self.ui.console.tab {
                ConsoleTab::Debugger => self.format_console_logs(
                    &self.console_log,
                    visible_height,
                    content_area.width as usize,
                ),
                ConsoleTab::Tracing => {
                    if let Ok(logs) = self.tracing_log.lock() {
                        self.format_console_logs(&logs, visible_height, content_area.width as usize)
                    } else {
                        Vec::new()
                    }
                }
            };

            if lines.is_empty() {
                let empty = Paragraph::new(Span::styled(
                    " No messages",
                    Style::default().fg(self.theme.dim),
                ));
                frame.render_widget(empty, content_area);
            } else {
                let paragraph = Paragraph::new(lines);
                frame.render_widget(paragraph, content_area);
            }
        }
    }

    fn format_console_logs(
        &self,
        logs: &[ConsoleEntry],
        visible_height: usize,
        area_width: usize,
    ) -> Vec<Line<'static>> {
        if logs.is_empty() || area_width < 10 {
            return Vec::new();
        }

        let logs_to_process = if logs.len() > 1000 {
            &logs[logs.len() - 1000..]
        } else {
            logs
        };

        let mut wrapped_lines = Vec::new();
        for entry in logs_to_process {
            let (level_str, color) = match entry.level {
                ConsoleLevel::Info => ("INFO", self.theme.info),
                ConsoleLevel::Error => ("ERR ", self.theme.error),
                ConsoleLevel::Warn => ("WARN", self.theme.warn),
            };

            let prefix_str = format!("[{}] {} ", entry.tick, level_str);
            let prefix_len = prefix_str.len();
            let text_width = area_width.saturating_sub(prefix_len + 2);

            if text_width < 10 {
                wrapped_lines.push(Line::from(vec![
                    Span::styled(
                        format!("[{}] ", entry.tick),
                        Style::default().fg(self.theme.dim),
                    ),
                    Span::styled(
                        format!("{} ", level_str),
                        Style::default().fg(color).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(entry.message.clone(), Style::default().fg(Color::White)),
                ]));
                continue;
            }

            let mut current_line = String::new();
            let mut is_first_line = true;
            for word in entry.message.split_whitespace() {
                if current_line.len() + word.len() + 1 > text_width {
                    if !current_line.is_empty() {
                        let mut spans = vec![];
                        if is_first_line {
                            spans.push(Span::styled(
                                format!("[{}] ", entry.tick),
                                Style::default().fg(self.theme.dim),
                            ));
                            spans.push(Span::styled(
                                format!("{} ", level_str),
                                Style::default().fg(color).add_modifier(Modifier::BOLD),
                            ));
                            is_first_line = false;
                        } else {
                            spans.push(Span::raw(" ".repeat(prefix_len)));
                        }
                        spans.push(Span::styled(
                            current_line,
                            Style::default().fg(Color::White),
                        ));
                        wrapped_lines.push(Line::from(spans));
                        current_line = String::new();
                    }
                    current_line.push_str(word);
                } else {
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(word);
                }
            }
            if !current_line.is_empty() || is_first_line {
                let mut spans = vec![];
                if is_first_line {
                    spans.push(Span::styled(
                        format!("[{}] ", entry.tick),
                        Style::default().fg(self.theme.dim),
                    ));
                    spans.push(Span::styled(
                        format!("{} ", level_str),
                        Style::default().fg(color).add_modifier(Modifier::BOLD),
                    ));
                } else {
                    spans.push(Span::raw(" ".repeat(prefix_len)));
                }
                spans.push(Span::styled(
                    current_line,
                    Style::default().fg(Color::White),
                ));
                wrapped_lines.push(Line::from(spans));
            }
        }

        let max_scroll = wrapped_lines.len().saturating_sub(visible_height);
        let scroll_offset = self.ui.console.scroll.min(max_scroll);
        let start_idx = max_scroll.saturating_sub(scroll_offset);

        wrapped_lines
            .into_iter()
            .skip(start_idx)
            .take(visible_height)
            .collect()
    }

    fn render_input_spans(&self, base_style: Style, cursor_color: Color) -> Vec<Span<'static>> {
        let buffer = self.ui.input_buffer();
        let cursor = self.ui.input_cursor;
        let chars: Vec<char> = buffer.chars().collect();
        let mut spans = vec![];
        if cursor < chars.len() {
            let before: String = chars[..cursor].iter().collect();
            let at: String = chars[cursor..cursor + 1].iter().collect();
            let after: String = chars[cursor + 1..].iter().collect();

            if !before.is_empty() {
                spans.push(Span::styled(before, base_style));
            }
            spans.push(Span::styled(
                at,
                Style::default().bg(cursor_color).fg(Color::Black),
            ));
            if !after.is_empty() {
                spans.push(Span::styled(after, base_style));
            }
        } else {
            spans.push(Span::styled(buffer.to_string(), base_style));
            spans.push(Span::styled("_", Style::default().fg(cursor_color)));
        }
        spans
    }

    fn render_bottom_bar(&self, frame: &mut Frame, area: Rect) {
        if self.ui.input_mode == InputMode::Command {
            let mut spans = vec![Span::styled(
                ":",
                Style::default()
                    .fg(self.theme.accent)
                    .add_modifier(Modifier::BOLD),
            )];
            spans.extend(
                self.render_input_spans(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                    self.theme.accent,
                ),
            );
            let line = Line::from(spans);
            let bar = Paragraph::new(line).style(Style::default().bg(Color::Rgb(30, 30, 50)));
            frame.render_widget(bar, area);
            return;
        } else if self.ui.input_mode == InputMode::Search {
            let mut prefix = "Search: ".to_string();
            let mut style = Style::default().fg(self.theme.accent);
            if self.ui.search.is_regex_error {
                prefix = "Search (Regex Error): ".to_string();
                style = Style::default().fg(self.theme.error);
            } else if self.ui.search.compiled_regex.is_some() {
                prefix = "Search (Regex): ".to_string();
            }

            let mut spans = vec![Span::styled(prefix, style.add_modifier(Modifier::BOLD))];
            let cursor_color = style.fg.unwrap_or(self.theme.accent);
            spans.extend(
                self.render_input_spans(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                    cursor_color,
                ),
            );
            let line = Line::from(spans);
            let bar = Paragraph::new(line).style(Style::default().bg(Color::Rgb(30, 30, 50)));
            frame.render_widget(bar, area);
            return;
        }

        let mut spans = vec![
            Span::styled(
                " F11 ",
                Style::default()
                    .fg(self.theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Step "),
            Span::styled(
                " c ",
                Style::default()
                    .fg(self.theme.running)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Run "),
            Span::styled(
                " b ",
                Style::default()
                    .fg(self.theme.breakpoint)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("BP "),
            Span::styled(" t ", Style::default().fg(self.theme.target)),
            Span::raw("Trace "),
            Span::styled(
                " T ",
                Style::default().fg(if self.ui.disasm.show_targets {
                    self.theme.target
                } else {
                    self.theme.dim
                }),
            ),
            Span::raw("Jumps "),
            Span::styled(
                " : ",
                Style::default()
                    .fg(self.theme.highlight)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Cmd "),
            Span::styled(" q ", Style::default().fg(self.theme.error)),
            Span::raw("Quit"),
        ];

        if let Some((ref msg, is_error)) = self.last_message {
            let color = if is_error {
                self.theme.error
            } else {
                self.theme.info
            };
            spans.push(Span::styled(
                format!(" {}", msg),
                Style::default().fg(color),
            ));
        }

        spans.push(Span::styled(
            format!(" ticks: {}", self.tick_count),
            Style::default().fg(self.theme.dim),
        ));

        let bar =
            Paragraph::new(Line::from(spans)).style(Style::default().bg(Color::Rgb(30, 30, 40)));
        frame.render_widget(bar, area);
    }

    fn panel_block(&self, title: &str, focused: bool) -> Block<'_> {
        let border_style = if focused {
            if self.ui.panel_focused {
                Style::default()
                    .fg(self.theme.highlight)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(self.theme.accent)
            }
        } else {
            Style::default().fg(self.theme.border)
        };
        Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(format!(" {} ", title))
            .title_style(Style::default().fg(if focused {
                if self.ui.panel_focused {
                    self.theme.highlight
                } else {
                    self.theme.accent
                }
            } else {
                Color::White
            }))
    }

    fn setup_field_row<'a>(&self, label: &'a str, value: &'a str, selected: bool) -> Line<'a> {
        let marker = if selected { "► " } else { " " };
        let style = if selected {
            Style::default()
                .fg(self.theme.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        Line::from(vec![
            Span::styled(marker, style),
            Span::styled(format!("{:<10}", label), Style::default().fg(Color::White)),
            Span::styled(value.to_string(), style),
        ])
    }
}

fn format_binary_grouped(val: u64) -> String {
    if val == 0 {
        return "0000".into();
    }
    let raw = format!("{:b}", val);
    let padded_len = raw.len().div_ceil(4) * 4;
    let padded = format!("{:0>width$}", raw, width = padded_len);
    padded
        .as_bytes()
        .chunks(4)
        .map(|c| std::str::from_utf8(c).unwrap())
        .collect::<Vec<_>>()
        .join("_")
}

// Parses register names directly from the disassembled string to avoid a massive AST match block.
fn parse_registers(asm: &str, x_regs: &[u64; 32]) -> Vec<(String, u64)> {
    let mut regs = Vec::new();
    let mut seen = Vec::new();

    for t in asm.split([' ', ',', '(', ')']).map(str::trim) {
        if let Some(idx) = parse_register_name(t)
            && idx != 0
            && !seen.contains(&idx)
        {
            seen.push(idx);
            regs.push((t.to_string(), x_regs[idx]));
        }
    }
    regs
}

fn parse_register_name(name: &str) -> Option<usize> {
    match name {
        "zero" | "x0" => Some(0),
        "ra" | "x1" => Some(1),
        "sp" | "x2" => Some(2),
        "gp" | "x3" => Some(3),
        "tp" | "x4" => Some(4),
        "t0" | "x5" => Some(5),
        "t1" | "x6" => Some(6),
        "t2" | "x7" => Some(7),
        "s0" | "fp" | "x8" => Some(8),
        "s1" | "x9" => Some(9),
        "a0" | "x10" => Some(10),
        "a1" | "x11" => Some(11),
        "a2" | "x12" => Some(12),
        "a3" | "x13" => Some(13),
        "a4" | "x14" => Some(14),
        "a5" | "x15" => Some(15),
        "a6" | "x16" => Some(16),
        "a7" | "x17" => Some(17),
        "s2" | "x18" => Some(18),
        "s3" | "x19" => Some(19),
        "s4" | "x20" => Some(20),
        "s5" | "x21" => Some(21),
        "s6" | "x22" => Some(22),
        "s7" | "x23" => Some(23),
        "s8" | "x24" => Some(24),
        "s9" | "x25" => Some(25),
        "s10" | "x26" => Some(26),
        "s11" | "x27" => Some(27),
        "t3" | "x28" => Some(28),
        "t4" | "x29" => Some(29),
        "t5" | "x30" => Some(30),
        "t6" | "x31" => Some(31),
        _ => None,
    }
}

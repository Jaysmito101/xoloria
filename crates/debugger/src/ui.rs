use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Tabs, Wrap},
};

use emulator::registers::{ControlRegisterName, GeneralRegisterName};

use crate::app::{Debugger, JumpTarget};
use crate::state::*;
use crate::ui_state::DisasmTab;

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

        let left = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(mid[0]);

        let lower_mid = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[2]);

        self.render_registers(frame, left[0]);
        self.render_csr(frame, left[1]);
        self.render_disassembly(frame, mid[1]);
        self.render_memory(frame, lower_mid[0]);
        self.render_symbols(frame, lower_mid[1]);
        self.render_console(frame, layout[3]);
        self.render_bottom_bar(frame, layout[4]);

        if self.ui.show_help {
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
            " n / Space : Step Instruction",
            " x : Stalled mode",
            "",
            "=== Disassembly ===",
            " b : Toggle breakpoint at cursor",
            " Shift+D : Clear all breakpoints",
            " Ctrl+S : Save breakpoints",
            " Ctrl+L : Load breakpoints",
            " g / Enter : Follow jump / Jump to selected symbol",
            " u / Backspace : Go back in history",
            " t : Toggle jump target labels",
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
            " / : Search symbols",
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
            .skip(self.ui.help_scroll)
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

    fn render_csr(&mut self, frame: &mut Frame, area: Rect) {
        self.ui.panel_rects.insert(Panel::Csr, area);
        let focused = self.ui.panel == Panel::Csr;
        let Some(machine) = self.machine.as_ref() else {
            let block = self.panel_block("CSR", focused);
            frame.render_widget(block, area);
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

        let title = if scroll > 0 {
            format!("CSR [{}/{}]", scroll, max_scroll)
        } else {
            "CSR".into()
        };

        let table = Table::new(
            visible_rows,
            [
                Constraint::Length(name_col as u16),
                Constraint::Length(hex_col as u16),
                Constraint::Min(12),
            ],
        )
        .block(self.panel_block(&title, focused))
        .header(
            Row::new(vec!["Register", "Hex", "Binary"]).style(
                Style::default()
                    .fg(self.theme.dim)
                    .add_modifier(Modifier::BOLD),
            ),
        );
        frame.render_widget(table, area);
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
        let center_addr = self.ui.view_center_addr.unwrap_or(hw_pc);

        let center_idx = all_entries
            .iter()
            .position(|e| e.addr == center_addr)
            .unwrap_or(0) as i32;
        let abs_cursor = (center_idx + self.ui.disasm_cursor).max(0) as usize;
        let abs_cursor = abs_cursor.min(all_entries.len().saturating_sub(1));

        let cursor_target_addr = if self.ui.show_targets {
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

        let titles = vec![Line::from(vec![
            Span::styled(
                if self.ui.disasm_tab == DisasmTab::Assembly {
                    " [Assembly] "
                } else {
                    " Assembly "
                },
                Style::default().fg(if self.ui.disasm_tab == DisasmTab::Assembly {
                    self.theme.accent
                } else {
                    self.theme.dim
                }),
            ),
            Span::styled(
                if self.ui.disasm_tab == DisasmTab::Source {
                    " [Source] "
                } else {
                    " Source "
                },
                Style::default().fg(if self.ui.disasm_tab == DisasmTab::Source {
                    self.theme.accent
                } else {
                    self.theme.dim
                }),
            ),
        ])];

        let title = if self.breakpoints.is_empty() {
            "Disassembly (s: toggle tab)".into()
        } else {
            format!(
                "Disassembly ({} bp) (s: toggle tab)",
                self.breakpoints.len()
            )
        };

        let block = self.panel_block(&title, focused);
        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)])
            .split(inner_area);

        let tabs = Tabs::new(titles);
        frame.render_widget(tabs, layout[0]);
        let content_area = layout[1];

        let visible_height = content_area.height as usize;

        if self.ui.disasm_tab == DisasmTab::Source {
            let target_addr = all_entries
                .get(abs_cursor)
                .map(|e| e.addr)
                .unwrap_or(center_addr);
            let source_loc = self.map_addr_to_source(target_addr, Some(&all_entries));

            if let Some((path, _)) = source_loc {
                let lines_len = self.get_source_file(&path).map(|l| l.len()).unwrap_or(0);
                if lines_len > 0 {
                    self.ui.source_cursor = self.ui.source_cursor.min(lines_len.saturating_sub(1));
                    let target_line = self.ui.source_cursor + 1;

                    let half = visible_height / 2;
                    self.ui.source_scroll = self.ui.source_cursor.saturating_sub(half);

                    let max_scroll = lines_len.saturating_sub(visible_height);
                    self.ui.source_scroll = self.ui.source_scroll.min(max_scroll);
                    let scroll = self.ui.source_scroll;

                    let mut source_lines = Vec::new();
                    let theme_dim = self.theme.dim;
                    let theme_accent = self.theme.accent;
                    let mapped_addr = self.map_source_to_addr(&path, target_line as u32, hw_pc);
                    let hw_pc_line = self.get_hw_pc_line(&path, hw_pc).map(|l| l as usize);

                    let lines = self.get_source_file(&path).unwrap();

                    for (i, line) in lines.iter().enumerate().skip(scroll).take(visible_height) {
                        let is_target = i + 1 == target_line;
                        let is_pc = Some(i + 1) == hw_pc_line;
                        let line_num = format!("{:4} | ", i + 1);

                        let mut spans =
                            vec![Span::styled(line_num, Style::default().fg(theme_dim))];

                        let marker = if is_pc {
                            Span::styled(
                                "► ",
                                Style::default()
                                    .fg(theme_accent)
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

                        let text_style = if is_target && is_pc {
                            Style::default()
                                .fg(theme_accent)
                                .add_modifier(Modifier::BOLD)
                                .add_modifier(Modifier::REVERSED)
                        } else if is_target {
                            Style::default().add_modifier(Modifier::REVERSED)
                        } else if is_pc {
                            Style::default()
                                .fg(theme_accent)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default()
                        };

                        spans.push(Span::styled(line, text_style));

                        source_lines.push(Line::from(spans));
                    }

                    let paragraph = Paragraph::new(source_lines);
                    frame.render_widget(paragraph, content_area);
                    return;
                } else {
                    let text = format!("Could not load source file: {}", path);
                    frame.render_widget(
                        Paragraph::new(text).style(Style::default().fg(self.theme.error)),
                        content_area,
                    );
                    return;
                }
            } else {
                let text = "No source information available for current address.";
                frame.render_widget(
                    Paragraph::new(text).style(Style::default().fg(self.theme.dim)),
                    content_area,
                );
                return;
            }
        }

        let x_regs = {
            if let Some(machine) = self.machine.as_ref() {
                *machine.harts()[self.ui.selected_hart].registers().x()
            } else {
                [0; 32]
            }
        };

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
            } else if is_target_line && self.ui.show_targets {
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
                Span::styled("     ", Style::default().bg(bg))
            };

            let mut spans = vec![
                marker,
                Span::styled(format!(" {:#010x} ", e.addr), addr_style.bg(bg)),
                compressed_marker,
            ];

            spans.push(Span::styled(e.text.clone(), base_style.bg(bg)));

            if self.ui.show_targets {
                match &e.jump_target {
                    Some(JumpTarget::Known(addr)) => {
                        let sym_name = self.sorted_symbols.iter().find(|(a, _)| a == addr).map(|(_, n)| n.as_str());
                        let target_str = if let Some(sym) = sym_name {
                            format!(" → {:#x} <{}>", addr, sym)
                        } else {
                            format!(" → {:#x}", addr)
                        };
                        spans.push(Span::styled(
                            target_str,
                            Style::default().fg(self.theme.target).bg(bg),
                        ));
                    }
                    Some(JumpTarget::Unknown) => {
                        spans.push(Span::styled(
                            " → ???",
                            Style::default().fg(self.theme.dim).bg(bg),
                        ));
                    }
                    None => {}
                }
            }

            if e.is_pc {
                let extracted = extract_register_values_from_asm(&e.text, &x_regs);
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
        self.ui.panel_rects.insert(Panel::Memory, area);
        let focused = self.ui.panel == Panel::Memory;

        let (inner, _) = if self.ui.input_mode == InputMode::GotoMemory {
            let splits = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(1)])
                .split(area);

            let input_line = Line::from(vec![
                Span::styled(" Goto: 0x", Style::default().fg(self.theme.accent)),
                Span::styled(
                    self.ui.input_buffer(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("_", Style::default().fg(self.theme.accent)),
                Span::styled(
                    " (Enter=go, Esc=cancel)",
                    Style::default().fg(self.theme.dim),
                ),
            ]);
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

        let paragraph = Paragraph::new(lines).block(self.panel_block("Memory", focused));
        frame.render_widget(paragraph, inner);
    }

    fn render_symbols(&mut self, frame: &mut Frame, area: Rect) {
        self.ui.panel_rects.insert(Panel::Symbols, area);
        let focused = self.ui.panel == Panel::Symbols;

        let (inner, _) = if self.ui.input_mode == InputMode::SearchSymbols {
            let splits = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(1)])
                .split(area);

            let input_line = Line::from(vec![
                Span::styled(" Search: ", Style::default().fg(self.theme.accent)),
                Span::styled(
                    self.ui.input_buffer(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("_", Style::default().fg(self.theme.accent)),
                Span::styled(
                    " (Enter=apply, Esc=cancel)",
                    Style::default().fg(self.theme.dim),
                ),
            ]);
            frame.render_widget(
                Paragraph::new(input_line).style(Style::default().bg(Color::Rgb(40, 40, 60))),
                splits[1],
            );
            (splits[0], true)
        } else {
            (area, false)
        };

        let search = self.ui.symbols_search.to_lowercase();
        let filtered: Vec<_> = self
            .sorted_symbols
            .iter()
            .filter(|(_, name)| search.is_empty() || name.to_lowercase().contains(&search))
            .collect();

        self.ui.symbols_cursor = self.ui.symbols_cursor.min(filtered.len().saturating_sub(1));

        let visible_height = inner.height.saturating_sub(2) as usize;

        if self.ui.symbols_cursor < self.ui.symbols_scroll {
            self.ui.symbols_scroll = self.ui.symbols_cursor;
        } else if self.ui.symbols_cursor >= self.ui.symbols_scroll + visible_height {
            self.ui.symbols_scroll = self
                .ui
                .symbols_cursor
                .saturating_sub(visible_height.saturating_sub(1));
        }

        let max_scroll = filtered.len().saturating_sub(visible_height);
        let scroll = self.ui.symbols_scroll.min(max_scroll);

        let lines: Vec<Line> = filtered
            .into_iter()
            .enumerate()
            .skip(scroll)
            .take(visible_height)
            .map(|(i, (addr, name))| {
                let mut spans = vec![];
                let selected = focused && i == self.ui.symbols_cursor;

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
                spans.push(Span::styled(
                    name.clone(),
                    Style::default().fg(if selected { Color::White } else { Color::Cyan }),
                ));
                Line::from(spans)
            })
            .collect();

        let title = if search.is_empty() {
            format!("Symbols [{}/{}]", scroll, max_scroll)
        } else if self.ui.input_mode == InputMode::SearchSymbols {
            "Symbols (Searching...)".to_string()
        } else {
            format!("Symbols (Search: {}) [{}/{}]", search, scroll, max_scroll)
        };

        let paragraph = Paragraph::new(lines).block(self.panel_block(&title, focused));
        frame.render_widget(paragraph, inner);
    }

    fn render_console(&mut self, frame: &mut Frame, area: Rect) {
        self.ui.panel_rects.insert(Panel::Console, area);
        let focused = self.ui.panel == Panel::Console;

        let block = self.panel_block("Console (v: tab)", focused);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner.height == 0 {
            return;
        }

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(inner);

        let titles = vec![
            Line::from(Span::styled(
                " Debugger ",
                if self.ui.console_tab == ConsoleTab::Debugger {
                    Style::default()
                        .fg(self.theme.accent)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(self.theme.dim)
                },
            )),
            Line::from(Span::styled(
                " Tracing ",
                if self.ui.console_tab == ConsoleTab::Tracing {
                    Style::default()
                        .fg(self.theme.accent)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(self.theme.dim)
                },
            )),
        ];

        let tabs = Tabs::new(titles)
            .select(match self.ui.console_tab {
                ConsoleTab::Debugger => 0,
                ConsoleTab::Tracing => 1,
            })
            .divider("│");
        frame.render_widget(tabs, layout[0]);

        if layout[1].height == 0 {
            return;
        }
        let visible_height = layout[1].height as usize;

        let lines = match self.ui.console_tab {
            ConsoleTab::Debugger => self.format_console_logs(
                &self.console_log,
                visible_height,
                layout[1].width as usize,
            ),
            ConsoleTab::Tracing => {
                if let Ok(logs) = self.tracing_log.lock() {
                    self.format_console_logs(&logs, visible_height, layout[1].width as usize)
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
            frame.render_widget(empty, layout[1]);
        } else {
            let paragraph = Paragraph::new(lines);
            frame.render_widget(paragraph, layout[1]);
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
                ConsoleLevel::Panic => ("PANC", self.theme.error),
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
        let scroll_offset = self.ui.console_scroll.min(max_scroll);
        let start_idx = max_scroll.saturating_sub(scroll_offset);

        wrapped_lines
            .into_iter()
            .skip(start_idx)
            .take(visible_height)
            .collect()
    }

    fn render_bottom_bar(&self, frame: &mut Frame, area: Rect) {
        if self.ui.input_mode == InputMode::Command {
            let line = Line::from(vec![
                Span::styled(
                    ":",
                    Style::default()
                        .fg(self.theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    self.ui.input_buffer(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("_", Style::default().fg(self.theme.accent)),
            ]);
            let bar = Paragraph::new(line).style(Style::default().bg(Color::Rgb(30, 30, 50)));
            frame.render_widget(bar, area);
            return;
        }

        let mut spans = vec![
            Span::styled(
                " n ",
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
            Span::styled(
                " t ",
                Style::default().fg(if self.ui.show_targets {
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
                Style::default().fg(self.theme.highlight).add_modifier(Modifier::BOLD)
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

fn extract_register_values_from_asm(asm: &str, x_regs: &[u64; 32]) -> Vec<(String, u64)> {
    let mut regs = Vec::new();
    let mut seen_indices = Vec::new();

    let tokens = asm.split([' ', ',', '(', ')']);
    for token in tokens {
        let token = token.trim();
        if let Some(idx) = parse_register_name(token)
            && idx != 0
            && !seen_indices.contains(&idx)
        {
            seen_indices.push(idx);
            let val = x_regs[idx];
            regs.push((token.to_string(), val));
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

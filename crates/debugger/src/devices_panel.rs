use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Cell, Paragraph, Row, Table, Tabs},
};

use crate::app::Debugger;
use crate::ui_state::DeviceTab;

struct InfoRow {
    label: String,
    value: String,
}

impl InfoRow {
    fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }

    fn separator() -> Self {
        Self {
            label: String::new(),
            value: String::new(),
        }
    }
}

impl Debugger {
    pub(crate) fn render_devices_content(&mut self, frame: &mut Frame, area: Rect) {
        let Some(machine) = self.machine.as_ref() else {
            let msg = Paragraph::new(Span::styled(
                " No machine loaded",
                Style::default().fg(self.theme.dim),
            ));
            frame.render_widget(msg, area);
            return;
        };

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(area);

        let tab_index = match self.ui.devices.tab {
            DeviceTab::Memory => 0,
            DeviceTab::Aclint => 1,
            DeviceTab::Mmu => 2,
        };

        let tab_labels: Vec<Line> = [DeviceTab::Memory, DeviceTab::Aclint, DeviceTab::Mmu]
            .iter()
            .enumerate()
            .map(|(i, t)| {
                if i == tab_index {
                    Line::from(Span::styled(
                        format!(" {} ", t.label()),
                        Style::default()
                            .bg(self.theme.highlight)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    ))
                } else {
                    Line::from(Span::styled(
                        format!(" {} ", t.label()),
                        Style::default().fg(self.theme.dim),
                    ))
                }
            })
            .collect();

        let tabs_widget = Tabs::new(tab_labels)
            .divider(Span::styled("│", Style::default().fg(self.theme.dim)))
            .select(tab_index)
            .highlight_style(Style::default());

        frame.render_widget(tabs_widget, layout[0]);

        if layout[1].height == 0 {
            return;
        }

        let content_area = layout[1];
        let devices = &machine.devices;
        let hart_count = machine.harts.len();

        let rows = match self.ui.devices.tab {
            DeviceTab::Memory => Self::collect_memory_info(&devices.memory),
            DeviceTab::Aclint => Self::collect_aclint_info(&devices.aclint, hart_count),
            DeviceTab::Mmu => Self::collect_mmu_info(),
        };

        self.render_device_table(frame, content_area, &rows);
    }

    fn collect_memory_info(memory: &emulator::devices::Memory) -> Vec<InfoRow> {
        let size = memory.size();
        let size_label = if size >= 1024 * 1024 * 1024 {
            format!("{:.0} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
        } else if size >= 1024 * 1024 {
            format!("{:.0} MB", size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.0} KB", size as f64 / 1024.0)
        };

        vec![
            InfoRow::new("Base Address", format!("{:#010x}", 0x80000000u64)),
            InfoRow::new("Size", format!("{} ({} bytes)", size_label, size)),
            InfoRow::new(
                "End Address",
                format!("{:#010x}", 0x80000000u64 + size as u64),
            ),
            InfoRow::new("Type", "RAM (Read/Write)"),
        ]
    }

    fn collect_aclint_info(aclint: &emulator::devices::Aclint, hart_count: usize) -> Vec<InfoRow> {
        let mtime = aclint.mtime();

        let mut rows = vec![
            InfoRow::new("Base Address", format!("{:#010x}", 0x02000000u64)),
            InfoRow::new(
                "Size",
                format!("{:#x} ({} bytes)", aclint.size(), aclint.size()),
            ),
            InfoRow::new("Harts", format!("{}", hart_count)),
            InfoRow::separator(),
            InfoRow::new("mtime", format!("{:#018x} ({})", mtime, mtime)),
        ];

        for h in 0..hart_count {
            let mtimecmp = aclint.mtimecmp(h);
            let mtip = aclint.mtip(h);
            let msip = aclint.msip(h);
            let ssip = aclint.ssip(h);

            rows.push(InfoRow::separator());
            rows.push(InfoRow::new(
                format!("Hart {} mtimecmp", h),
                format!("{:#018x} ({})", mtimecmp, mtimecmp),
            ));
            rows.push(InfoRow::new(
                format!("Hart {} MTIP", h),
                if mtip { "⚡ Pending" } else { "  Idle" },
            ));
            rows.push(InfoRow::new(
                format!("Hart {} MSIP", h),
                if msip { "⚡ Pending" } else { "  Idle" },
            ));
            rows.push(InfoRow::new(
                format!("Hart {} SSIP", h),
                if ssip { "⚡ Pending" } else { "  Idle" },
            ));
        }

        rows
    }

    fn collect_mmu_info() -> Vec<InfoRow> {
        vec![
            InfoRow::new("Status", "Stub (not yet implemented)"),
            InfoRow::new("Translation Mode", "Bare (no virtual memory)"),
        ]
    }

    fn render_device_table(&self, frame: &mut Frame, area: Rect, info_rows: &[InfoRow]) {
        let visible_height = area.height as usize;
        let total_rows = info_rows.len();
        let max_scroll = total_rows.saturating_sub(visible_height);
        let scroll = self.ui.devices.scroll.min(max_scroll);

        let rows: Vec<Row> = info_rows
            .iter()
            .skip(scroll)
            .take(visible_height)
            .map(|r| {
                if r.label.is_empty() && r.value.is_empty() {
                    Row::new(vec![Cell::from(""), Cell::from("")])
                } else {
                    Row::new(vec![
                        Cell::from(Span::styled(
                            format!(" {}", r.label),
                            Style::default()
                                .fg(self.theme.accent)
                                .add_modifier(Modifier::BOLD),
                        )),
                        Cell::from(Span::styled(
                            r.value.clone(),
                            Style::default().fg(Color::White),
                        )),
                    ])
                }
            })
            .collect();

        let widths = [
            Constraint::Length(area.width.saturating_sub(2) / 3),
            Constraint::Min(0),
        ];

        let table = Table::new(rows, widths);
        frame.render_widget(table, area);
    }
}

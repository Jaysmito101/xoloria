use crate::app::Debugger;
use crate::state::{DataType, HartMode, InputMode, Panel, parse_addr, parse_expr};

#[derive(Debug)]
pub enum BreakpointTarget {
    Address(u64),
    Symbol(String),
}

#[derive(Debug)]
pub enum WatchCommand {
    Add {
        name: String,
        address: u64,
        data_type: DataType,
    },
    Del {
        name: String,
    },
}

#[derive(Debug)]
pub enum DebugCommand {
    Breakpoint(Option<BreakpointTarget>),
    Delete(DeleteTarget),
    Info(InfoTarget),
    Watch(WatchCommand),
    SaveWorkspace,
    LoadWorkspace,
    Memory(u64),
    Step(usize),
    ClearTrace,
    Continue,
    Pause,
    Stall,
    Hart(usize),
    Reset,
    Targets,
    Help,
    ReadMemory {
        data_type: DataType,
        addr_expr: String,
    },
    WriteMemory {
        data_type: DataType,
        addr_expr: String,
        value_expr: String,
    },
    SearchNext(i32, bool),
    Quit,
    GotoAddress(u64),
    GotoMemory(u64),
    DeleteWatchIndex(usize),
    ToggleWatchBreakpoint(usize),
}

#[derive(Debug)]
pub enum DeleteTarget {
    All,
    Address(u64),
}

#[derive(Debug)]
pub enum InfoTarget {
    Breakpoints,
    Registers,
}

impl DebugCommand {
    pub fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Err(String::new());
        }
        match parts[0] {
            "bp" | "break" | "b" => {
                let target = parts.get(1).map(|s| {
                    if let Ok(addr) = parse_addr(s) {
                        BreakpointTarget::Address(addr)
                    } else {
                        BreakpointTarget::Symbol(s.to_string())
                    }
                });
                Ok(Self::Breakpoint(target))
            }
            "watch" => {
                if parts.len() >= 3 && parts[1] == "del" {
                    let name = parts[2..].join(" ");
                    return Ok(Self::Watch(WatchCommand::Del { name }));
                } else if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    let dt_str = parts.last().unwrap();
                    if let Ok(dt) = DataType::parse(dt_str) {
                        let addr_expr = parts[2..parts.len() - 1].join(" ");
                        let addr =
                            parse_addr(&addr_expr).map_err(|_| "Invalid address".to_string())?;
                        return Ok(Self::Watch(WatchCommand::Add {
                            name,
                            address: addr,
                            data_type: dt,
                        }));
                    } else {
                        return Err("Invalid data type".into());
                    }
                } else {
                    return Err("Usage: watch <name> <addr_expr> <type> | watch del <name>".into());
                }
            }
            "clear" => Ok(Self::ClearTrace),
            "save" => Ok(Self::SaveWorkspace),
            "load" => Ok(Self::LoadWorkspace),
            "read" => {
                if parts.len() < 3 {
                    return Err("Usage: read <type> <addr_expr>".into());
                }
                let data_type = DataType::parse(parts[1])?;
                let addr_expr = parts[2..].join(" ");
                Ok(Self::ReadMemory {
                    data_type,
                    addr_expr,
                })
            }
            "write" => {
                if parts.len() < 4 {
                    return Err("Usage: write <type> <addr_expr> = <value_expr>".into());
                }
                let data_type = DataType::parse(parts[1])?;
                let rest = parts[2..].join(" ");
                let (addr_expr, value_expr) = if let Some((a, v)) = rest.split_once('=') {
                    (a.trim().to_string(), v.trim().to_string())
                } else {
                    return Err("Usage: write <type> <addr_expr> = <value_expr>\nExample: write u32 0x1000 = 0x50".into());
                };
                Ok(Self::WriteMemory {
                    data_type,
                    addr_expr,
                    value_expr,
                })
            }
            "del" | "delete" => {
                if parts.get(1).copied() == Some("all") {
                    Ok(Self::Delete(DeleteTarget::All))
                } else if let Some(addr_str) = parts.get(1) {
                    let addr = parse_addr(addr_str)?;
                    Ok(Self::Delete(DeleteTarget::Address(addr)))
                } else {
                    Err("Usage: del <addr> | del all".into())
                }
            }
            "info" => match parts.get(1).copied() {
                Some("bp") | Some("break") => Ok(Self::Info(InfoTarget::Breakpoints)),
                Some("reg") | Some("regs") => Ok(Self::Info(InfoTarget::Registers)),
                _ => Err("Usage: info bp | info reg".into()),
            },
            "mem" | "x" => {
                if let Some(addr_str) = parts.get(1) {
                    let addr = parse_addr(addr_str)?;
                    Ok(Self::Memory(addr))
                } else {
                    Err("Usage: mem <addr>".into())
                }
            }
            "step" | "s" | "si" => {
                let n = parts
                    .get(1)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(1);
                Ok(Self::Step(n))
            }
            "continue" | "c" => Ok(Self::Continue),
            "pause" | "p" => Ok(Self::Pause),
            "stall" => Ok(Self::Stall),
            "hart" => {
                if let Some(idx) = parts.get(1).and_then(|s| s.parse::<usize>().ok()) {
                    Ok(Self::Hart(idx))
                } else {
                    Err("Usage: hart <n>".into())
                }
            }
            "reset" => Ok(Self::Reset),
            "targets" | "t" => Ok(Self::Targets),
            "help" | "?" => Ok(Self::Help),
            other => Err(format!(
                "Unknown command: {}. Type 'help' for commands",
                other
            )),
        }
    }
}

impl Debugger {
    pub(crate) fn execute_input_buffer_command(&mut self) {
        let input = self.ui.input_buffer_take();
        let cmd = input.trim().to_string();
        self.ui.set_input_mode(InputMode::Normal);

        if cmd.is_empty() {
            return;
        }

        self.ui.push_command_history(cmd.clone());

        match DebugCommand::parse(&cmd) {
            Err(msg) => {
                if !msg.is_empty() {
                    self.set_error(msg);
                }
            }
            Ok(command) => self.execute_command(command),
        }
    }

    pub(crate) fn execute_command(&mut self, command: DebugCommand) {
        match command {
            DebugCommand::Breakpoint(target) => match target {
                None => {
                    let addr = self
                        .machine
                        .as_ref()
                        .map(|m| m.harts[self.ui.selected_hart].registers().pc())
                        .unwrap_or(0);
                    self.toggle_breakpoint_at(addr);
                }
                Some(BreakpointTarget::Address(addr)) => {
                    self.toggle_breakpoint_at(addr);
                }
                Some(BreakpointTarget::Symbol(name)) => {
                    let mut found = None;
                    for (a, n) in &self.sorted_symbols {
                        if n == &name {
                            found = Some(*a);
                            break;
                        }
                    }
                    if let Some(addr) = found {
                        self.toggle_breakpoint_at(addr);
                    } else {
                        self.set_error(format!("Symbol '{}' not found", name));
                    }
                }
            },
            DebugCommand::Delete(target) => match target {
                DeleteTarget::All => {
                    let count = self.breakpoints.len();
                    self.breakpoints.clear();
                    self.set_info(format!("Cleared {} breakpoints", count));
                    self.disasm_cache = None;
                }
                DeleteTarget::Address(addr) => {
                    if self.breakpoints.remove(&addr) {
                        self.set_info(format!("Removed breakpoint at {:#x}", addr));
                        self.disasm_cache = None;
                    } else {
                        self.set_error(format!("No breakpoint at {:#x}", addr));
                    }
                }
            },
            DebugCommand::Info(target) => match target {
                InfoTarget::Breakpoints => {
                    if self.breakpoints.is_empty() {
                        self.set_info("No breakpoints set");
                    } else {
                        let list: Vec<String> = self
                            .breakpoints
                            .iter()
                            .map(|a| format!("{:#x}", a))
                            .collect();
                        self.set_info(format!("Breakpoints: {}", list.join(", ")));
                    }
                }
                InfoTarget::Registers => {
                    if let Some(m) = self.machine.as_ref() {
                        let regs = m.harts[self.ui.selected_hart].registers();
                        self.set_info(format!("pc={:#x}", regs.pc()));
                    } else {
                        self.set_error("No machine");
                    }
                }
            },
            DebugCommand::Memory(addr) => {
                self.ui.memory_addr = addr;
                self.ui.panel = Panel::Memory;
                self.set_info(format!("Memory at {:#x}", addr));
            }
            DebugCommand::Step(n) => {
                if self.hart_modes[self.ui.selected_hart] == HartMode::Debug {
                    self.step_hart(n);
                    self.set_info(format!("Stepped {} instruction(s)", n));
                } else {
                    self.set_error("Hart is not in Debug mode");
                }
            }
            DebugCommand::Continue => {
                self.hart_modes[self.ui.selected_hart] = HartMode::Running;
                self.set_info("Continuing...");
            }
            DebugCommand::Pause => {
                self.hart_modes[self.ui.selected_hart] = HartMode::Debug;
                self.set_info("Paused");
            }
            DebugCommand::Stall => {
                self.hart_modes[self.ui.selected_hart] = HartMode::Stalled;
                self.set_info("Stalled");
            }
            DebugCommand::Hart(idx) => {
                if idx < self.hart_modes.len() {
                    self.ui.selected_hart = idx;
                    self.ui.disasm.cursor = 0;
                    self.set_info(format!("Selected hart {}", idx));
                    self.disasm_cache = None;
                } else {
                    self.set_error(format!(
                        "Hart {} does not exist (0-{})",
                        idx,
                        self.hart_modes.len() - 1
                    ));
                }
            }
            DebugCommand::Reset => {
                let config = crate::state::MachineConfig {
                    harts: self.config_harts,
                    memory_size: self.memory_size(),
                };
                self.rebuild_machine(config);
                self.tick_count = 0;
                self.ui.disasm.cursor = 0;
                self.tracing_log.lock().unwrap().clear();
                self.console_log.clear();
                self.ui.trace.stack.clear();
                self.ui.trace.forward_stack.clear();
                self.set_info("Machine reset");
            }
            DebugCommand::ClearTrace => {
                self.ui.trace.stack.clear();
                self.ui.trace.forward_stack.clear();
                self.set_info("Trace cleared");
            }
            DebugCommand::Targets => {
                self.ui.disasm.show_targets = !self.ui.disasm.show_targets;
                self.set_info(if self.ui.disasm.show_targets {
                    "Jump targets: ON"
                } else {
                    "Jump targets: OFF"
                });
            }
            DebugCommand::Help => {
                self.set_info(
                        "bp [addr|symbol] | del <addr|all> | info bp | mem <addr> | step [n] | continue | pause | hart <n> | reset | targets | save | load | help"
                    );
            }
            DebugCommand::ReadMemory {
                data_type,
                addr_expr,
            } => match parse_addr(&addr_expr) {
                Ok(addr) => self.do_read_memory(data_type, addr),
                Err(e) => self.set_error(e),
            },
            DebugCommand::WriteMemory {
                data_type,
                addr_expr,
                value_expr,
            } => match (parse_addr(&addr_expr), parse_expr(&value_expr)) {
                (Ok(addr), Ok(val)) => self.do_write_memory(data_type, addr, val),
                (Err(e), _) => self.set_error(e),
                (_, Err(e)) => self.set_error(e),
            },
            DebugCommand::Watch(cmd) => match cmd {
                WatchCommand::Add {
                    name,
                    address,
                    data_type,
                } => {
                    if self.watches.iter().any(|w| w.name == name) {
                        self.set_error(format!("Watchpoint with name '{}' already exists", name));
                    } else {
                        self.watches.push(crate::state::WatchItem {
                            name: name.clone(),
                            address,
                            data_type,
                            break_on_change: false,
                        });
                        self.set_info(format!(
                            "Added watchpoint '{}' for address {:#x}",
                            name, address
                        ));
                    }
                }
                WatchCommand::Del { name } => {
                    if let Some(pos) = self.watches.iter().position(|w| w.name == name) {
                        self.watches.remove(pos);
                        self.set_info(format!("Deleted watchpoint '{}'", name));
                    } else {
                        self.set_error(format!("Watchpoint '{}' not found", name));
                    }
                }
            },
            DebugCommand::SaveWorkspace => self.save_workspace(),
            DebugCommand::LoadWorkspace => self.load_workspace(),
            DebugCommand::SearchNext(delta, wrap) => self.search_next(delta, wrap),
            DebugCommand::Quit => self.running = false,
            DebugCommand::GotoAddress(addr) => {
                let (_target_addr, target_entry, _entries) = self.resolve_cursor_target();
                if let Some(entry) = target_entry {
                    self.ui.disasm.view_history.push(entry.addr);
                } else if let Some(center) = self.ui.disasm.view_center_addr {
                    self.ui.disasm.view_history.push(center);
                }
                self.ui.disasm.view_center_addr = Some(addr);
                self.ui.disasm.cursor = 0;
                self.disasm_cache = None;
                self.ui.panel = Panel::Disassembly;
            }
            DebugCommand::GotoMemory(addr) => {
                self.ui.memory_addr = addr;
                self.ui.panel = Panel::Memory;
            }
            DebugCommand::DeleteWatchIndex(idx) => {
                if idx < self.watches.len() {
                    self.watches.remove(idx);
                    if self.ui.watch_cursor >= self.watches.len() && !self.watches.is_empty() {
                        self.ui.watch_cursor = self.watches.len() - 1;
                    }
                }
            }
            DebugCommand::ToggleWatchBreakpoint(idx) => {
                if idx < self.watches.len() {
                    self.watches[idx].break_on_change = !self.watches[idx].break_on_change;
                }
            }
        }
    }
}

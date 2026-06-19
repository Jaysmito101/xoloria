use std::fmt::Display;

use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HartMode {
    Debug,
    Running,
    Stalled,
}

impl Display for HartMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Debug => write!(f, "Debug"),
            Self::Running => write!(f, "Running"),
            Self::Stalled => write!(f, "Stalled"),
        }
    }
}

impl HartMode {
    pub fn next(self) -> Self {
        match self {
            Self::Debug => Self::Running,
            Self::Running => Self::Stalled,
            Self::Stalled => Self::Debug,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Debug => Self::Stalled,
            Self::Running => Self::Debug,
            Self::Stalled => Self::Running,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Panel {
    Registers,
    Disassembly,
    Csr,
    Memory,
    Symbols,
    Console,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleTab {
    Debugger,
    Tracing,
}

impl ConsoleTab {
    pub fn next(self) -> Self {
        match self {
            Self::Debugger => Self::Tracing,
            Self::Tracing => Self::Debugger,
        }
    }
}

impl Panel {
    pub fn next(self) -> Self {
        match self {
            Self::Registers => Self::Disassembly,
            Self::Disassembly => Self::Csr,
            Self::Csr => Self::Memory,
            Self::Memory => Self::Symbols,
            Self::Symbols => Self::Console,
            Self::Console => Self::Registers,
        }
    }

    pub fn nav(self, dir: Direction) -> Self {
        match (self, dir) {
            (Self::Registers, Direction::Right) => Self::Disassembly,
            (Self::Registers, Direction::Down) => Self::Csr,
            (Self::Csr, Direction::Right) => Self::Disassembly,
            (Self::Csr, Direction::Up) => Self::Registers,
            (Self::Csr, Direction::Down) => Self::Memory,
            (Self::Disassembly, Direction::Left) => Self::Registers,
            (Self::Disassembly, Direction::Down) => Self::Symbols,
            (Self::Memory, Direction::Up) => Self::Csr,
            (Self::Memory, Direction::Left) => Self::Csr,
            (Self::Memory, Direction::Right) => Self::Symbols,
            (Self::Memory, Direction::Down) => Self::Console,
            (Self::Symbols, Direction::Up) => Self::Disassembly,
            (Self::Symbols, Direction::Left) => Self::Memory,
            (Self::Symbols, Direction::Down) => Self::Console,
            (Self::Console, Direction::Up) => Self::Memory,
            (Self::Console, Direction::Left) => Self::Csr,
            (Self::Console, Direction::Right) => Self::Symbols,
            _ => self,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Setup,
    Debug,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    GotoMemory,
    Command,
    SearchSymbols,
}

pub struct Theme {
    pub accent: Color,
    pub dim: Color,
    pub highlight: Color,
    pub error: Color,
    pub warn: Color,
    pub running: Color,
    pub stalled: Color,
    pub border: Color,
    pub jump: Color,
    pub branch: Color,
    pub system: Color,
    pub breakpoint: Color,
    pub cursor_bg: Color,
    pub target: Color,
    pub info: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            accent: Color::Cyan,
            dim: Color::DarkGray,
            highlight: Color::Yellow,
            error: Color::Red,
            warn: Color::Yellow,
            running: Color::Green,
            stalled: Color::Rgb(180, 80, 80),
            border: Color::Rgb(80, 80, 100),
            jump: Color::Magenta,
            branch: Color::Yellow,
            system: Color::Red,
            breakpoint: Color::Red,
            cursor_bg: Color::Rgb(50, 50, 70),
            target: Color::Rgb(120, 200, 120),
            info: Color::Rgb(100, 180, 255),
        }
    }
}

impl Theme {
    pub fn mode_color(&self, mode: HartMode) -> Color {
        match mode {
            HartMode::Debug => self.accent,
            HartMode::Running => self.running,
            HartMode::Stalled => self.stalled,
        }
    }

    pub fn instruction_color(&self, text: &str) -> Color {
        let mnemonic = text.split_whitespace().next().unwrap_or("");
        if matches!(mnemonic, "jal" | "jalr") {
            self.jump
        } else if matches!(mnemonic, "beq" | "bne" | "blt" | "bge" | "bltu" | "bgeu") {
            self.branch
        } else if matches!(
            mnemonic,
            "ecall" | "ebreak" | "mret" | "sret" | "wfi" | "fence" | "fence.i"
        ) {
            self.system
        } else {
            Color::White
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleLevel {
    Info,
    Error,
    Warn,
    Panic,
}

#[derive(Debug, Clone)]
pub struct ConsoleEntry {
    pub message: String,
    pub level: ConsoleLevel,
    pub tick: u64,
}

pub struct MachineConfig {
    pub harts: usize,
    pub memory_size: usize,
}

#[derive(Debug)]
pub enum BreakpointTarget {
    Address(u64),
    Symbol(String),
}

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    U8, U16, U32, U64,
    I8, I16, I32, I64,
}

impl DataType {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "u8" => Ok(Self::U8),
            "u16" => Ok(Self::U16),
            "u32" => Ok(Self::U32),
            "u64" => Ok(Self::U64),
            "i8" => Ok(Self::I8),
            "i16" => Ok(Self::I16),
            "i32" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            _ => Err(format!("Unknown data type: {}", s)),
        }
    }
}

#[derive(Debug)]
pub enum DebugCommand {
    Breakpoint(Option<BreakpointTarget>),
    Delete(DeleteTarget),
    Info(InfoTarget),
    Memory(u64),
    Step(usize),
    Continue,
    Pause,
    Stall,
    Hart(usize),
    Reset,
    Targets,
    Help,
    SaveBreakpoints,
    LoadBreakpoints,
    ReadMemory { data_type: DataType, addr_expr: String },
    WriteMemory { data_type: DataType, addr_expr: String, value_expr: String },
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
            "save" => {
                if parts.get(1).copied() == Some("bp") || parts.get(1).copied() == Some("breakpoints") {
                    Ok(Self::SaveBreakpoints)
                } else {
                    Err("Usage: save bp".into())
                }
            }
            "load" => {
                if parts.get(1).copied() == Some("bp") || parts.get(1).copied() == Some("breakpoints") {
                    Ok(Self::LoadBreakpoints)
                } else {
                    Err("Usage: load bp".into())
                }
            }
            "read" => {
                if parts.len() < 3 {
                    return Err("Usage: read <type> <addr_expr>".into());
                }
                let data_type = DataType::parse(parts[1])?;
                let addr_expr = parts[2..].join(" ");
                Ok(Self::ReadMemory { data_type, addr_expr })
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
                Ok(Self::WriteMemory { data_type, addr_expr, value_expr })
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

pub(crate) fn parse_expr(s: &str) -> Result<u64, String> {
    let re = regex::Regex::new(r"(?i)\b(?:0x)?([0-9a-f]+)\b").expect("Failed to create regex for parsing expr");
    let expr_str = re.replace_all(s, "0x$1");

    match evalexpr::eval(&expr_str) {
        Ok(evalexpr::Value::Int(val)) => Ok(val as u64),
        Ok(_) => Err(format!("Expression evaluated to non-integer: {}", s)),
        Err(e) => Err(format!("Failed to evaluate expression '{}': {}", s, e)),
    }
}

pub(crate) fn parse_addr(s: &str) -> Result<u64, String> {
    parse_expr(s).map(|mut addr| {
        if addr < 0x80000000 {
            addr |= 0x80000000;
        }
        addr
    })
}

use std::fmt::Display;

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Panel {
    Registers,
    Disassembly,
    Csr,
    Memory,
    Symbols,
    Console,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ConsoleTab {
    #[default]
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
    GotoAddress,
    Command,
    Search,
    EditWatch(usize),
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


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DataType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
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

    pub fn size_bytes(&self) -> i32 {
        match self {
            Self::U8 | Self::I8 => 1,
            Self::U16 | Self::I16 => 2,
            Self::U32 | Self::I32 => 4,
            Self::U64 | Self::I64 => 8,
        }
    }
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
        };
        write!(f, "{}", name)
    }
}



pub(crate) fn parse_expr(s: &str) -> Result<u64, String> {
    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(r"(?i)\b(?:0x)?([0-9a-f]+)\b")
            .expect("Failed to create regex for parsing expr")
    });
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchItem {
    pub name: String,
    pub address: u64,
    pub data_type: DataType,
    pub break_on_change: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Workspace {
    pub breakpoints: Vec<u64>,
    pub watches: Vec<WatchItem>,
    #[serde(default)]
    pub panel: Option<Panel>,
    #[serde(default)]
    pub disasm_tab: Option<crate::ui_state::DisasmTab>,
    #[serde(default)]
    pub registers_tab: Option<crate::ui_state::RegistersTab>,
    #[serde(default)]
    pub memory_tab: Option<crate::ui_state::MemoryTab>,
    #[serde(default)]
    pub symbols_tab: Option<crate::ui_state::SymbolsTab>,
    #[serde(default)]
    pub console_tab: Option<ConsoleTab>,
}

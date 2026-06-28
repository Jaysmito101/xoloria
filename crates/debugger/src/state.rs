use emulator::registers::{ControlRegisterName, GeneralRegisterName};
use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use std::fmt::Display;
use std::str::FromStr;
use strum::IntoEnumIterator;

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
    Disassembly,
    Registers,
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
            Self::Disassembly => Self::Registers,
            Self::Registers => Self::Memory,
            Self::Memory => Self::Symbols,
            Self::Symbols => Self::Console,
            Self::Console => Self::Disassembly,
        }
    }

    pub fn nav(self, dir: Direction) -> Self {
        match (self, dir) {
            (Self::Registers, Direction::Right) => Self::Disassembly,
            (Self::Registers, Direction::Down) => Self::Memory,
            (Self::Memory, Direction::Right) => Self::Disassembly,
            (Self::Memory, Direction::Up) => Self::Registers,
            (Self::Memory, Direction::Down) => Self::Console,
            (Self::Disassembly, Direction::Left) => Self::Memory,
            (Self::Disassembly, Direction::Down) => Self::Symbols,
            (Self::Console, Direction::Up) => Self::Memory,
            (Self::Console, Direction::Right) => Self::Symbols,
            (Self::Symbols, Direction::Up) => Self::Disassembly,
            (Self::Symbols, Direction::Left) => Self::Console,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputMode {
    Normal,
    GotoMemory,
    GotoAddress,
    Command,
    Search,
    SearchRegisters,
    EditWatch(usize),
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

impl WatchItem {
    pub fn read_value(&self, bus: &impl emulator::BusIO) -> Vec<u8> {
        let mut data = vec![0u8; self.data_type.size_bytes() as usize];
        for (i, b) in data.iter_mut().enumerate() {
            *b = bus.read::<u8>(self.address + i as u64).unwrap_or(0);
        }
        data
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegisterIdentifier {
    Gpr(GeneralRegisterName),
    Csr(ControlRegisterName),
    Pc,
}

impl Display for RegisterIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gpr(g) => write!(f, "{}", g),
            Self::Csr(c) => write!(f, "{}", c),
            Self::Pc => write!(f, "pc"),
        }
    }
}

impl FromStr for RegisterIdentifier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        if lower == "pc" {
            return Ok(Self::Pc);
        }

        for gpr in GeneralRegisterName::iter() {
            if format!("{}", gpr).to_lowercase() == lower {
                return Ok(Self::Gpr(gpr));
            }
        }

        for csr in ControlRegisterName::iter() {
            if format!("{}", csr).to_lowercase() == lower {
                return Ok(Self::Csr(csr));
            }
        }

        Err(format!("Unknown register: {}", s))
    }
}

impl Serialize for RegisterIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for RegisterIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<RegisterIdentifier>()
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Workspace {
    pub breakpoints: Vec<u64>,
    pub watches: Vec<WatchItem>,
    pub pinned_registers: Vec<RegisterIdentifier>,
    pub register_watchpoints: Vec<RegisterIdentifier>,
    pub ui: Option<crate::ui_state::UiState>,
}

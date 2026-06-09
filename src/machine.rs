use crate::{Address, Bus, BusIO, Hart, Memory, MemoryManagementUnit, Result};

struct MachineParams {
    harts: usize,
    memory: usize,
    _name: String,
}

pub struct MachineBuilder {
    inner: MachineParams,
}

impl MachineBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            inner: MachineParams {
                harts: 1,
                memory: 1024 * 16,
                _name: name.into(),
            },
        }
    }

    pub fn with_harts(mut self, count: usize) -> Result<Self> {
        if count == 0 {
            return crate::err!(crate::Error::InvalidParameter(
                "Hart count must be greater than 0".into(),
            ));
        }
        self.inner.harts = count;
        Ok(self)
    }

    pub fn with_memory(mut self, size: usize) -> Result<Self> {
        if size == 0 {
            return crate::err!(crate::Error::InvalidParameter(
                "Memory must be greater than 0".into(),
            ));
        }
        self.inner.memory = size;
        Ok(self)
    }

    pub fn build(self) -> Result<Machine> {
        Machine::new(self.inner)
    }
}

pub struct Machine {
    bus: Bus,
    _mmu: MemoryManagementUnit,
    harts: Vec<Hart>,
}

impl Machine {
    fn new(params: MachineParams) -> Result<Self> {
        let mmu = MemoryManagementUnit::new()?;
        let mut bus = Bus::new()?;

        bus.map(
            0x80000000,
            0x80000000 + params.memory as Address,
            Memory::new(params.memory)?,
        )?;

        let harts = (0..params.harts)
            .map(|id| Hart::new(id as u64))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            bus,
            _mmu: mmu,
            harts,
        })
    }

    pub fn load_binary(&mut self, location: Address, binary: &[u8]) -> Result<()> {
        self.bus.write(location, binary)?;
        Ok(())
    }

    pub fn simulate(self) -> Result<()> {
        Ok(())
    }
}

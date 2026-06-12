use std::sync::Arc;

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
    bus: Arc<Bus>,
    _mmu: Arc<MemoryManagementUnit>,
    harts: Vec<Hart>,
    cycle_count: u64,
}

impl Machine {
    const RTC_DIVISOR: u64 = 100;

    fn new(params: MachineParams) -> Result<Self> {
        let mmu = Arc::new(MemoryManagementUnit::new()?);
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
            bus: Arc::new(bus),
            _mmu: mmu,
            harts,
            cycle_count: 0,
        })
    }

    pub fn load_binary(&self, location: Address, binary: &[u8]) -> Result<()> {
        self.bus.write(location, binary)?;
        Ok(())
    }

    pub fn simulate(self) -> Result<()> {
        let mut threads = Vec::new();
        for mut hart in self.harts {
            let bus = self.bus.clone();
            let handle = std::thread::Builder::new()
                .name(format!("Hart-{}", hart.id()))
                .spawn(move || {
                    loop {
                        hart.tick(&bus).unwrap();
                    }
                })
                .map_err(crate::Error::ThreadSpawnFailed)?;
            threads.push(handle);
        }
        for handle in threads {
            let name = handle.thread().name().unwrap_or("Unknown").to_owned();
            let _ = handle
                .join()
                .inspect_err(|e| {
                    tracing::error!("Thread [{}] Join Failed: {:?}", name, e);
                })
                .ok();
        }

        Ok(())
    }
}

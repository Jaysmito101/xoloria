use std::sync::Arc;

use crate::{
    Address, Bus, BusIO, Hart, MemoryManagementUnit, Result,
    devices::{Aclint, Memory},
};

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

pub struct Devices {
    pub memory: Arc<Memory>,
    pub aclint: Arc<Aclint>,
    pub mmu: Arc<MemoryManagementUnit>,
}

pub struct Machine {
    pub bus: Arc<Bus>,
    pub harts: Vec<Hart>,
    pub cycle_count: u64,
    pub devices: Devices,
}

impl Devices {
    fn new(params: &MachineParams) -> Result<Self> {
        Ok(Self {
            memory: Arc::new(Memory::new(params.memory)?),
            aclint: Arc::new(Aclint::new(params.harts)),
            mmu: Arc::new(MemoryManagementUnit::new()?),
        })
    }

    fn map_to(&self, mut bus: Bus) -> Result<Bus> {
        bus.map(0x80000000, self.memory.size() as u64, self.memory.clone())?;
        bus.map(0x02000000, 0xC0000, self.aclint.clone())?;
        Ok(bus)
    }
}

impl Machine {
    fn new(params: MachineParams) -> Result<Self> {
        let devices = Devices::new(&params)?;
        let bus = devices.map_to(Bus::new()?)?;

        let harts = (0..params.harts)
            .map(|id| Hart::new(id as u64))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            bus: Arc::new(bus),
            harts,
            cycle_count: 0,
            devices,
        })
    }

    pub fn load_binary(&self, location: Address, binary: &[u8]) -> Result<()> {
        self.bus.write_bytes(location, binary)?;
        Ok(())
    }

    pub fn simulate(self) -> Result<()> {
        let mut threads = Vec::new();

        std::thread::Builder::new()
            .name("ACLINT".into())
            .spawn({
                let aclint = self.devices.aclint.clone();
                move || loop {
                    aclint.tick();
                }
            })
            .map_err(crate::Error::ThreadSpawnFailed)?;

        for mut hart in self.harts {
            let bus = self.bus.clone();
            let handle = std::thread::Builder::new()
                .name(format!("Hart-{}", hart.id()))
                .spawn(move || {
                    loop {
                        if let Err(e) = hart.tick(&bus) {
                            tracing::error!("Hart [{}] Tick Failed: {:?}", hart.id(), e);
                            break;
                        }
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

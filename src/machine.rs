use crate::Result;

struct MachineParams {
    harts: usize,
    memory: usize,
    name: String,
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
                name: name.into(),
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
        Ok(Machine::new(self.inner))
    }
}

pub struct Machine {}

impl Machine {
    fn new(params: MachineParams) -> Self {
        Self {}
    }

    pub fn load_binary(&mut self, location: usize, binary: &[u8]) -> Result<()> {
        Ok(())
    }

    pub fn simulate(mut self) -> Result<()> {
        Ok(())
    }
}

use crate::Result;

pub struct MachineBuilder {
    harts: usize,
    name: String,
}

pub struct Machine {}

impl MachineBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            harts: 1,
            name: name.into(),
        }
    }

    pub fn with_harts(mut self, count: usize) -> Result<Self> {
        if count == 0 {
            return crate::err!(crate::Error::InvalidParameter(
                "Hart count must be greater than 0".into(),
            ));
        }
        self.harts = count;
        Ok(self)
    }

    pub fn build(self) -> Result<Machine> {
        Ok(Machine {})
    }
}

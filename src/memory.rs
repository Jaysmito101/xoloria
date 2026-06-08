use crate::bus::BusIO;
use crate::{Address, Result};

pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Result<Self> {
        let mut data = Vec::new();
        data.try_reserve_exact(size)
            .map_err(crate::Error::AllocationFailed)?;
        data.resize(size, 0);
        Ok(Self { data })
    }
}

impl BusIO for Memory {
    fn read(&mut self, offset: Address, buffer: &mut [u8]) -> Result<()> {
        let end = offset
            .checked_add(buffer.len() as u64)
            .ok_or(crate::Error::OutOfBounds(offset))?;
        if end > self.data.len() as u64 {
            return crate::err!(crate::Error::OutOfBounds(offset));
        }
        buffer.copy_from_slice(&self.data[offset as usize..end as usize]);
        Ok(())
    }

    fn write(&mut self, offset: Address, data: &[u8]) -> Result<()> {
        let end = offset
            .checked_add(data.len() as u64)
            .ok_or(crate::Error::OutOfBounds(offset))?;
        if end > self.data.len() as u64 {
            return crate::err!(crate::Error::OutOfBounds(offset));
        }
        self.data[offset as usize..end as usize].copy_from_slice(data);
        Ok(())
    }
}

impl From<Memory> for crate::bus::BusDevice {
    fn from(val: Memory) -> Self {
        crate::bus::BusDevice::Memory(val)
    }
}

use crate::bus::{BusError, BusIO, BusResult};
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
    fn read(&mut self, offset: Address, buffer: &mut [u8]) -> BusResult<()> {
        let end = offset
            .checked_add(buffer.len() as u64)
            .ok_or(BusError::AddressOverflow(offset, buffer.len()))?;

        if end > self.data.len() as u64 {
            return Err(BusError::IndexOutOfBounds(
                end,
                0..self.data.len() as Address,
            ));
        }

        buffer.copy_from_slice(&self.data[offset as usize..end as usize]);
        Ok(())
    }

    fn write(&mut self, offset: Address, buffer: &[u8]) -> BusResult<()> {
        let end = offset
            .checked_add(buffer.len() as u64)
            .ok_or(BusError::AddressOverflow(offset, buffer.len()))?;

        if end > self.data.len() as u64 {
            return Err(BusError::IndexOutOfBounds(
                end,
                0..self.data.len() as Address,
            ));
        }

        self.data[offset as usize..end as usize].copy_from_slice(buffer);
        Ok(())
    }
}

impl From<Memory> for crate::bus::BusDevice {
    fn from(val: Memory) -> Self {
        crate::bus::BusDevice::Memory(val)
    }
}

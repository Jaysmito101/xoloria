use std::ops::Range;

use crate::{Memory, Result};

pub type Address = u64;

pub enum BusDevice {
    Memory(Memory),
    Generic(Box<dyn BusIO + Send + Sync>),
}

#[derive(Debug)]
pub enum BusError {
    UnmappedLocation(Address),
    IndexOutOfBounds(Address, Range<Address>),
    AddressOverflow(Address, usize),
}

pub type BusResult<T> = std::result::Result<T, BusError>;

pub trait BusIO {
    fn read(&mut self, offset: Address, buffer: &mut [u8]) -> BusResult<()>;
    fn write(&mut self, offset: Address, data: &[u8]) -> BusResult<()>;

    #[inline]
    fn read_u8(&mut self, offset: Address) -> BusResult<u8> {
        let mut buffer = [0u8; 1];
        self.read(offset, &mut buffer)?;
        Ok(buffer[0])
    }

    #[inline]
    fn write_u8(&mut self, offset: Address, data: u8) -> BusResult<()> {
        self.write(offset, &[data])
    }

    #[inline]
    fn read_u16(&mut self, offset: Address) -> BusResult<u16> {
        let mut buffer = [0u8; 2];
        self.read(offset, &mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    #[inline]
    fn write_u16(&mut self, offset: Address, data: u16) -> BusResult<()> {
        self.write(offset, &data.to_le_bytes())
    }

    #[inline]
    fn read_u32(&mut self, offset: Address) -> BusResult<u32> {
        let mut buffer = [0u8; 4];
        self.read(offset, &mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    #[inline]
    fn write_u32(&mut self, offset: Address, data: u32) -> BusResult<()> {
        self.write(offset, &data.to_le_bytes())
    }

    #[inline]
    fn read_u64(&mut self, offset: Address) -> BusResult<u64> {
        let mut buffer = [0u8; 8];
        self.read(offset, &mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }

    #[inline]
    fn write_u64(&mut self, offset: Address, data: u64) -> BusResult<()> {
        self.write(offset, &data.to_le_bytes())
    }
}

impl BusIO for BusDevice {
    fn read(&mut self, offset: Address, buffer: &mut [u8]) -> BusResult<()> {
        match self {
            BusDevice::Memory(memory) => memory.read(offset, buffer),
            BusDevice::Generic(device) => device.read(offset, buffer),
        }
    }

    fn write(&mut self, offset: Address, data: &[u8]) -> BusResult<()> {
        match self {
            BusDevice::Memory(memory) => memory.write(offset, data),
            BusDevice::Generic(device) => device.write(offset, data),
        }
    }
}

struct BusMapping {
    range: Range<Address>,
    handler: BusDevice,
}

impl BusMapping {
    #[inline]
    pub fn contains(&self, location: Address) -> bool {
        self.range.contains(&location)
    }
}

pub struct Bus {
    mappings: Vec<BusMapping>,
}

impl Bus {
    pub fn new() -> Result<Self> {
        Ok(Self { mappings: vec![] })
    }

    #[inline]
    pub fn is_mapped(&self, location: Address) -> bool {
        self.mapping_index(location).is_some()
    }

    pub fn map<D>(&mut self, start: Address, end: Address, device: D) -> Result<()>
    where
        D: Into<BusDevice>,
    {
        if start >= end {
            return crate::err!(crate::Error::InvalidParameter(format!(
                "Invalid mapping range: {start} to {end}"
            )));
        }

        for mapping in &self.mappings {
            if mapping.range.start.max(start) < mapping.range.end.min(end) {
                return crate::err!(crate::Error::InvalidParameter(format!(
                    "Cannot map on overlapped ranges {:?} | {:?}",
                    (start, end),
                    mapping.range
                )));
            }
        }

        let mapping = BusMapping {
            range: start..end,
            handler: device.into(),
        };

        let pos = self
            .mappings
            .binary_search_by_key(&start, |mapping| mapping.range.start)
            .unwrap_or_else(|e| e);
        self.mappings.insert(pos, mapping);

        Ok(())
    }

    fn mapping_mut(&mut self, location: Address) -> Option<&mut BusMapping> {
        self.mapping_index(location)
            .map(|index| &mut self.mappings[index])
    }

    fn mapping_index(&self, location: Address) -> Option<usize> {
        self.mappings
            .partition_point(|mapping| mapping.range.start <= location)
            .checked_sub(1)
            .filter(|&i| self.mappings[i].contains(location))
    }
}

impl BusIO for Bus {
    fn read(&mut self, location: Address, buffer: &mut [u8]) -> BusResult<()> {
        let mapping = self
            .mapping_mut(location)
            .ok_or(BusError::UnmappedLocation(location))?;

        if buffer.len() as u64 > mapping.range.end - location {
            return Err(BusError::IndexOutOfBounds(location, mapping.range.clone()));
        }

        mapping.handler.read(location - mapping.range.start, buffer)
    }

    fn write(&mut self, location: Address, data: &[u8]) -> BusResult<()> {
        let mapping = self
            .mapping_mut(location)
            .ok_or(BusError::UnmappedLocation(location))?;

        if data.len() as u64 > mapping.range.end - location {
            return Err(BusError::IndexOutOfBounds(location, mapping.range.clone()));
        }

        mapping.handler.write(location - mapping.range.start, data)
    }
}

use std::{ops::Range, sync::Arc};

use crate::{
    Result,
    devices::{Aclint, Memory},
};

pub type Address = u64;

pub enum BusDevice {
    Memory(Arc<Memory>),
    Aclint(Arc<Aclint>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BusError {
    UnmappedLocation(Address),
    IndexOutOfBounds(Address, Range<Address>),
    AddressOverflow(Address, usize),
    LockFailed,
}

pub type BusResult<T> = std::result::Result<T, BusError>;

pub trait BusOperable: Sized + Copy + Send + Sync {}

impl BusOperable for u8 {}
impl BusOperable for u16 {}
impl BusOperable for u32 {}
impl BusOperable for u64 {}

pub trait BusIO {
    fn read<T>(&self, offset: Address) -> BusResult<T>
    where
        T: BusOperable;

    fn write<T>(&self, offset: Address, data: T) -> BusResult<()>
    where
        T: BusOperable;

    fn rmw<T, F>(&self, offset: Address, f: F) -> BusResult<T>
    where
        T: BusOperable,
        F: FnOnce(T) -> T;

    fn write_bytes(&self, offset: Address, data: &[u8]) -> BusResult<()> {
        for (i, &byte) in data.iter().enumerate() {
            self.write(offset + i as u64, byte)?;
        }
        Ok(())
    }
}

impl BusIO for BusDevice {
    fn read<T>(&self, offset: Address) -> BusResult<T>
    where
        T: BusOperable,
    {
        match self {
            BusDevice::Memory(mem) => mem.read(offset),
            BusDevice::Aclint(aclint) => aclint.read(offset),
        }
    }

    fn write<T>(&self, offset: Address, data: T) -> BusResult<()>
    where
        T: BusOperable,
    {
        match self {
            BusDevice::Memory(mem) => mem.write(offset, data),
            BusDevice::Aclint(aclint) => aclint.write(offset, data),
        }
    }

    fn rmw<T, F>(&self, offset: Address, f: F) -> BusResult<T>
    where
        T: BusOperable,
        F: FnOnce(T) -> T,
    {
        match self {
            BusDevice::Memory(mem) => mem.rmw(offset, f),
            BusDevice::Aclint(aclint) => aclint.rmw(offset, f),
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

    pub fn map<D>(&mut self, start: Address, size: u64, device: D) -> Result<()>
    where
        D: Into<BusDevice>,
    {
        let end = start + size;
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

    fn mapping_index(&self, location: Address) -> Option<usize> {
        self.mappings
            .partition_point(|mapping| mapping.range.start <= location)
            .checked_sub(1)
            .filter(|&i| self.mappings[i].contains(location))
    }
}

impl BusIO for Bus {
    fn read<T>(&self, offset: Address) -> BusResult<T>
    where
        T: BusOperable,
    {
        let index = self
            .mapping_index(offset)
            .ok_or(BusError::UnmappedLocation(offset))?;
        let mapping = &self.mappings[index];
        mapping.handler.read(offset - mapping.range.start)
    }

    fn write<T>(&self, offset: Address, data: T) -> BusResult<()>
    where
        T: BusOperable,
    {
        let index = self
            .mapping_index(offset)
            .ok_or(BusError::UnmappedLocation(offset))?;
        let mapping = &self.mappings[index];
        mapping.handler.write(offset - mapping.range.start, data)
    }

    fn rmw<T, F>(&self, offset: Address, f: F) -> BusResult<T>
    where
        T: BusOperable,
        F: FnOnce(T) -> T,
    {
        let index = self
            .mapping_index(offset)
            .ok_or(BusError::UnmappedLocation(offset))?;
        let mapping = &self.mappings[index];
        mapping.handler.rmw(offset - mapping.range.start, f)
    }
}

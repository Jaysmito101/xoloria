use std::sync::{Arc, RwLock};

use crate::Result;
use crate::bus::{BusError, BusIO, BusResult};

pub struct Memory {
    data: RwLock<Vec<u8>>,
}

impl Memory {
    pub fn new(size: usize) -> Result<Self> {
        let mut data = Vec::new();
        data.try_reserve_exact(size)
            .map_err(crate::Error::AllocationFailed)?;
        data.resize(size, 0);
        Ok(Self {
            data: RwLock::new(data),
        })
    }
}

impl BusIO for Memory {
    fn read<T>(&self, offset: crate::Address) -> BusResult<T>
    where
        T: crate::bus::BusOperable,
    {
        let data = self.data.read().map_err(|_| BusError::LockFailed)?;
        // read the bytes from the memory and copy them into a T
        let size = std::mem::size_of::<T>();
        if offset as usize + size > data.len() {
            return Err(BusError::AddressOverflow(offset, size));
        }
        let mut bytes = [0u8; 8];
        bytes[..size].copy_from_slice(&data[offset as usize..offset as usize + size]);
        Ok(unsafe { std::ptr::read_unaligned(bytes.as_ptr() as *const T) })
    }

    fn write<T>(&self, offset: crate::Address, data: T) -> BusResult<()>
    where
        T: crate::bus::BusOperable,
    {
        let mut mem_data = self.data.write().map_err(|_| BusError::LockFailed)?;
        let size = std::mem::size_of::<T>();
        if offset as usize + size > mem_data.len() {
            return Err(BusError::AddressOverflow(offset, size));
        }
        let bytes = unsafe {
            std::slice::from_raw_parts(&data as *const T as *const u8, std::mem::size_of::<T>())
        };
        mem_data[offset as usize..offset as usize + size].copy_from_slice(bytes);
        Ok(())
    }

    fn rmw<T, F>(&self, offset: crate::Address, f: F) -> BusResult<T>
    where
        T: crate::bus::BusOperable,
        F: FnOnce(T) -> T,
    {
        let mut mem_data = self.data.write().map_err(|_| BusError::LockFailed)?;
        let size = std::mem::size_of::<T>();
        if offset as usize + size > mem_data.len() {
            return Err(BusError::AddressOverflow(offset, size));
        }
        let mut bytes = [0u8; 8];
        bytes[..size].copy_from_slice(&mem_data[offset as usize..offset as usize + size]);
        let value = unsafe { std::ptr::read_unaligned(bytes.as_ptr() as *const T) };
        let new_value = f(value);
        let new_bytes = unsafe {
            std::slice::from_raw_parts(
                &new_value as *const T as *const u8,
                std::mem::size_of::<T>(),
            )
        };
        mem_data[offset as usize..offset as usize + size].copy_from_slice(new_bytes);
        Ok(value)
    }
}

impl From<Arc<Memory>> for crate::bus::BusDevice {
    fn from(val: Arc<Memory>) -> Self {
        crate::bus::BusDevice::Memory(val)
    }
}

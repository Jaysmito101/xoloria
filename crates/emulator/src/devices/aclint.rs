use std::{
    ops::Range,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};

use crate::{Address, BusIO, bus::BusError};

pub struct Aclint {
    harts: usize,
    mtime: AtomicU64,
    mtimecmp: [AtomicU64; 4096],
    mswi: [AtomicBool; 4096],
    sswi: [AtomicBool; 4096],
}

impl Aclint {
    const MSWI: Range<Address> = 0x0000..0x4000;
    const MTIMECMP: Range<Address> = 0x4000..0xbff8;
    const MTIME: Range<Address> = 0xbff8..0xc000;
    const SSWI: Range<Address> = 0xC000..0x10000;

    pub fn new(harts: usize) -> Self {
        Self {
            harts,
            mtime: AtomicU64::new(0),
            mtimecmp: [const { AtomicU64::new(0) }; 4096],
            mswi: [const { AtomicBool::new(false) }; 4096],
            sswi: [const { AtomicBool::new(false) }; 4096],
        }
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        0x10000
    }

    pub fn reset(&self) {
        for i in 0..4096 {
            self.mtimecmp[i].store(0, Ordering::SeqCst);
            self.mswi[i].store(false, Ordering::SeqCst);
            self.sswi[i].store(false, Ordering::SeqCst);
        }
        self.mtime.store(0, Ordering::SeqCst);
    }

    pub fn tick(&self) {
        self.mtime.fetch_add(1, Ordering::SeqCst);
    }

    pub fn hart_count(&self) -> usize {
        self.harts
    }

    pub fn mtime(&self) -> u64 {
        self.mtime.load(Ordering::SeqCst)
    }

    pub fn mtimecmp(&self, hart: usize) -> u64 {
        self.mtimecmp[hart].load(Ordering::SeqCst)
    }

    pub fn mtip(&self, hart: usize) -> bool {
        let mtime = self.mtime.load(Ordering::SeqCst);
        let mtimecmp = self.mtimecmp[hart].load(Ordering::SeqCst);
        mtime >= mtimecmp
    }

    pub fn msip(&self, hart: usize) -> bool {
        self.mswi[hart].load(Ordering::SeqCst)
    }

    pub fn ssip(&self, hart: usize) -> bool {
        self.sswi[hart].load(Ordering::SeqCst)
    }

    #[inline(always)]
    fn check_size_and_alignment<T>(offset: Address, expected_size: usize) -> Result<(), BusError>
    where
        T: crate::bus::BusOperable,
    {
        let size = std::mem::size_of::<T>();
        if size != expected_size {
            return Err(BusError::InvalidSize(offset, size));
        }
        if !offset.is_multiple_of(size as u64) {
            return Err(BusError::InvalidAlignment(offset, size));
        }
        Ok(())
    }
}

impl BusIO for Aclint {
    fn read<T>(&self, offset: crate::Address) -> crate::bus::BusResult<T>
    where
        T: crate::bus::BusOperable,
    {
        match offset {
            offset if Self::MSWI.contains(&offset) => {
                Self::check_size_and_alignment::<T>(offset, 4)?;
                let hart = (offset / 4) as usize;
                match self.mswi[hart].load(Ordering::Acquire) {
                    true => Ok(T::one()),
                    false => Ok(T::zero()),
                }
            }
            offset if Self::MTIMECMP.contains(&offset) => {
                Self::check_size_and_alignment::<T>(offset, 8)?;
                let hart = ((offset - Self::MTIMECMP.start) / 8) as usize;
                let value = self.mtimecmp[hart].load(Ordering::SeqCst);
                Ok(unsafe { std::mem::transmute_copy(&value) })
            }
            offset if Self::MTIME.contains(&offset) => {
                Self::check_size_and_alignment::<T>(offset, 8)?;
                let value = self.mtime.load(Ordering::SeqCst);
                Ok(unsafe { std::mem::transmute_copy(&value) })
            }
            offset if Self::SSWI.contains(&offset) => {
                Self::check_size_and_alignment::<T>(offset, 4)?;
                let hart = ((offset - Self::SSWI.start) / 4) as usize;
                match self.sswi[hart].load(Ordering::Acquire) {
                    true => Ok(T::one()),
                    false => Ok(T::zero()),
                }
            }
            _ => Err(BusError::UnmappedLocation(offset)),
        }
    }

    fn write<T>(&self, offset: crate::Address, data: T) -> crate::bus::BusResult<()>
    where
        T: crate::bus::BusOperable,
    {
        match offset {
            offset if Self::MSWI.contains(&offset) => {
                Self::check_size_and_alignment::<T>(offset, 4)?;
                let hart = (offset / 4) as usize;
                let value = data != T::zero();
                self.mswi[hart].store(value, Ordering::Release);
                Ok(())
            }
            offset if Self::MTIMECMP.contains(&offset) => {
                Self::check_size_and_alignment::<T>(offset, 8)?;
                let hart = ((offset - Self::MTIMECMP.start) / 8) as usize;
                let value = unsafe { std::mem::transmute_copy(&data) };
                self.mtimecmp[hart].store(value, Ordering::SeqCst);
                Ok(())
            }
            offset if Self::SSWI.contains(&offset) => {
                Self::check_size_and_alignment::<T>(offset, 4)?;
                let hart = ((offset - Self::SSWI.start) / 4) as usize;
                let value = data != T::zero();
                self.sswi[hart].store(value, Ordering::Release);
                Ok(())
            }
            _ => Err(BusError::UnmappedLocation(offset)),
        }
    }

    fn rmw<T, F>(&self, _offset: crate::Address, _f: F) -> crate::bus::BusResult<T>
    where
        T: crate::bus::BusOperable,
        F: FnOnce(T) -> T,
    {
        unimplemented!("RMW operation is not supported for ACLINT");
    }
}

impl From<Arc<Aclint>> for crate::bus::BusDevice {
    fn from(aclint: Arc<Aclint>) -> Self {
        Self::Aclint(aclint)
    }
}

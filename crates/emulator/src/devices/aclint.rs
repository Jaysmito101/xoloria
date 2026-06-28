use crate::BusIO;

pub struct Aclint {}

impl Aclint {
    pub fn new(harts: usize) -> Self {
        Self {}
    }
}

impl BusIO for Aclint {
    fn read<T>(&self, offset: crate::Address) -> crate::bus::BusResult<T>
    where
        T: crate::bus::BusOperable,
    {
        todo!()
    }

    fn write<T>(&self, offset: crate::Address, data: T) -> crate::bus::BusResult<()>
    where
        T: crate::bus::BusOperable,
    {
        todo!()
    }

    fn rmw<T, F>(&self, offset: crate::Address, f: F) -> crate::bus::BusResult<T>
    where
        T: crate::bus::BusOperable,
        F: FnOnce(T) -> T,
    {
        todo!()
    }
}

impl From<Aclint> for crate::bus::BusDevice {
    fn from(aclint: Aclint) -> Self {
        Self::Aclint(aclint)
    }
}

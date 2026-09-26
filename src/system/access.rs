//! To keep track of access to objects a special object is used.

use std::any::TypeId;

#[derive(Default)]
pub struct Access {
    reads: Vec<TypeId>,
    writes: Vec<TypeId>,
}

impl Access {
    pub fn read<T: 'static>(&mut self) -> Result<(), AccessError> {
        let id = TypeId::of::<T>();

        if self.writes.contains(&id) {
            return Err(AccessError::ReadWriteConflict);
        }

        self.reads.push(id);
        Ok(())
    }

    pub fn write<T: 'static>(&mut self) -> Result<(), AccessError> {
        let id = TypeId::of::<T>();

        if self.reads.contains(&id) || self.writes.contains(&id) {
            return Err(AccessError::WriteWriteConflict);
        }

        self.writes.push(id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum AccessError {
    ReadWriteConflict,
    WriteWriteConflict,
}

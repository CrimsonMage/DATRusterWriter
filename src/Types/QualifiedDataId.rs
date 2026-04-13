use std::marker::PhantomData;
use std::hash::{Hash, Hasher};

use crate::{
    DatCollection::DatCollection,
    Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj, IPackable::IPackable, IUnpackable::IUnpackable},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct QualifiedDataId<T> {
    pub data_id: u32,
    _marker: PhantomData<T>,
}

impl<T> QualifiedDataId<T> {
    pub fn new(data_id: u32) -> Self {
        Self { data_id, _marker: PhantomData }
    }

    pub fn is_null(&self) -> bool {
        self.data_id == 0
    }
}

impl<T> PartialEq for QualifiedDataId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data_id == other.data_id
    }
}

impl<T> Eq for QualifiedDataId<T> {}

impl<T> Hash for QualifiedDataId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data_id.hash(state);
    }
}

impl<T> QualifiedDataId<T>
where
    T: IDBObj + Default,
{
    pub fn get(&self, dat_collection: &DatCollection) -> std::io::Result<Option<T>> {
        dat_collection.get::<T>(self.data_id)
    }

    pub fn try_get(&self, dat_collection: &DatCollection) -> std::io::Result<Option<T>> {
        dat_collection.try_get::<T>(self.data_id)
    }
}

impl<T> IUnpackable for QualifiedDataId<T> {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.data_id = reader.read_u32();
        true
    }
}

impl<T> IPackable for QualifiedDataId<T> {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.data_id);
        true
    }
}

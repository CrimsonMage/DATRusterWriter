use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{
    DatCollection::DatCollection,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj,
        IPackable::IPackable, IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Copy, Default)]
pub struct PackedQualifiedDataId<T> {
    pub data_id: u32,
    _marker: PhantomData<T>,
}

impl<T> PackedQualifiedDataId<T> {
    pub fn new(data_id: u32) -> Self {
        Self {
            data_id,
            _marker: PhantomData,
        }
    }
}

impl<T> PartialEq for PackedQualifiedDataId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data_id == other.data_id
    }
}

impl<T> Eq for PackedQualifiedDataId<T> {}

impl<T> Hash for PackedQualifiedDataId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data_id.hash(state);
    }
}

impl<T> PackedQualifiedDataId<T>
where
    T: IDBObj + Default,
{
    pub fn get(&self, dat_collection: &DatCollection) -> std::io::Result<Option<T>> {
        dat_collection.get::<T>(self.data_id)
    }
}

impl<T> IUnpackable for PackedQualifiedDataId<T>
where
    T: IDBObj,
{
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.data_id = reader.read_data_id_of_known_type(T::db_obj_type_attr().mask_id);
        true
    }
}

impl<T> IPackable for PackedQualifiedDataId<T>
where
    T: IDBObj,
{
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_data_id_of_known_type(self.data_id, T::db_obj_type_attr().mask_id);
        true
    }
}

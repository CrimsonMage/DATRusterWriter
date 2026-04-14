use std::collections::BTreeMap;

use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

pub trait PHashTableKey: Copy + Ord {
    fn read_key(reader: &mut DatBinReader<'_>) -> Self;
    fn write_key(&self, writer: &mut DatBinWriter<'_>);
}

impl PHashTableKey for u32 {
    fn read_key(reader: &mut DatBinReader<'_>) -> Self {
        reader.read_u32()
    }

    fn write_key(&self, writer: &mut DatBinWriter<'_>) {
        writer.write_u32(*self);
    }
}

impl PHashTableKey for crate::Generated::Enums::EquipmentSet::EquipmentSet {
    fn read_key(reader: &mut DatBinReader<'_>) -> Self {
        Self::from(reader.read_u32())
    }

    fn write_key(&self, writer: &mut DatBinWriter<'_>) {
        writer.write_u32((*self).into());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PHashTable<K, V> {
    pub bucket_size: u16,
    pub entries: BTreeMap<K, V>,
}

impl<K, V> Default for PHashTable<K, V> {
    fn default() -> Self {
        Self {
            bucket_size: 256,
            entries: BTreeMap::new(),
        }
    }
}

impl<K, V> PHashTable<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        self.entries.insert(key, value)
    }

    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: Ord,
    {
        self.entries.get(key)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl<K, V> IUnpackable for PHashTable<K, V>
where
    K: PHashTableKey,
    V: IUnpackable + Default,
{
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let count = reader.read_u16() as usize;
        self.bucket_size = reader.read_u16();
        self.entries.clear();
        for _ in 0..count {
            let key = K::read_key(reader);
            let value = reader.read_item::<V>();
            self.entries.insert(key, value);
        }
        true
    }
}

impl<K, V> IPackable for PHashTable<K, V>
where
    K: PHashTableKey,
    V: IPackable,
{
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u16(self.entries.len() as u16);
        writer.write_u16(self.bucket_size);
        for (key, value) in &self.entries {
            key.write_key(writer);
            writer.write_item(value);
        }
        true
    }
}

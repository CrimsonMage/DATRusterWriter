use std::collections::BTreeMap;

use crate::{
    Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable},
    Types::HashTable::HashTableKey,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackableHashTable<K, V> {
    pub bucket_size: u16,
    pub entries: BTreeMap<K, V>,
}

impl<K, V> Default for PackableHashTable<K, V> {
    fn default() -> Self {
        Self { bucket_size: 32, entries: BTreeMap::new() }
    }
}

impl<K, V> PackableHashTable<K, V> {
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

    pub fn len(&self) -> usize { self.entries.len() }
}

impl<K, V> IUnpackable for PackableHashTable<K, V>
where
    K: HashTableKey,
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

impl<K, V> IPackable for PackableHashTable<K, V>
where
    K: HashTableKey,
    V: IPackable,
{
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u16(self.entries.len() as u16);
        writer.write_u16(self.bucket_size);
        let bucket_size = self.bucket_size.max(1) as u64;
        let mut items: Vec<_> = self.entries.iter().collect();
        items.sort_by_key(|(key, _)| key.hash_key() % bucket_size);
        for (key, value) in items {
            key.write_key(writer);
            writer.write_item(value);
        }
        true
    }
}

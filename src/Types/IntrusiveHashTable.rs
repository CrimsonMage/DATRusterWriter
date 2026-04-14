use std::collections::BTreeMap;

use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

use super::AutoGrowHashTable::{GenericHashKey, GenericHashValue};

const BUCKET_SIZES: [u32; 23] = [
    11, 23, 47, 89, 191, 383, 761, 1531, 3067, 6143, 12281, 24571, 49139, 98299, 196597, 393209,
    786431, 1572853, 3145721, 6291449, 12582893, 25165813, 50331599,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntrusiveHashTable<K, V> {
    pub bucket_size_index: u8,
    pub entries: BTreeMap<K, V>,
}

impl<K, V> Default for IntrusiveHashTable<K, V> {
    fn default() -> Self {
        Self {
            bucket_size_index: 1,
            entries: BTreeMap::new(),
        }
    }
}

impl<K, V> IntrusiveHashTable<K, V> {
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

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, K, V> {
        self.entries.iter()
    }
}

impl<K, V> IUnpackable for IntrusiveHashTable<K, V>
where
    K: GenericHashKey,
    V: GenericHashValue,
{
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.bucket_size_index = reader.read_byte();
        let count = reader.read_compressed_uint() as usize;
        self.entries.clear();

        for _ in 0..count {
            let key = K::read_value(reader);
            let value = V::read_value(reader);
            self.entries.insert(key, value);
        }

        true
    }
}

impl<K, V> IPackable for IntrusiveHashTable<K, V>
where
    K: GenericHashKey,
    V: GenericHashValue,
{
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(self.bucket_size_index);
        writer.write_compressed_uint(self.entries.len() as u32);

        let bucket_size = BUCKET_SIZES[self.bucket_size_index as usize] as u64;
        let mut items: Vec<_> = self
            .entries
            .iter()
            .map(|(key, value)| ((*key).hash_key() % bucket_size, *key, value))
            .collect();
        items.sort_by_key(|(hash, _, _)| *hash);

        for (_, key, value) in items {
            key.write_value(writer);
            value.write_value(writer);
        }

        true
    }
}

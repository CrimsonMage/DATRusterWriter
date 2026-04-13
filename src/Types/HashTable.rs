use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::QualifiedDataId::QualifiedDataId,
};

const BUCKET_SIZES: [u32; 23] = [
    11, 23, 47, 89, 191, 383, 761, 1531, 3067, 6143, 12281, 24571, 49139, 98299, 196597, 393209,
    786431, 1572853, 3145721, 6291449, 12582893, 25165813, 50331599,
];

pub trait HashTableKey: Copy + Ord {
    fn read_key(reader: &mut DatBinReader<'_>) -> Self;
    fn write_key(&self, writer: &mut DatBinWriter<'_>);
    fn hash_key(&self) -> u64;
}

impl HashTableKey for i32 {
    fn read_key(reader: &mut DatBinReader<'_>) -> Self {
        reader.read_i32()
    }
    fn write_key(&self, writer: &mut DatBinWriter<'_>) {
        writer.write_i32(*self);
    }
    fn hash_key(&self) -> u64 {
        *self as u32 as u64
    }
}

impl HashTableKey for u32 {
    fn read_key(reader: &mut DatBinReader<'_>) -> Self {
        reader.read_u32()
    }
    fn write_key(&self, writer: &mut DatBinWriter<'_>) {
        writer.write_u32(*self);
    }
    fn hash_key(&self) -> u64 {
        *self as u64
    }
}

impl<T> HashTableKey for QualifiedDataId<T> {
    fn read_key(reader: &mut DatBinReader<'_>) -> Self {
        QualifiedDataId::new(reader.read_u32())
    }
    fn write_key(&self, writer: &mut DatBinWriter<'_>) {
        writer.write_u32(self.data_id);
    }
    fn hash_key(&self) -> u64 {
        self.data_id as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashTable<K, V> {
    pub bucket_size_index: u8,
    pub entries: std::collections::BTreeMap<K, V>,
}

impl<K, V> Default for HashTable<K, V> {
    fn default() -> Self {
        Self {
            bucket_size_index: 1,
            entries: std::collections::BTreeMap::new(),
        }
    }
}

impl<K, V> HashTable<K, V> {
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

impl<K, V> IUnpackable for HashTable<K, V>
where
    K: HashTableKey,
    V: IUnpackable + Default,
{
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.bucket_size_index = reader.read_byte();
        let count = reader.read_compressed_uint() as usize;
        self.entries.clear();
        for _ in 0..count {
            let key = K::read_key(reader);
            let value = reader.read_item::<V>();
            self.entries.insert(key, value);
        }
        true
    }
}

impl<K, V> IPackable for HashTable<K, V>
where
    K: HashTableKey,
    V: IPackable,
{
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(self.bucket_size_index);
        writer.write_compressed_uint(self.entries.len() as u32);
        let bucket_size = BUCKET_SIZES[self.bucket_size_index as usize] as u64;
        let mut items: Vec<_> = self.entries.iter().collect();
        items.sort_by_key(|(key, _)| key.hash_key() % bucket_size);
        for (key, value) in items {
            key.write_key(writer);
            writer.write_item(value);
        }
        true
    }
}

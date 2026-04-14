use crate::{
    Generated::Enums::{SkillId::SkillId, UIStateId::UIStateId},
    Lib::{
        HashTableHelpers::{BUCKET_SIZES, HashKeyable},
        IO::{
            DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
            IUnpackable::IUnpackable,
        },
    },
    Types::QualifiedDataId::QualifiedDataId,
};

pub trait HashTableItem: Sized {
    fn read_item(reader: &mut DatBinReader<'_>) -> Self;
    fn write_item(&self, writer: &mut DatBinWriter<'_>);
}

pub trait HashTableKey: Ord {
    fn read_key(reader: &mut DatBinReader<'_>) -> Self;
    fn write_key(&self, writer: &mut DatBinWriter<'_>);
    fn hash_key(&self) -> u64;
}

macro_rules! impl_hash_item_numeric {
    ($ty:ty, $read:ident, $write:ident, $hash:expr) => {
        impl HashTableItem for $ty {
            fn read_item(reader: &mut DatBinReader<'_>) -> Self {
                reader.$read()
            }
            fn write_item(&self, writer: &mut DatBinWriter<'_>) {
                writer.$write(*self);
            }
        }

        impl HashTableKey for $ty {
            fn read_key(reader: &mut DatBinReader<'_>) -> Self {
                <Self as HashTableItem>::read_item(reader)
            }
            fn write_key(&self, writer: &mut DatBinWriter<'_>) {
                <Self as HashTableItem>::write_item(self, writer);
            }
            fn hash_key(&self) -> u64 {
                $hash(*self)
            }
        }
    };
}

impl_hash_item_numeric!(i8, read_sbyte, write_sbyte, |v: i8| v as u8 as u64);
impl_hash_item_numeric!(u8, read_byte, write_byte, |v: u8| v as u64);
impl_hash_item_numeric!(i16, read_i16, write_i16, |v: i16| v as u16 as u64);
impl_hash_item_numeric!(u16, read_u16, write_u16, |v: u16| v as u64);
impl_hash_item_numeric!(i32, read_i32, write_i32, |v: i32| v as u32 as u64);
impl_hash_item_numeric!(u32, read_u32, write_u32, |v: u32| v as u64);

impl HashTableItem for bool {
    fn read_item(reader: &mut DatBinReader<'_>) -> Self {
        reader.read_bool(4)
    }
    fn write_item(&self, writer: &mut DatBinWriter<'_>) {
        writer.write_bool(*self, 4);
    }
}

impl HashTableItem for f32 {
    fn read_item(reader: &mut DatBinReader<'_>) -> Self {
        reader.read_single()
    }
    fn write_item(&self, writer: &mut DatBinWriter<'_>) {
        writer.write_single(*self);
    }
}

impl HashTableItem for f64 {
    fn read_item(reader: &mut DatBinReader<'_>) -> Self {
        reader.read_double()
    }
    fn write_item(&self, writer: &mut DatBinWriter<'_>) {
        writer.write_double(*self);
    }
}

impl HashTableItem for String {
    fn read_item(reader: &mut DatBinReader<'_>) -> Self {
        reader.read_string16_l()
    }
    fn write_item(&self, writer: &mut DatBinWriter<'_>) {
        writer.write_string16_l(self);
    }
}

impl HashTableKey for String {
    fn read_key(reader: &mut DatBinReader<'_>) -> Self {
        <Self as HashTableItem>::read_item(reader)
    }
    fn write_key(&self, writer: &mut DatBinWriter<'_>) {
        <Self as HashTableItem>::write_item(self, writer);
    }
    fn hash_key(&self) -> u64 {
        <Self as HashKeyable>::hash_key(self)
    }
}

impl HashTableItem for SkillId {
    fn read_item(reader: &mut DatBinReader<'_>) -> Self {
        SkillId::from(reader.read_i32())
    }
    fn write_item(&self, writer: &mut DatBinWriter<'_>) {
        writer.write_i32((*self).into());
    }
}

impl HashTableKey for SkillId {
    fn read_key(reader: &mut DatBinReader<'_>) -> Self {
        <Self as HashTableItem>::read_item(reader)
    }
    fn write_key(&self, writer: &mut DatBinWriter<'_>) {
        <Self as HashTableItem>::write_item(self, writer);
    }
    fn hash_key(&self) -> u64 {
        self.0 as u32 as u64
    }
}

impl HashTableItem for UIStateId {
    fn read_item(reader: &mut DatBinReader<'_>) -> Self {
        UIStateId::from(reader.read_u32())
    }
    fn write_item(&self, writer: &mut DatBinWriter<'_>) {
        writer.write_u32((*self).into());
    }
}

impl HashTableKey for UIStateId {
    fn read_key(reader: &mut DatBinReader<'_>) -> Self {
        <Self as HashTableItem>::read_item(reader)
    }
    fn write_key(&self, writer: &mut DatBinWriter<'_>) {
        <Self as HashTableItem>::write_item(self, writer);
    }
    fn hash_key(&self) -> u64 {
        self.0 as u64
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

impl<T> HashTableItem for T
where
    T: IUnpackable + IPackable + Default,
{
    fn read_item(reader: &mut DatBinReader<'_>) -> Self {
        reader.read_item::<T>()
    }
    fn write_item(&self, writer: &mut DatBinWriter<'_>) {
        writer.write_item(self);
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
    V: HashTableItem,
{
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.bucket_size_index = reader.read_byte();
        let count = reader.read_compressed_uint() as usize;
        self.entries.clear();
        for _ in 0..count {
            let key = K::read_key(reader);
            let value = V::read_item(reader);
            self.entries.insert(key, value);
        }
        true
    }
}

impl<K, V> IPackable for HashTable<K, V>
where
    K: HashTableKey,
    V: HashTableItem,
{
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(self.bucket_size_index);
        writer.write_compressed_uint(self.entries.len() as u32);
        let bucket_size = BUCKET_SIZES[self.bucket_size_index as usize] as u64;
        let mut items: Vec<_> = self.entries.iter().collect();
        items.sort_by_key(|(key, _)| key.hash_key() % bucket_size);
        for (key, value) in items {
            key.write_key(writer);
            value.write_item(writer);
        }
        true
    }
}

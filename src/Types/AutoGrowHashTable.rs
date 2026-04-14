use std::{any::TypeId, collections::BTreeMap};

use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::PStringBase::PStringBase,
};

const BUCKET_SIZES: [u32; 23] = [
    11, 23, 47, 89, 191, 383, 761, 1531, 3067, 6143, 12281, 24571, 49139, 98299, 196597, 393209,
    786431, 1572853, 3145721, 6291449, 12582893, 25165813, 50331599,
];

pub trait GenericHashValue: Sized {
    fn read_value(reader: &mut DatBinReader<'_>) -> Self;
    fn write_value(&self, writer: &mut DatBinWriter<'_>);
}

impl<T> GenericHashValue for T
where
    T: Copy + Ord + 'static,
{
    fn read_value(reader: &mut DatBinReader<'_>) -> Self {
        reader.read_generic::<T>()
    }

    fn write_value(&self, writer: &mut DatBinWriter<'_>) {
        writer.write_generic(*self);
    }
}

impl GenericHashValue for PStringBase<u8> {
    fn read_value(reader: &mut DatBinReader<'_>) -> Self {
        reader.read_item::<Self>()
    }

    fn write_value(&self, writer: &mut DatBinWriter<'_>) {
        writer.write_item(self);
    }
}

fn get_bucket_size_index(count: usize, auto_grow: bool) -> u8 {
    if count == 0 {
        return 1;
    }

    let target = if auto_grow {
        count.saturating_mul(2)
    } else {
        count
    };
    BUCKET_SIZES
        .iter()
        .position(|bucket_size| *bucket_size as usize >= target)
        .unwrap_or(BUCKET_SIZES.len() - 1) as u8
}

pub trait GenericHashKey: GenericHashValue + Copy + Ord + 'static {
    fn hash_key(self) -> u64;
}

impl<T> GenericHashKey for T
where
    T: Copy + Ord + 'static,
{
    fn hash_key(self) -> u64 {
        let ty = TypeId::of::<T>();

        macro_rules! match_copy {
            ($kind:ty) => {
                if ty == TypeId::of::<$kind>() {
                    let value = unsafe { std::mem::transmute_copy::<T, $kind>(&self) };
                    return value as u64;
                }
            };
        }

        match_copy!(u8);
        match_copy!(u16);
        match_copy!(u32);
        match_copy!(u64);
        match_copy!(i8);
        match_copy!(i16);
        match_copy!(i32);
        match_copy!(i64);

        panic!("Type is not supported by GenericHashKey yet");
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutoGrowHashTable<K, V> {
    pub bucket_size_index: u8,
    pub entries: BTreeMap<K, V>,
}

impl<K, V> Default for AutoGrowHashTable<K, V> {
    fn default() -> Self {
        Self {
            bucket_size_index: 1,
            entries: BTreeMap::new(),
        }
    }
}

impl<K, V> AutoGrowHashTable<K, V> {
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

impl<K, V> IUnpackable for AutoGrowHashTable<K, V>
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

impl<K, V> IPackable for AutoGrowHashTable<K, V>
where
    K: GenericHashKey,
    V: GenericHashValue,
{
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let bucket_size_index = get_bucket_size_index(self.entries.len(), true);
        writer.write_byte(bucket_size_index);
        writer.write_compressed_uint(self.entries.len() as u32);

        let bucket_size = BUCKET_SIZES[bucket_size_index as usize] as u64;
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

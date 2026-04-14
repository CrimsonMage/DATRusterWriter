use crate::Types::{QualifiedDataId::QualifiedDataId, StringBase::string_hash};

pub const BUCKET_SIZES: [u32; 23] = [
    11, 23, 47, 89, 191, 383, 761, 1531, 3067, 6143, 12281, 24571, 49139, 98299, 196597, 393209,
    786431, 1572853, 3145721, 6291449, 12582893, 25165813, 50331599,
];

pub fn get_bucket_size(entry_count: usize, is_auto_grow: bool) -> u32 {
    if is_auto_grow {
        for size in BUCKET_SIZES {
            if entry_count + 1 > 2 * size as usize {
                continue;
            }
            return size;
        }
    } else {
        for size in BUCKET_SIZES {
            if size as usize >= entry_count {
                return size;
            }
        }
    }

    *BUCKET_SIZES.last().unwrap()
}

pub fn get_bucket_size_index(entry_count: usize, is_auto_grow: bool) -> u8 {
    let bucket_size = get_bucket_size(entry_count, is_auto_grow);
    BUCKET_SIZES
        .iter()
        .position(|size| *size == bucket_size)
        .unwrap_or(BUCKET_SIZES.len() - 1) as u8
}

pub trait HashKeyable {
    fn hash_key(&self) -> u64;
}

macro_rules! impl_numeric_hash_keyable {
    ($($ty:ty),* $(,)?) => {
        $(
            impl HashKeyable for $ty {
                fn hash_key(&self) -> u64 {
                    *self as u64
                }
            }
        )*
    };
}

impl_numeric_hash_keyable!(u8, u16, u32, u64, i8, i16, i32, i64);

impl HashKeyable for String {
    fn hash_key(&self) -> u64 {
        string_hash(self) as u32 as u64
    }
}

impl<T> HashKeyable for QualifiedDataId<T> {
    fn hash_key(&self) -> u64 {
        self.data_id as u64
    }
}

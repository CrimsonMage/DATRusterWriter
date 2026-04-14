use std::{any::TypeId, marker::PhantomData};

use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};
use crate::Types::StringBase::StringBase;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PStringBase<T> {
    pub value: String,
    _marker: PhantomData<T>,
}

impl<T> PStringBase<T> {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            _marker: PhantomData,
        }
    }
}

impl<T> From<&str> for PStringBase<T> {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl<T> From<String> for PStringBase<T> {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl<T> StringBase for PStringBase<T> {
    fn value(&self) -> &str {
        &self.value
    }
}

impl<T: 'static> IUnpackable for PStringBase<T> {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let length = reader.read_compressed_uint() as usize;

        if TypeId::of::<T>() == TypeId::of::<u8>() {
            self.value = String::from_utf8_lossy(&reader.read_bytes(length)).to_string();
            return true;
        }

        if TypeId::of::<T>() == TypeId::of::<u16>() {
            let mut value = String::with_capacity(length);
            for _ in 0..length {
                value.push(char::from_u32(reader.read_u16() as u32).unwrap_or('\u{FFFD}'));
            }
            self.value = value;
            return true;
        }

        false
    }
}

impl<T: 'static> IPackable for PStringBase<T> {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            let bytes = self.value.as_bytes();
            writer.write_compressed_uint(bytes.len() as u32);
            writer.write_bytes(bytes, bytes.len());
            return true;
        }

        if TypeId::of::<T>() == TypeId::of::<u16>() {
            writer.write_compressed_uint(self.value.chars().count() as u32);
            for ch in self.value.chars() {
                writer.write_u16(ch as u16);
            }
            return true;
        }

        false
    }
}

use std::{any::TypeId, marker::PhantomData};

use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{AC1LegacyString::AC1LegacyString, StringBase::StringBase},
};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AC1LegacyPStringBase<T> {
    pub value: String,
    _marker: PhantomData<T>,
}

impl<T> From<&str> for AC1LegacyPStringBase<T> {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
            _marker: PhantomData,
        }
    }
}

impl<T> From<String> for AC1LegacyPStringBase<T> {
    fn from(value: String) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

impl<T> StringBase for AC1LegacyPStringBase<T> {
    fn value(&self) -> &str {
        &self.value
    }
}

impl<T: 'static> IUnpackable for AC1LegacyPStringBase<T> {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let length_marker = reader.read_u16();
        let length = if length_marker == 0xFFFF {
            reader.read_u32() as usize
        } else {
            length_marker as usize
        };

        if TypeId::of::<T>() == TypeId::of::<u8>() {
            let bytes = reader.read_bytes(length);
            self.value = String::from_utf8_lossy(&bytes).to_string();
            reader.align(4);
            return true;
        }

        if TypeId::of::<T>() == TypeId::of::<u16>() {
            let mut value = String::with_capacity(length);
            for _ in 0..length {
                value.push(char::from_u32(reader.read_u16() as u32).unwrap_or('\u{FFFD}'));
            }
            self.value = value;
            reader.align(4);
            return true;
        }

        let mut inner = AC1LegacyString::default();
        inner.value = self.value.clone();
        let result = inner.unpack(reader);
        self.value = inner.value;
        result
    }
}

impl<T: 'static> IPackable for AC1LegacyPStringBase<T> {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            let bytes = self.value.as_bytes();
            if bytes.len() >= 0xFFFF {
                writer.write_u16(0xFFFF);
                writer.write_u32(bytes.len() as u32);
            } else {
                writer.write_u16(bytes.len() as u16);
            }
            writer.write_bytes(bytes, bytes.len());
            writer.align(4);
            return true;
        }

        if TypeId::of::<T>() == TypeId::of::<u16>() {
            let length = self.value.chars().count();
            if length >= 0xFFFF {
                writer.write_u16(0xFFFF);
                writer.write_u32(length as u32);
            } else {
                writer.write_u16(length as u16);
            }
            for ch in self.value.chars() {
                writer.write_u16(ch as u16);
            }
            writer.align(4);
            return true;
        }

        AC1LegacyString {
            value: self.value.clone(),
        }
        .pack(writer)
    }
}

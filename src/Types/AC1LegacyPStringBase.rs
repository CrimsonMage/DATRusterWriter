use std::marker::PhantomData;

use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::AC1LegacyString::AC1LegacyString,
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

impl<T> IUnpackable for AC1LegacyPStringBase<T> {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let mut inner = AC1LegacyString::default();
        let result = inner.unpack(reader);
        self.value = inner.value;
        result
    }
}

impl<T> IPackable for AC1LegacyPStringBase<T> {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        AC1LegacyString {
            value: self.value.clone(),
        }
        .pack(writer)
    }
}

use crate::{
    Generated::Enums::{DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DBObjBase {
    pub id: u32,
    pub data_category: u32,
    pub header_flags: DBObjHeaderFlags,
}

pub trait DBObj: crate::Lib::IO::IDBObj::IDBObj {
    fn header_flags(&self) -> DBObjHeaderFlags;
    fn db_obj_type(&self) -> DBObjType;
    fn id(&self) -> u32;
    fn set_id(&mut self, id: u32);
    fn data_category(&self) -> u32;
    fn set_data_category(&mut self, data_category: u32);
}

impl IUnpackable for DBObjBase {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.unpack_with_flags(reader, self.header_flags)
    }
}

impl IPackable for DBObjBase {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        self.pack_with_flags(writer, self.header_flags)
    }
}

impl DBObjBase {
    pub fn with_header_flags(header_flags: DBObjHeaderFlags) -> Self {
        Self {
            header_flags,
            ..Default::default()
        }
    }

    pub fn unpack_with_flags(
        &mut self,
        reader: &mut DatBinReader<'_>,
        header_flags: DBObjHeaderFlags,
    ) -> bool {
        self.header_flags = header_flags;
        if header_flags.contains(DBObjHeaderFlags::HasId) {
            self.id = reader.read_u32();
        }
        if header_flags.contains(DBObjHeaderFlags::HasDataCategory) {
            self.data_category = reader.read_u32();
        }
        true
    }

    pub fn pack_with_flags(
        &self,
        writer: &mut DatBinWriter<'_>,
        header_flags: DBObjHeaderFlags,
    ) -> bool {
        if header_flags.contains(DBObjHeaderFlags::HasId) {
            writer.write_u32(self.id);
        }
        if header_flags.contains(DBObjHeaderFlags::HasDataCategory) {
            writer.write_u32(self.data_category);
        }
        true
    }
}

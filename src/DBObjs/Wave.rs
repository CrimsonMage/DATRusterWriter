use std::any::Any;

use crate::{
    Generated::Enums::{DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType},
    Lib::{Attributes::DBObjTypeAttribute::DBObjTypeAttribute, IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj, IPackable::IPackable, IUnpackable::IUnpackable}},
    Types::DBObj::{DBObj, DBObjBase},
};

pub const WAVE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute { rust_type_name: "Wave", dat_file_type: DatFileType::Portal, db_obj_type: DBObjType::Wave, header_flags: DBObjHeaderFlags::HasId, first_id: 0x0A000000, last_id: 0x0A00FFFF, mask_id: 0x0A000000 };

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Wave {
    pub base: DBObjBase,
    pub header: Vec<u8>,
    pub data: Vec<u8>,
}

impl DBObj for Wave {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::Wave }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for Wave {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        let header_size = reader.read_i32().max(0) as usize;
        let data_size = reader.read_i32().max(0) as usize;
        self.header = reader.read_bytes(header_size);
        self.data = reader.read_bytes(data_size);
        true
    }
}

impl IPackable for Wave {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_i32(self.header.len() as i32);
        writer.write_i32(self.data.len() as i32);
        writer.write_bytes(&self.header, self.header.len());
        writer.write_bytes(&self.data, self.data.len());
        true
    }
}

impl IDBObj for Wave {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self: Sized { &WAVE_ATTR }
    fn db_obj_type(&self) -> DBObjType { DBObjType::Wave }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn as_any(&self) -> &dyn Any { self }
}

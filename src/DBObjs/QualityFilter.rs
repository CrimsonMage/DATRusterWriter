use std::any::Any;

use crate::{
    Generated::Enums::{
        DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
    },
    Lib::{
        Attributes::DBObjTypeAttribute::DBObjTypeAttribute,
        IO::{
            DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj,
            IPackable::IPackable, IUnpackable::IUnpackable,
        },
    },
    Types::DBObj::{DBObj, DBObjBase},
};

pub const QUALITY_FILTER_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "QualityFilter",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::QualityFilter,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x0E010000,
    last_id: 0x0E01FFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct QualityFilter {
    pub base: DBObjBase,
    pub int_stat_filter: Vec<u32>,
    pub int64_stat_filter: Vec<u32>,
    pub bool_stat_filter: Vec<u32>,
    pub float_stat_filter: Vec<u32>,
    pub data_id_stat_filter: Vec<u32>,
    pub instance_id_stat_filter: Vec<u32>,
    pub string_stat_filter: Vec<u32>,
    pub position_stat_filter: Vec<u32>,
    pub attribute_stat_filter: Vec<u32>,
    pub attribute2nd_stat_filter: Vec<u32>,
    pub skill_stat_filter: Vec<u32>,
}

fn read_u32_vec(reader: &mut DatBinReader<'_>, count: usize) -> Vec<u32> {
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        values.push(reader.read_u32());
    }
    values
}

fn write_u32_vec(writer: &mut DatBinWriter<'_>, values: &[u32]) {
    for value in values {
        writer.write_u32(*value);
    }
}

impl DBObj for QualityFilter {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::QualityFilter }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for QualityFilter {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        let num_ints = reader.read_u32() as usize;
        let num_int64 = reader.read_u32() as usize;
        let num_bools = reader.read_u32() as usize;
        let num_floats = reader.read_u32() as usize;
        let num_data_ids = reader.read_u32() as usize;
        let num_instance_ids = reader.read_u32() as usize;
        let num_strings = reader.read_u32() as usize;
        let num_positions = reader.read_u32() as usize;

        self.int_stat_filter = read_u32_vec(reader, num_ints);
        self.int64_stat_filter = read_u32_vec(reader, num_int64);
        self.bool_stat_filter = read_u32_vec(reader, num_bools);
        self.float_stat_filter = read_u32_vec(reader, num_floats);
        self.data_id_stat_filter = read_u32_vec(reader, num_data_ids);
        self.instance_id_stat_filter = read_u32_vec(reader, num_instance_ids);
        self.string_stat_filter = read_u32_vec(reader, num_strings);
        self.position_stat_filter = read_u32_vec(reader, num_positions);

        let num_attributes = reader.read_u32() as usize;
        let num_attribute2nds = reader.read_u32() as usize;
        let num_skills = reader.read_u32() as usize;
        self.attribute_stat_filter = read_u32_vec(reader, num_attributes);
        self.attribute2nd_stat_filter = read_u32_vec(reader, num_attribute2nds);
        self.skill_stat_filter = read_u32_vec(reader, num_skills);
        true
    }
}

impl IPackable for QualityFilter {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.int_stat_filter.len() as u32);
        writer.write_u32(self.int64_stat_filter.len() as u32);
        writer.write_u32(self.bool_stat_filter.len() as u32);
        writer.write_u32(self.float_stat_filter.len() as u32);
        writer.write_u32(self.data_id_stat_filter.len() as u32);
        writer.write_u32(self.instance_id_stat_filter.len() as u32);
        writer.write_u32(self.string_stat_filter.len() as u32);
        writer.write_u32(self.position_stat_filter.len() as u32);
        write_u32_vec(writer, &self.int_stat_filter);
        write_u32_vec(writer, &self.int64_stat_filter);
        write_u32_vec(writer, &self.bool_stat_filter);
        write_u32_vec(writer, &self.float_stat_filter);
        write_u32_vec(writer, &self.data_id_stat_filter);
        write_u32_vec(writer, &self.instance_id_stat_filter);
        write_u32_vec(writer, &self.string_stat_filter);
        write_u32_vec(writer, &self.position_stat_filter);
        writer.write_u32(self.attribute_stat_filter.len() as u32);
        writer.write_u32(self.attribute2nd_stat_filter.len() as u32);
        writer.write_u32(self.skill_stat_filter.len() as u32);
        write_u32_vec(writer, &self.attribute_stat_filter);
        write_u32_vec(writer, &self.attribute2nd_stat_filter);
        write_u32_vec(writer, &self.skill_stat_filter);
        true
    }
}

impl IDBObj for QualityFilter {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self: Sized { &QUALITY_FILTER_ATTR }
    fn db_obj_type(&self) -> DBObjType { DBObjType::QualityFilter }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn as_any(&self) -> &dyn Any { self }
}

use crate::Generated::Enums::{
    DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DBObjTypeAttribute {
    pub rust_type_name: &'static str,
    pub dat_file_type: DatFileType,
    pub db_obj_type: DBObjType,
    pub header_flags: DBObjHeaderFlags,
    pub first_id: u32,
    pub last_id: u32,
    pub mask_id: u32,
}

impl DBObjTypeAttribute {
    pub const fn is_singular(&self) -> bool {
        self.first_id == self.last_id && self.first_id != 0
    }

    pub const fn has_range_data(&self) -> bool {
        self.first_id != 0 || self.last_id != 0
    }

    pub const fn has_mask(&self) -> bool {
        self.mask_id != 0
    }
}

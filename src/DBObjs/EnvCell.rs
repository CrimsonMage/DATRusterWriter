use std::any::Any;

use crate::{
    Generated::Enums::{
        DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
        EnvCellFlags::EnvCellFlags,
    },
    Lib::{
        Attributes::DBObjTypeAttribute::DBObjTypeAttribute,
        IO::{
            DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj,
            IPackable::IPackable, IUnpackable::IUnpackable,
        },
    },
    Types::{
        CellPortal::CellPortal,
        DBObj::{DBObj, DBObjBase},
        Frame::Frame,
        Stab::Stab,
    },
};

pub const ENV_CELL_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "EnvCell",
    dat_file_type: DatFileType::Cell,
    db_obj_type: DBObjType::EnvCell,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0,
    last_id: 0,
    mask_id: 0,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnvCell {
    pub base: DBObjBase,
    pub flags: EnvCellFlags,
    pub surfaces: Vec<u16>,
    pub environment_id: u16,
    pub cell_structure: u16,
    pub position: Frame,
    pub cell_portals: Vec<CellPortal>,
    pub visible_cells: Vec<u16>,
    pub static_objects: Vec<Stab>,
    pub restriction_obj: u32,
}

impl DBObj for EnvCell {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::EnvCell
    }
    fn id(&self) -> u32 {
        self.base.id
    }
    fn set_id(&mut self, id: u32) {
        self.base.id = id;
    }
    fn data_category(&self) -> u32 {
        self.base.data_category
    }
    fn set_data_category(&mut self, data_category: u32) {
        self.base.data_category = data_category;
    }
}

impl IUnpackable for EnvCell {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.flags = EnvCellFlags::from_bits_truncate(reader.read_i32());
        let _cell_id = reader.read_u32();
        let num_surfaces = reader.read_byte() as usize;
        let num_portals = reader.read_byte() as usize;
        let num_visible_cells = reader.read_u16() as usize;
        self.surfaces = (0..num_surfaces).map(|_| reader.read_u16()).collect();
        self.environment_id = reader.read_u16();
        self.cell_structure = reader.read_u16();
        self.position = reader.read_item::<Frame>();
        self.cell_portals = (0..num_portals)
            .map(|_| reader.read_item::<CellPortal>())
            .collect();
        self.visible_cells = (0..num_visible_cells).map(|_| reader.read_u16()).collect();
        self.static_objects.clear();
        if self.flags.contains(EnvCellFlags::HasStaticObjs) {
            let num_stabs = reader.read_u32() as usize;
            self.static_objects = (0..num_stabs).map(|_| reader.read_item::<Stab>()).collect();
        }
        self.restriction_obj = if self.flags.contains(EnvCellFlags::HasRestrictionObj) {
            reader.read_u32()
        } else {
            0
        };
        true
    }
}

impl IPackable for EnvCell {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_i32(self.flags.bits());
        writer.write_u32(self.base.id);
        writer.write_byte(self.surfaces.len() as u8);
        writer.write_byte(self.cell_portals.len() as u8);
        writer.write_u16(self.visible_cells.len() as u16);
        for surface in &self.surfaces {
            writer.write_u16(*surface);
        }
        writer.write_u16(self.environment_id);
        writer.write_u16(self.cell_structure);
        writer.write_item(&self.position);
        for portal in &self.cell_portals {
            writer.write_item(portal);
        }
        for visible_cell in &self.visible_cells {
            writer.write_u16(*visible_cell);
        }
        if self.flags.contains(EnvCellFlags::HasStaticObjs) {
            writer.write_u32(self.static_objects.len() as u32);
            for object in &self.static_objects {
                writer.write_item(object);
            }
        }
        if self.flags.contains(EnvCellFlags::HasRestrictionObj) {
            writer.write_u32(self.restriction_obj);
        }
        true
    }
}

impl IDBObj for EnvCell {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &ENV_CELL_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::EnvCell
    }
    fn id(&self) -> u32 {
        self.base.id
    }
    fn set_id(&mut self, id: u32) {
        self.base.id = id;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

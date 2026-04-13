use std::any::Any;

use crate::{
    Generated::Enums::{AnimationFlags::AnimationFlags, DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType},
    Lib::{Attributes::DBObjTypeAttribute::DBObjTypeAttribute, IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj, IPackable::IPackable, IUnpackable::IUnpackable}},
    Types::{AnimationFrame::AnimationFrame, DBObj::{DBObj, DBObjBase}, Frame::Frame},
};

pub const ANIMATION_ATTR: DBObjTypeAttribute = DBObjTypeAttribute { rust_type_name: "Animation", dat_file_type: DatFileType::Portal, db_obj_type: DBObjType::Animation, header_flags: DBObjHeaderFlags::HasId, first_id: 0x03000000, last_id: 0x0300FFFF, mask_id: 0x03000000 };

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Animation {
    pub base: DBObjBase,
    pub flags: AnimationFlags,
    pub num_parts: u32,
    pub pos_frames: Vec<Frame>,
    pub part_frames: Vec<AnimationFrame>,
}

impl DBObj for Animation {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::Animation }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for Animation {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.flags = AnimationFlags::from_bits_truncate(reader.read_u32());
        self.num_parts = reader.read_u32();
        let frame_count = reader.read_u32() as usize;

        self.pos_frames.clear();
        if self.flags.contains(AnimationFlags::PosFrames) {
            for _ in 0..frame_count {
                self.pos_frames.push(reader.read_item::<Frame>());
            }
        }

        self.part_frames.clear();
        for _ in 0..frame_count {
            let mut frame = AnimationFrame::default();
            let _ = frame.unpack_with_num_parts(reader, self.num_parts);
            self.part_frames.push(frame);
        }
        true
    }
}

impl IPackable for Animation {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.flags.bits());
        writer.write_u32(self.num_parts);
        writer.write_u32(self.part_frames.len() as u32);
        if self.flags.contains(AnimationFlags::PosFrames) {
            for frame in &self.pos_frames {
                writer.write_item(frame);
            }
        }
        for frame in &self.part_frames {
            writer.write_item(frame);
        }
        true
    }
}

impl IDBObj for Animation {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self: Sized { &ANIMATION_ATTR }
    fn db_obj_type(&self) -> DBObjType { DBObjType::Animation }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn as_any(&self) -> &dyn Any { self }
}

use crate::DBObjs::{GfxObj::GfxObj, PhysicsScript::PhysicsScript};
use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};
use crate::Types::QualifiedDataId::QualifiedDataId;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkyObject {
    pub begin_time: f32,
    pub end_time: f32,
    pub begin_angle: f32,
    pub end_angle: f32,
    pub tex_velocity_x: f32,
    pub tex_velocity_y: f32,
    pub default_gfx_object_id: QualifiedDataId<GfxObj>,
    pub default_pes_object_id: QualifiedDataId<PhysicsScript>,
    pub properties: u32,
}

impl IUnpackable for SkyObject {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.begin_time = reader.read_single();
        self.end_time = reader.read_single();
        self.begin_angle = reader.read_single();
        self.end_angle = reader.read_single();
        self.tex_velocity_x = reader.read_single();
        self.tex_velocity_y = reader.read_single();
        self.default_gfx_object_id = reader.read_item::<QualifiedDataId<GfxObj>>();
        self.default_pes_object_id = reader.read_item::<QualifiedDataId<PhysicsScript>>();
        self.properties = reader.read_u32();
        true
    }
}

impl IPackable for SkyObject {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_single(self.begin_time);
        writer.write_single(self.end_time);
        writer.write_single(self.begin_angle);
        writer.write_single(self.end_angle);
        writer.write_single(self.tex_velocity_x);
        writer.write_single(self.tex_velocity_y);
        writer.write_item(&self.default_gfx_object_id);
        writer.write_item(&self.default_pes_object_id);
        writer.write_u32(self.properties);
        true
    }
}

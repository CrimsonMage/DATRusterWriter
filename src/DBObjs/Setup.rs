use std::{any::Any, collections::BTreeMap};

use crate::{
    DBObjs::{Animation::Animation, GfxObj::GfxObj, MotionTable::MotionTable, PhysicsScript::PhysicsScript, PhysicsScriptTable::PhysicsScriptTable, SoundTable::SoundTable},
    Generated::Enums::{DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType, ParentLocation::ParentLocation, Placement::Placement, SetupFlags::SetupFlags},
    Lib::{Attributes::DBObjTypeAttribute::DBObjTypeAttribute, IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj, IPackable::IPackable, IUnpackable::IUnpackable, Numerics::Vector3}},
    Types::{AnimationFrame::AnimationFrame, CylSphere::CylSphere, DBObj::{DBObj, DBObjBase}, LightInfo::LightInfo, LocationType::LocationType, QualifiedDataId::QualifiedDataId, Sphere::Sphere},
};

pub const SETUP_ATTR: DBObjTypeAttribute = DBObjTypeAttribute { rust_type_name: "Setup", dat_file_type: DatFileType::Portal, db_obj_type: DBObjType::Setup, header_flags: DBObjHeaderFlags::HasId, first_id: 0x02000000, last_id: 0x0200FFFF, mask_id: 0x00000000 };

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Setup {
    pub base: DBObjBase,
    pub flags: SetupFlags,
    pub num_parts: u32,
    pub parts: Vec<QualifiedDataId<GfxObj>>,
    pub parent_index: Vec<u32>,
    pub default_scale: Vec<Vector3>,
    pub holding_locations: BTreeMap<ParentLocation, LocationType>,
    pub connection_points: BTreeMap<ParentLocation, LocationType>,
    pub placement_frames: BTreeMap<Placement, AnimationFrame>,
    pub cyl_spheres: Vec<CylSphere>,
    pub spheres: Vec<Sphere>,
    pub height: f32,
    pub radius: f32,
    pub step_up_height: f32,
    pub step_down_height: f32,
    pub sorting_sphere: Sphere,
    pub selection_sphere: Sphere,
    pub lights: BTreeMap<i32, LightInfo>,
    pub default_animation: QualifiedDataId<Animation>,
    pub default_script: QualifiedDataId<PhysicsScript>,
    pub default_motion_table: QualifiedDataId<MotionTable>,
    pub default_sound_table: QualifiedDataId<SoundTable>,
    pub default_script_table: QualifiedDataId<PhysicsScriptTable>,
}

impl DBObj for Setup {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::Setup }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for Setup {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.flags = SetupFlags::from_bits_truncate(reader.read_u32());
        self.num_parts = reader.read_u32();

        self.parts.clear();
        for _ in 0..self.num_parts {
            self.parts.push(reader.read_item::<QualifiedDataId<GfxObj>>());
        }

        self.parent_index.clear();
        if self.flags.contains(SetupFlags::HasParent) {
            for _ in 0..self.num_parts {
                self.parent_index.push(reader.read_u32());
            }
        }

        self.default_scale.clear();
        if self.flags.contains(SetupFlags::HasDefaultScale) {
            for _ in 0..self.num_parts {
                self.default_scale.push(reader.read_vector3());
            }
        }

        let holding_count = reader.read_i32() as usize;
        self.holding_locations.clear();
        for _ in 0..holding_count {
            self.holding_locations.insert(ParentLocation::from(reader.read_i32()), reader.read_item::<LocationType>());
        }

        let connection_count = reader.read_i32() as usize;
        self.connection_points.clear();
        for _ in 0..connection_count {
            self.connection_points.insert(ParentLocation::from(reader.read_i32()), reader.read_item::<LocationType>());
        }

        let placement_count = reader.read_i32() as usize;
        self.placement_frames.clear();
        for _ in 0..placement_count {
            let key = Placement::from(reader.read_u32());
            let mut value = AnimationFrame::default();
            let _ = value.unpack_with_num_parts(reader, self.num_parts);
            self.placement_frames.insert(key, value);
        }

        let cyl_count = reader.read_u32() as usize;
        self.cyl_spheres.clear();
        for _ in 0..cyl_count {
            self.cyl_spheres.push(reader.read_item::<CylSphere>());
        }

        let sphere_count = reader.read_u32() as usize;
        self.spheres.clear();
        for _ in 0..sphere_count {
            self.spheres.push(reader.read_item::<Sphere>());
        }

        self.height = reader.read_single();
        self.radius = reader.read_single();
        self.step_up_height = reader.read_single();
        self.step_down_height = reader.read_single();
        self.sorting_sphere = reader.read_item::<Sphere>();
        self.selection_sphere = reader.read_item::<Sphere>();

        let light_count = reader.read_i32() as usize;
        self.lights.clear();
        for _ in 0..light_count {
            self.lights.insert(reader.read_i32(), reader.read_item::<LightInfo>());
        }

        self.default_animation = reader.read_item::<QualifiedDataId<Animation>>();
        self.default_script = reader.read_item::<QualifiedDataId<PhysicsScript>>();
        self.default_motion_table = reader.read_item::<QualifiedDataId<MotionTable>>();
        self.default_sound_table = reader.read_item::<QualifiedDataId<SoundTable>>();
        self.default_script_table = reader.read_item::<QualifiedDataId<PhysicsScriptTable>>();
        true
    }
}

impl IPackable for Setup {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.flags.bits());
        writer.write_u32(self.parts.len() as u32);
        for part in &self.parts {
            writer.write_item(part);
        }
        if self.flags.contains(SetupFlags::HasParent) {
            for parent in &self.parent_index {
                writer.write_u32(*parent);
            }
        }
        if self.flags.contains(SetupFlags::HasDefaultScale) {
            for scale in &self.default_scale {
                writer.write_vector3(*scale);
            }
        }

        writer.write_i32(self.holding_locations.len() as i32);
        for (key, value) in &self.holding_locations {
            writer.write_i32((*key).into());
            writer.write_item(value);
        }

        writer.write_i32(self.connection_points.len() as i32);
        for (key, value) in &self.connection_points {
            writer.write_i32((*key).into());
            writer.write_item(value);
        }

        writer.write_i32(self.placement_frames.len() as i32);
        for (key, value) in &self.placement_frames {
            writer.write_u32((*key).into());
            writer.write_item(value);
        }

        writer.write_u32(self.cyl_spheres.len() as u32);
        for cyl in &self.cyl_spheres {
            writer.write_item(cyl);
        }

        writer.write_u32(self.spheres.len() as u32);
        for sphere in &self.spheres {
            writer.write_item(sphere);
        }

        writer.write_single(self.height);
        writer.write_single(self.radius);
        writer.write_single(self.step_up_height);
        writer.write_single(self.step_down_height);
        writer.write_item(&self.sorting_sphere);
        writer.write_item(&self.selection_sphere);

        writer.write_i32(self.lights.len() as i32);
        for (key, value) in &self.lights {
            writer.write_i32(*key);
            writer.write_item(value);
        }

        writer.write_item(&self.default_animation);
        writer.write_item(&self.default_script);
        writer.write_item(&self.default_motion_table);
        writer.write_item(&self.default_sound_table);
        writer.write_item(&self.default_script_table);
        true
    }
}

impl IDBObj for Setup {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self: Sized { &SETUP_ATTR }
    fn db_obj_type(&self) -> DBObjType { DBObjType::Setup }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn as_any(&self) -> &dyn Any { self }
}

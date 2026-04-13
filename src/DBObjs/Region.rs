use std::any::Any;

use crate::{
    Generated::Enums::{DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType, PartsMask::PartsMask},
    Lib::{Attributes::DBObjTypeAttribute::DBObjTypeAttribute, IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj, IPackable::IPackable, IUnpackable::IUnpackable}},
    Types::{
        AC1LegacyString::AC1LegacyString,
        DBObj::{DBObj, DBObjBase},
        GameTime::GameTime,
        LandDefs::LandDefs,
        RegionMisc::RegionMisc,
        SceneDesc::SceneDesc,
        SkyDesc::SkyDesc,
        SoundDesc::SoundDesc,
        TerrainDesc::TerrainDesc,
    },
};

pub const REGION_ATTR: DBObjTypeAttribute = DBObjTypeAttribute { rust_type_name: "Region", dat_file_type: DatFileType::Portal, db_obj_type: DBObjType::Region, header_flags: DBObjHeaderFlags::HasId, first_id: 0x13000000, last_id: 0x1300FFFF, mask_id: 0x00000000 };

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Region {
    pub base: DBObjBase,
    pub region_number: u32,
    pub version: u32,
    pub region_name: AC1LegacyString,
    pub land_defs: LandDefs,
    pub game_time: GameTime,
    pub parts_mask: PartsMask,
    pub sky_info: Option<SkyDesc>,
    pub sound_info: Option<SoundDesc>,
    pub scene_info: Option<SceneDesc>,
    pub terrain_info: TerrainDesc,
    pub region_misc: Option<RegionMisc>,
    pub raw_remainder: Vec<u8>,
}

impl DBObj for Region {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::Region }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for Region {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.region_number = reader.read_u32();
        self.version = reader.read_u32();
        self.region_name = reader.read_item::<AC1LegacyString>();
        reader.align(4);
        self.land_defs = reader.read_item::<LandDefs>();
        self.game_time = reader.read_item::<GameTime>();
        self.parts_mask = PartsMask::from_bits_truncate(reader.read_u32());

        self.sky_info = if self.parts_mask.contains(PartsMask::HasSkyInfo) {
            Some(reader.read_item::<SkyDesc>())
        } else {
            None
        };

        self.sound_info = if self.parts_mask.contains(PartsMask::HasSoundInfo) {
            Some(reader.read_item::<SoundDesc>())
        } else {
            None
        };

        self.scene_info = if self.parts_mask.contains(PartsMask::HasSceneInfo) {
            Some(reader.read_item::<SceneDesc>())
        } else {
            None
        };

        self.terrain_info = reader.read_item::<TerrainDesc>();

        self.region_misc = if self.parts_mask.contains(PartsMask::HasRegionMisc) {
            Some(reader.read_item::<RegionMisc>())
        } else {
            None
        };

        self.raw_remainder = reader.read_remaining_bytes();
        true
    }
}

impl IPackable for Region {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.region_number);
        writer.write_u32(self.version);
        writer.write_item(&self.region_name);
        writer.align(4);
        writer.write_item(&self.land_defs);
        writer.write_item(&self.game_time);
        writer.write_u32(self.parts_mask.bits());

        if self.parts_mask.contains(PartsMask::HasSkyInfo) {
            if let Some(sky_info) = &self.sky_info {
                writer.write_item(sky_info);
            }
        }

        if self.parts_mask.contains(PartsMask::HasSoundInfo) {
            if let Some(sound_info) = &self.sound_info {
                writer.write_item(sound_info);
            }
        }

        if self.parts_mask.contains(PartsMask::HasSceneInfo) {
            if let Some(scene_info) = &self.scene_info {
                writer.write_item(scene_info);
            }
        }

        writer.write_item(&self.terrain_info);

        if self.parts_mask.contains(PartsMask::HasRegionMisc) {
            if let Some(region_misc) = &self.region_misc {
                writer.write_item(region_misc);
            }
        }

        writer.write_bytes(&self.raw_remainder, self.raw_remainder.len());
        true
    }
}

impl IDBObj for Region {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self: Sized { &REGION_ATTR }
    fn db_obj_type(&self) -> DBObjType { DBObjType::Region }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn as_any(&self) -> &dyn Any { self }
}

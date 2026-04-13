use crate::{
    DBObjs::{
        CombatTable::CombatTable, MotionTable::MotionTable, PalSet::PalSet, Palette::Palette,
        PhysicsScriptTable::PhysicsScriptTable, RenderSurface::RenderSurface, Setup::Setup,
        SoundTable::SoundTable,
    },
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{
        AC1LegacyString::AC1LegacyString, EyeStripCG::EyeStripCG, FaceStripCG::FaceStripCG,
        GearCG::GearCG, HairStyleCG::HairStyleCG, ObjDesc::ObjDesc,
        QualifiedDataId::QualifiedDataId,
    },
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SexCG {
    pub name: AC1LegacyString,
    pub scale: u32,
    pub setup_id: QualifiedDataId<Setup>,
    pub sound_table: QualifiedDataId<SoundTable>,
    pub icon_id: QualifiedDataId<RenderSurface>,
    pub base_palette: QualifiedDataId<Palette>,
    pub skin_pal_set: QualifiedDataId<PalSet>,
    pub physics_table: QualifiedDataId<PhysicsScriptTable>,
    pub motion_table: QualifiedDataId<MotionTable>,
    pub combat_table: QualifiedDataId<CombatTable>,
    pub base_obj_desc: ObjDesc,
    pub hair_colors: Vec<u32>,
    pub hair_styles: Vec<HairStyleCG>,
    pub eye_colors: Vec<u32>,
    pub eye_strips: Vec<EyeStripCG>,
    pub nose_strips: Vec<FaceStripCG>,
    pub mouth_strips: Vec<FaceStripCG>,
    pub headgears: Vec<GearCG>,
    pub shirts: Vec<GearCG>,
    pub pants: Vec<GearCG>,
    pub footwear: Vec<GearCG>,
    pub clothing_colors: Vec<u32>,
}

fn read_vec<T>(reader: &mut DatBinReader<'_>) -> Vec<T>
where
    T: IUnpackable + Default,
{
    let count = reader.read_compressed_uint() as usize;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        values.push(reader.read_item::<T>());
    }
    values
}

fn read_u32_vec(reader: &mut DatBinReader<'_>) -> Vec<u32> {
    let count = reader.read_compressed_uint() as usize;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        values.push(reader.read_u32());
    }
    values
}

fn write_vec<T>(writer: &mut DatBinWriter<'_>, values: &[T])
where
    T: IPackable,
{
    writer.write_compressed_uint(values.len() as u32);
    for value in values {
        writer.write_item(value);
    }
}

fn write_u32_vec(writer: &mut DatBinWriter<'_>, values: &[u32]) {
    writer.write_compressed_uint(values.len() as u32);
    for value in values {
        writer.write_u32(*value);
    }
}

impl IUnpackable for SexCG {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.name = reader.read_item::<AC1LegacyString>();
        self.scale = reader.read_u32();
        self.setup_id = reader.read_item::<QualifiedDataId<Setup>>();
        self.sound_table = reader.read_item::<QualifiedDataId<SoundTable>>();
        self.icon_id = reader.read_item::<QualifiedDataId<RenderSurface>>();
        self.base_palette = reader.read_item::<QualifiedDataId<Palette>>();
        self.skin_pal_set = reader.read_item::<QualifiedDataId<PalSet>>();
        self.physics_table = reader.read_item::<QualifiedDataId<PhysicsScriptTable>>();
        self.motion_table = reader.read_item::<QualifiedDataId<MotionTable>>();
        self.combat_table = reader.read_item::<QualifiedDataId<CombatTable>>();
        self.base_obj_desc = reader.read_item::<ObjDesc>();
        self.hair_colors = read_u32_vec(reader);
        self.hair_styles = read_vec::<HairStyleCG>(reader);
        self.eye_colors = read_u32_vec(reader);
        self.eye_strips = read_vec::<EyeStripCG>(reader);
        self.nose_strips = read_vec::<FaceStripCG>(reader);
        self.mouth_strips = read_vec::<FaceStripCG>(reader);
        self.headgears = read_vec::<GearCG>(reader);
        self.shirts = read_vec::<GearCG>(reader);
        self.pants = read_vec::<GearCG>(reader);
        self.footwear = read_vec::<GearCG>(reader);
        self.clothing_colors = read_u32_vec(reader);
        true
    }
}

impl IPackable for SexCG {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.name);
        writer.write_u32(self.scale);
        writer.write_item(&self.setup_id);
        writer.write_item(&self.sound_table);
        writer.write_item(&self.icon_id);
        writer.write_item(&self.base_palette);
        writer.write_item(&self.skin_pal_set);
        writer.write_item(&self.physics_table);
        writer.write_item(&self.motion_table);
        writer.write_item(&self.combat_table);
        writer.write_item(&self.base_obj_desc);
        write_u32_vec(writer, &self.hair_colors);
        write_vec(writer, &self.hair_styles);
        write_u32_vec(writer, &self.eye_colors);
        write_vec(writer, &self.eye_strips);
        write_vec(writer, &self.nose_strips);
        write_vec(writer, &self.mouth_strips);
        write_vec(writer, &self.headgears);
        write_vec(writer, &self.shirts);
        write_vec(writer, &self.pants);
        write_vec(writer, &self.footwear);
        write_u32_vec(writer, &self.clothing_colors);
        true
    }
}

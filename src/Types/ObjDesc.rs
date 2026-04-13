use crate::{
    DBObjs::{Palette::Palette, SurfaceTexture::SurfaceTexture},
    Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable},
    Types::{AnimationPartChange::AnimationPartChange, PackedQualifiedDataId::PackedQualifiedDataId},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ObjDesc {
    pub unknown_byte: u8,
    pub palette_id: PackedQualifiedDataId<Palette>,
    pub sub_palettes: Vec<SubPalette>,
    pub texture_changes: Vec<TextureMapChange>,
    pub anim_part_changes: Vec<AnimationPartChange>,
}

impl IUnpackable for ObjDesc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        reader.align(4);
        self.unknown_byte = reader.read_byte();
        let num_sub_palettes = reader.read_byte() as usize;
        let num_texture_map_changes = reader.read_byte() as usize;
        let num_anim_part_changes = reader.read_byte() as usize;

        if num_sub_palettes > 0 {
            self.palette_id = reader.read_item::<PackedQualifiedDataId<Palette>>();
        }

        self.sub_palettes.clear();
        for _ in 0..num_sub_palettes {
            self.sub_palettes.push(reader.read_item::<SubPalette>());
        }

        self.texture_changes.clear();
        for _ in 0..num_texture_map_changes {
            self.texture_changes.push(reader.read_item::<TextureMapChange>());
        }

        self.anim_part_changes.clear();
        for _ in 0..num_anim_part_changes {
            self.anim_part_changes.push(reader.read_item::<AnimationPartChange>());
        }

        reader.align(4);
        true
    }
}

impl IPackable for ObjDesc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.align(4);
        writer.write_byte(self.unknown_byte);
        writer.write_byte(self.sub_palettes.len() as u8);
        writer.write_byte(self.texture_changes.len() as u8);
        writer.write_byte(self.anim_part_changes.len() as u8);

        if !self.sub_palettes.is_empty() {
            writer.write_item(&self.palette_id);
        }

        for item in &self.sub_palettes {
            writer.write_item(item);
        }
        for item in &self.texture_changes {
            writer.write_item(item);
        }
        for item in &self.anim_part_changes {
            writer.write_item(item);
        }

        writer.align(4);
        true
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SubPalette {
    pub sub_id: PackedQualifiedDataId<Palette>,
    pub offset: u8,
    pub num_colors: u8,
}

impl IUnpackable for SubPalette {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.sub_id = reader.read_item::<PackedQualifiedDataId<Palette>>();
        self.offset = reader.read_byte();
        self.num_colors = reader.read_byte();
        true
    }
}

impl IPackable for SubPalette {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.sub_id);
        writer.write_byte(self.offset);
        writer.write_byte(self.num_colors);
        true
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextureMapChange {
    pub part_index: u8,
    pub old_texture: PackedQualifiedDataId<SurfaceTexture>,
    pub new_texture: PackedQualifiedDataId<SurfaceTexture>,
}

impl IUnpackable for TextureMapChange {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.part_index = reader.read_byte();
        self.old_texture = reader.read_item::<PackedQualifiedDataId<SurfaceTexture>>();
        self.new_texture = reader.read_item::<PackedQualifiedDataId<SurfaceTexture>>();
        true
    }
}

impl IPackable for TextureMapChange {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(self.part_index);
        writer.write_item(&self.old_texture);
        writer.write_item(&self.new_texture);
        true
    }
}

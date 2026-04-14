#![allow(non_camel_case_types)]

use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::PStringBase::PStringBase,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LayerStage {
    pub sampler_name: PStringBase<u8>,
    pub texture: u32,
    pub special_texture: u32,
    pub address_mode_u: u8,
    pub address_mode_v: u8,
    pub min_filter_mode: u8,
    pub mag_filter_mode: u8,
    pub mip_filter_mode: u8,
    pub ff_color_op: u8,
    pub ff_color_arg1: u32,
    pub ff_color_arg2: u32,
    pub ff_alpha_op: u8,
    pub ff_alpha_arg1: u32,
    pub ff_alpha_arg2: u32,
    pub ff_tex_coord_index: u32,
    pub ff_use_projection: u32,
}

impl IUnpackable for LayerStage {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.sampler_name = reader.read_item::<PStringBase<u8>>();
        self.texture = reader.read_u32();
        self.special_texture = reader.read_u32();
        self.address_mode_u = reader.read_byte();
        self.address_mode_v = reader.read_byte();
        self.min_filter_mode = reader.read_byte();
        self.mag_filter_mode = reader.read_byte();
        self.mip_filter_mode = reader.read_byte();
        self.ff_color_op = reader.read_byte();
        self.ff_color_arg1 = reader.read_u32();
        self.ff_color_arg2 = reader.read_u32();
        self.ff_alpha_op = reader.read_byte();
        self.ff_alpha_arg1 = reader.read_u32();
        self.ff_alpha_arg2 = reader.read_u32();
        self.ff_tex_coord_index = reader.read_u32();
        self.ff_use_projection = reader.read_u32();
        true
    }
}

impl IPackable for LayerStage {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.sampler_name);
        writer.write_u32(self.texture);
        writer.write_u32(self.special_texture);
        writer.write_byte(self.address_mode_u);
        writer.write_byte(self.address_mode_v);
        writer.write_byte(self.min_filter_mode);
        writer.write_byte(self.mag_filter_mode);
        writer.write_byte(self.mip_filter_mode);
        writer.write_byte(self.ff_color_op);
        writer.write_u32(self.ff_color_arg1);
        writer.write_u32(self.ff_color_arg2);
        writer.write_byte(self.ff_alpha_op);
        writer.write_u32(self.ff_alpha_arg1);
        writer.write_u32(self.ff_alpha_arg2);
        writer.write_u32(self.ff_tex_coord_index);
        writer.write_u32(self.ff_use_projection);
        true
    }
}

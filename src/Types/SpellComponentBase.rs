use crate::{
    DBObjs::RenderSurface::RenderSurface,
    Generated::Enums::ComponentType::ComponentType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{
        ObfuscatedPStringBase::ObfuscatedPStringBase, QualifiedDataId::QualifiedDataId,
    },
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpellComponentBase {
    pub name: ObfuscatedPStringBase,
    pub category: u32,
    pub icon: QualifiedDataId<RenderSurface>,
    pub component_type: ComponentType,
    pub gesture: u32,
    pub time: f32,
    pub text: ObfuscatedPStringBase,
    pub cdm: f32,
}

impl IUnpackable for SpellComponentBase {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.name = reader.read_item::<ObfuscatedPStringBase>();
        reader.align(4);
        self.category = reader.read_u32();
        self.icon = reader.read_item::<QualifiedDataId<RenderSurface>>();
        self.component_type = ComponentType::from(reader.read_u32());
        self.gesture = reader.read_u32();
        self.time = reader.read_single();
        self.text = reader.read_item::<ObfuscatedPStringBase>();
        reader.align(4);
        self.cdm = reader.read_single();
        true
    }
}

impl IPackable for SpellComponentBase {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.name);
        writer.align(4);
        writer.write_u32(self.category);
        writer.write_item(&self.icon);
        writer.write_u32(self.component_type.into());
        writer.write_u32(self.gesture);
        writer.write_single(self.time);
        writer.write_item(&self.text);
        writer.align(4);
        writer.write_single(self.cdm);
        true
    }
}

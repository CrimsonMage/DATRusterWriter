use crate::{
    Generated::Enums::RenderPassType::RenderPassType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MaterialLayer {
    pub options: u32,
    pub true_flags: u32,
    pub false_flags: u32,
    pub render_pass: RenderPassType,
}

impl IUnpackable for MaterialLayer {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.options = reader.read_u32();
        self.true_flags = reader.read_u32();
        self.false_flags = reader.read_u32();
        self.render_pass = RenderPassType::from(reader.read_u32());
        true
    }
}

impl IPackable for MaterialLayer {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.options);
        writer.write_u32(self.true_flags);
        writer.write_u32(self.false_flags);
        writer.write_u32(self.render_pass.into());
        true
    }
}

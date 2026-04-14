use crate::{
    DBObjs::ClothingTable::ClothingTable,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{PStringBase::PStringBase, QualifiedDataId::QualifiedDataId},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GearCG {
    pub name: PStringBase<u8>,
    pub clothing_table: QualifiedDataId<ClothingTable>,
    pub weenie_default: u32,
}

impl IUnpackable for GearCG {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.name = reader.read_item::<PStringBase<u8>>();
        self.clothing_table = reader.read_item::<QualifiedDataId<ClothingTable>>();
        self.weenie_default = reader.read_u32();
        true
    }
}

impl IPackable for GearCG {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.name);
        writer.write_item(&self.clothing_table);
        writer.write_u32(self.weenie_default);
        true
    }
}


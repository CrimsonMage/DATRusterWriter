use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{AC1LegacyPStringBase::AC1LegacyPStringBase, Position::Position},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Contract {
    pub version: u32,
    pub contract_id: u32,
    pub contract_name: AC1LegacyPStringBase<u8>,
    pub description: AC1LegacyPStringBase<u8>,
    pub description_progress: AC1LegacyPStringBase<u8>,
    pub name_npc_start: AC1LegacyPStringBase<u8>,
    pub name_npc_end: AC1LegacyPStringBase<u8>,
    pub questflag_stamped: AC1LegacyPStringBase<u8>,
    pub questflag_started: AC1LegacyPStringBase<u8>,
    pub questflag_finished: AC1LegacyPStringBase<u8>,
    pub questflag_progress: AC1LegacyPStringBase<u8>,
    pub questflag_timer: AC1LegacyPStringBase<u8>,
    pub questflag_repeat_time: AC1LegacyPStringBase<u8>,
    pub location_npc_start: Position,
    pub location_npc_end: Position,
    pub location_quest_area: Position,
}

impl IUnpackable for Contract {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.version = reader.read_u32();
        self.contract_id = reader.read_u32();
        self.contract_name = reader.read_item::<AC1LegacyPStringBase<u8>>();
        self.description = reader.read_item::<AC1LegacyPStringBase<u8>>();
        self.description_progress = reader.read_item::<AC1LegacyPStringBase<u8>>();
        self.name_npc_start = reader.read_item::<AC1LegacyPStringBase<u8>>();
        self.name_npc_end = reader.read_item::<AC1LegacyPStringBase<u8>>();
        self.questflag_stamped = reader.read_item::<AC1LegacyPStringBase<u8>>();
        self.questflag_started = reader.read_item::<AC1LegacyPStringBase<u8>>();
        self.questflag_finished = reader.read_item::<AC1LegacyPStringBase<u8>>();
        self.questflag_progress = reader.read_item::<AC1LegacyPStringBase<u8>>();
        self.questflag_timer = reader.read_item::<AC1LegacyPStringBase<u8>>();
        self.questflag_repeat_time = reader.read_item::<AC1LegacyPStringBase<u8>>();
        self.location_npc_start = reader.read_item::<Position>();
        self.location_npc_end = reader.read_item::<Position>();
        self.location_quest_area = reader.read_item::<Position>();
        true
    }
}

impl IPackable for Contract {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.version);
        writer.write_u32(self.contract_id);
        writer.write_item(&self.contract_name);
        writer.write_item(&self.description);
        writer.write_item(&self.description_progress);
        writer.write_item(&self.name_npc_start);
        writer.write_item(&self.name_npc_end);
        writer.write_item(&self.questflag_stamped);
        writer.write_item(&self.questflag_started);
        writer.write_item(&self.questflag_finished);
        writer.write_item(&self.questflag_progress);
        writer.write_item(&self.questflag_timer);
        writer.write_item(&self.questflag_repeat_time);
        writer.write_item(&self.location_npc_start);
        writer.write_item(&self.location_npc_end);
        writer.write_item(&self.location_quest_area);
        true
    }
}

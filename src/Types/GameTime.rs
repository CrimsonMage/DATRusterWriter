use crate::{Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable}, Types::{AC1LegacyString::AC1LegacyString, Season::Season, TimeOfDay::TimeOfDay}};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameTime {
    pub zero_time_of_year: f64,
    pub zero_year: u32,
    pub day_length: f32,
    pub days_per_year: u32,
    pub year_spec: AC1LegacyString,
    pub times_of_day: Vec<TimeOfDay>,
    pub days_of_week: Vec<AC1LegacyString>,
    pub seasons: Vec<Season>,
}

impl IUnpackable for GameTime {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.zero_time_of_year = reader.read_double();
        self.zero_year = reader.read_u32();
        self.day_length = reader.read_single();
        self.days_per_year = reader.read_u32();
        self.year_spec = reader.read_item::<AC1LegacyString>();
        reader.align(4);
        let num_times_of_day = reader.read_u32() as usize;
        self.times_of_day.clear();
        for _ in 0..num_times_of_day { self.times_of_day.push(reader.read_item::<TimeOfDay>()); }
        let num_days_of_week = reader.read_u32() as usize;
        self.days_of_week.clear();
        for _ in 0..num_days_of_week { self.days_of_week.push(reader.read_item::<AC1LegacyString>()); }
        let num_seasons = reader.read_u32() as usize;
        self.seasons.clear();
        for _ in 0..num_seasons { self.seasons.push(reader.read_item::<Season>()); }
        true
    }
}

impl IPackable for GameTime {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_double(self.zero_time_of_year);
        writer.write_u32(self.zero_year);
        writer.write_single(self.day_length);
        writer.write_u32(self.days_per_year);
        let _ = self.year_spec.pack(writer);
        writer.align(4);
        writer.write_u32(self.times_of_day.len() as u32);
        for item in &self.times_of_day { let _ = item.pack(writer); }
        writer.write_u32(self.days_of_week.len() as u32);
        for item in &self.days_of_week { let _ = item.pack(writer); }
        writer.write_u32(self.seasons.len() as u32);
        for item in &self.seasons { let _ = item.pack(writer); }
        true
    }
}

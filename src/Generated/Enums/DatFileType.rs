#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum DatFileType {
    #[default]
    Undefined = 0,
    Portal = 1,
    Cell = 2,
    Local = 3,
}

impl From<u32> for DatFileType {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Portal,
            2 => Self::Cell,
            3 => Self::Local,
            _ => Self::Undefined,
        }
    }
}

impl From<DatFileType> for u32 {
    fn from(value: DatFileType) -> Self {
        value as u32
    }
}

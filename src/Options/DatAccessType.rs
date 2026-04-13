#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DatAccessType {
    #[default]
    Read,
    ReadWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FileCachingStrategy {
    Never,
    #[default]
    OnDemand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IndexCachingStrategy {
    Never,
    #[default]
    OnDemand,
    Upfront,
}

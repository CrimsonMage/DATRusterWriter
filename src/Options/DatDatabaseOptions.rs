use crate::Options::{
    DatAccessType::DatAccessType, FileCachingStrategy::FileCachingStrategy,
    IndexCachingStrategy::IndexCachingStrategy,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatDatabaseOptions {
    pub file_path: String,
    pub index_caching_strategy: IndexCachingStrategy,
    pub file_caching_strategy: FileCachingStrategy,
    pub access_type: DatAccessType,
}

impl Default for DatDatabaseOptions {
    fn default() -> Self {
        Self {
            file_path: r"C:\Turbine\Asheron's Call\client_portal.dat".to_string(),
            index_caching_strategy: IndexCachingStrategy::OnDemand,
            file_caching_strategy: FileCachingStrategy::OnDemand,
            access_type: DatAccessType::Read,
        }
    }
}

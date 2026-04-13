use std::path::PathBuf;

use crate::Options::{
    DatAccessType::DatAccessType, FileCachingStrategy::FileCachingStrategy,
    IndexCachingStrategy::IndexCachingStrategy,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatCollectionOptions {
    pub dat_directory: String,
    pub access_type: DatAccessType,
    pub portal_dat_file_name: String,
    pub cell_dat_file_name: String,
    pub local_dat_file_name: String,
    pub high_res_dat_file_name: String,
    pub index_caching_strategy: IndexCachingStrategy,
    pub file_caching_strategy: FileCachingStrategy,
    portal_path: Option<String>,
    cell_path: Option<String>,
    local_path: Option<String>,
    high_res_path: Option<String>,
    portal_index_caching_strategy: Option<IndexCachingStrategy>,
    cell_index_caching_strategy: Option<IndexCachingStrategy>,
    local_index_caching_strategy: Option<IndexCachingStrategy>,
    high_res_index_caching_strategy: Option<IndexCachingStrategy>,
    portal_file_caching_strategy: Option<FileCachingStrategy>,
    cell_file_caching_strategy: Option<FileCachingStrategy>,
    local_file_caching_strategy: Option<FileCachingStrategy>,
    high_res_file_caching_strategy: Option<FileCachingStrategy>,
}

impl DatCollectionOptions {
    pub fn portal_index_caching_strategy(&self) -> IndexCachingStrategy {
        self.portal_index_caching_strategy
            .unwrap_or(self.index_caching_strategy)
    }

    pub fn set_portal_index_caching_strategy(&mut self, value: IndexCachingStrategy) {
        self.portal_index_caching_strategy = Some(value);
    }

    pub fn cell_index_caching_strategy(&self) -> IndexCachingStrategy {
        self.cell_index_caching_strategy
            .unwrap_or(self.index_caching_strategy)
    }

    pub fn set_cell_index_caching_strategy(&mut self, value: IndexCachingStrategy) {
        self.cell_index_caching_strategy = Some(value);
    }

    pub fn local_index_caching_strategy(&self) -> IndexCachingStrategy {
        self.local_index_caching_strategy
            .unwrap_or(self.index_caching_strategy)
    }

    pub fn set_local_index_caching_strategy(&mut self, value: IndexCachingStrategy) {
        self.local_index_caching_strategy = Some(value);
    }

    pub fn high_res_index_caching_strategy(&self) -> IndexCachingStrategy {
        self.high_res_index_caching_strategy
            .unwrap_or(self.index_caching_strategy)
    }

    pub fn set_high_res_index_caching_strategy(&mut self, value: IndexCachingStrategy) {
        self.high_res_index_caching_strategy = Some(value);
    }

    pub fn portal_file_caching_strategy(&self) -> FileCachingStrategy {
        self.portal_file_caching_strategy
            .unwrap_or(self.file_caching_strategy)
    }

    pub fn set_portal_file_caching_strategy(&mut self, value: FileCachingStrategy) {
        self.portal_file_caching_strategy = Some(value);
    }

    pub fn cell_file_caching_strategy(&self) -> FileCachingStrategy {
        self.cell_file_caching_strategy
            .unwrap_or(self.file_caching_strategy)
    }

    pub fn set_cell_file_caching_strategy(&mut self, value: FileCachingStrategy) {
        self.cell_file_caching_strategy = Some(value);
    }

    pub fn local_file_caching_strategy(&self) -> FileCachingStrategy {
        self.local_file_caching_strategy
            .unwrap_or(self.file_caching_strategy)
    }

    pub fn set_local_file_caching_strategy(&mut self, value: FileCachingStrategy) {
        self.local_file_caching_strategy = Some(value);
    }

    pub fn high_res_file_caching_strategy(&self) -> FileCachingStrategy {
        self.high_res_file_caching_strategy
            .unwrap_or(self.file_caching_strategy)
    }

    pub fn set_high_res_file_caching_strategy(&mut self, value: FileCachingStrategy) {
        self.high_res_file_caching_strategy = Some(value);
    }

    pub fn portal_dat_path(&self) -> String {
        self.portal_path.clone().unwrap_or_else(|| {
            PathBuf::from(&self.dat_directory)
                .join(&self.portal_dat_file_name)
                .to_string_lossy()
                .to_string()
        })
    }

    pub fn set_portal_dat_path(&mut self, value: impl Into<String>) {
        self.portal_path = Some(value.into());
    }

    pub fn cell_dat_path(&self) -> String {
        self.cell_path.clone().unwrap_or_else(|| {
            PathBuf::from(&self.dat_directory)
                .join(&self.cell_dat_file_name)
                .to_string_lossy()
                .to_string()
        })
    }

    pub fn set_cell_dat_path(&mut self, value: impl Into<String>) {
        self.cell_path = Some(value.into());
    }

    pub fn local_dat_path(&self) -> String {
        self.local_path.clone().unwrap_or_else(|| {
            PathBuf::from(&self.dat_directory)
                .join(&self.local_dat_file_name)
                .to_string_lossy()
                .to_string()
        })
    }

    pub fn set_local_dat_path(&mut self, value: impl Into<String>) {
        self.local_path = Some(value.into());
    }

    pub fn high_res_dat_path(&self) -> String {
        self.high_res_path.clone().unwrap_or_else(|| {
            PathBuf::from(&self.dat_directory)
                .join(&self.high_res_dat_file_name)
                .to_string_lossy()
                .to_string()
        })
    }

    pub fn set_high_res_dat_path(&mut self, value: impl Into<String>) {
        self.high_res_path = Some(value.into());
    }
}

impl Default for DatCollectionOptions {
    fn default() -> Self {
        Self {
            dat_directory: r"C:\Turbine\Asheron's Call\".to_string(),
            access_type: DatAccessType::Read,
            portal_dat_file_name: "client_portal.dat".to_string(),
            cell_dat_file_name: "client_cell_1.dat".to_string(),
            local_dat_file_name: "client_local_English.dat".to_string(),
            high_res_dat_file_name: "client_highres.dat".to_string(),
            index_caching_strategy: IndexCachingStrategy::OnDemand,
            file_caching_strategy: FileCachingStrategy::OnDemand,
            portal_path: None,
            cell_path: None,
            local_path: None,
            high_res_path: None,
            portal_index_caching_strategy: None,
            cell_index_caching_strategy: None,
            local_index_caching_strategy: None,
            high_res_index_caching_strategy: None,
            portal_file_caching_strategy: None,
            cell_file_caching_strategy: None,
            local_file_caching_strategy: None,
            high_res_file_caching_strategy: None,
        }
    }
}

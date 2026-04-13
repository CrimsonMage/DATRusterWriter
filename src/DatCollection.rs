use std::io;

use crate::{
    CellDatabase::CellDatabase,
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::{DatBTree::DatBTreeFile::DatBTreeFile, DatHeader::DatHeader, IDBObj::IDBObj},
    LocalDatabase::LocalDatabase,
    Options::{DatAccessType::DatAccessType, DatCollectionOptions::DatCollectionOptions},
    PortalDatabase::PortalDatabase,
};

pub struct DatCollection {
    pub options: DatCollectionOptions,
    pub cell: CellDatabase,
    pub portal: PortalDatabase,
    pub local: LocalDatabase,
    pub high_res: PortalDatabase,
}

impl DatCollection {
    pub fn new(options: DatCollectionOptions) -> io::Result<Self> {
        let cell = CellDatabase::new(crate::Options::DatDatabaseOptions::DatDatabaseOptions {
            file_path: options.cell_dat_path(),
            access_type: options.access_type,
            index_caching_strategy: options.cell_index_caching_strategy(),
            file_caching_strategy: options.cell_file_caching_strategy(),
        })?;

        let portal = PortalDatabase::new(crate::Options::DatDatabaseOptions::DatDatabaseOptions {
            file_path: options.portal_dat_path(),
            access_type: options.access_type,
            index_caching_strategy: options.portal_index_caching_strategy(),
            file_caching_strategy: options.portal_file_caching_strategy(),
        })?;

        let local = LocalDatabase::new(crate::Options::DatDatabaseOptions::DatDatabaseOptions {
            file_path: options.local_dat_path(),
            access_type: options.access_type,
            index_caching_strategy: options.local_index_caching_strategy(),
            file_caching_strategy: options.local_file_caching_strategy(),
        })?;

        let high_res = PortalDatabase::new(crate::Options::DatDatabaseOptions::DatDatabaseOptions {
            file_path: options.high_res_dat_path(),
            access_type: options.access_type,
            index_caching_strategy: options.high_res_index_caching_strategy(),
            file_caching_strategy: options.high_res_file_caching_strategy(),
        })?;

        Ok(Self {
            options,
            cell,
            portal,
            local,
            high_res,
        })
    }

    pub fn from_directory(dat_directory: impl Into<String>, dat_access_type: DatAccessType) -> io::Result<Self> {
        let mut options = DatCollectionOptions::default();
        options.dat_directory = dat_directory.into();
        options.access_type = dat_access_type;
        Self::new(options)
    }

    pub fn clear_cache(&mut self) {
        self.cell.clear_cache();
        self.portal.clear_cache();
        self.local.clear_cache();
        self.high_res.clear_cache();
    }

    pub fn try_get_file_entry(&self, dat_file_type: DatFileType, file_id: u32) -> io::Result<Option<DatBTreeFile>> {
        match dat_file_type {
            DatFileType::Cell => self.cell.inner.try_get_file_entry(file_id),
            DatFileType::Portal => {
                let portal = self.portal.inner.try_get_file_entry(file_id)?;
                if portal.is_some() {
                    Ok(portal)
                } else {
                    self.high_res.inner.try_get_file_entry(file_id)
                }
            }
            DatFileType::Local => self.local.inner.try_get_file_entry(file_id),
            DatFileType::Undefined => Ok(None),
        }
    }

    pub fn try_get_file_bytes(&self, dat_file_type: DatFileType, file_id: u32, auto_decompress: bool) -> io::Result<Option<Vec<u8>>> {
        match dat_file_type {
            DatFileType::Cell => self.cell.inner.try_get_file_bytes(file_id, auto_decompress),
            DatFileType::Portal => {
                let portal = self.portal.inner.try_get_file_bytes(file_id, auto_decompress)?;
                if portal.is_some() {
                    Ok(portal)
                } else {
                    self.high_res.inner.try_get_file_bytes(file_id, auto_decompress)
                }
            }
            DatFileType::Local => self.local.inner.try_get_file_bytes(file_id, auto_decompress),
            DatFileType::Undefined => Ok(None),
        }
    }

    pub fn header_for(&self, dat_file_type: DatFileType) -> Option<&DatHeader> {
        match dat_file_type {
            DatFileType::Cell => Some(self.cell.inner.header()),
            DatFileType::Portal => Some(self.portal.inner.header()),
            DatFileType::Local => Some(self.local.inner.header()),
            DatFileType::Undefined => None,
        }
    }

    pub fn type_to_dat_file_type<T>(&self) -> DatFileType
    where
        T: IDBObj,
    {
        T::db_obj_type_attr().dat_file_type
    }

    pub fn try_get<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default,
    {
        match self.type_to_dat_file_type::<T>() {
            DatFileType::Cell => self.cell.try_get::<T>(file_id),
            DatFileType::Portal => {
                let portal = self.portal.try_get::<T>(file_id)?;
                if portal.is_some() {
                    Ok(portal)
                } else {
                    self.high_res.try_get::<T>(file_id)
                }
            }
            DatFileType::Local => self.local.try_get::<T>(file_id),
            DatFileType::Undefined => Ok(None),
        }
    }

    pub fn get<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default,
    {
        self.try_get::<T>(file_id)
    }

    pub fn get_all_ids_of_type<T>(&self) -> io::Result<Vec<u32>>
    where
        T: IDBObj,
    {
        match self.type_to_dat_file_type::<T>() {
            DatFileType::Cell => self.cell.get_all_ids_of_type::<T>(),
            DatFileType::Portal => {
                let mut ids = self.portal.get_all_ids_of_type::<T>()?;
                ids.extend(self.high_res.get_all_ids_of_type::<T>()?);
                ids.sort_unstable();
                ids.dedup();
                Ok(ids)
            }
            DatFileType::Local => self.local.get_all_ids_of_type::<T>(),
            DatFileType::Undefined => Ok(Vec::new()),
        }
    }
}

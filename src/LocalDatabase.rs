use std::io;

use crate::{
    DatDatabase::DatDatabase,
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::{DatBTree::DatBTreeFile::DatBTreeFile, IDBObj::IDBObj},
    Options::{DatAccessType::DatAccessType, DatDatabaseOptions::DatDatabaseOptions},
};

pub struct LocalDatabase {
    pub inner: DatDatabase,
}

impl LocalDatabase {
    pub fn new(options: DatDatabaseOptions) -> io::Result<Self> {
        let inner = DatDatabase::new(options)?;
        if inner.block_allocator.has_header_data() && inner.header().r#type != DatFileType::Local {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Tried to open {} as a local database, but its type is {:?}",
                    inner.options.file_path,
                    inner.header().r#type
                ),
            ));
        }

        Ok(Self { inner })
    }

    pub fn from_path(
        dat_file_path: impl Into<String>,
        access_type: DatAccessType,
    ) -> io::Result<Self> {
        let options = DatDatabaseOptions {
            file_path: dat_file_path.into(),
            access_type,
            ..DatDatabaseOptions::default()
        };
        Self::new(options)
    }

    pub fn clear_cache(&mut self) {
        self.inner.clear_cache();
    }

    pub fn try_get<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default,
    {
        self.inner.try_get::<T>(file_id)
    }

    pub fn get_all_ids_of_type<T>(&self) -> io::Result<Vec<u32>>
    where
        T: IDBObj,
    {
        self.inner.get_all_ids_of_type::<T>()
    }

    pub fn try_get_file_entry(&self, file_id: u32) -> io::Result<Option<DatBTreeFile>> {
        self.inner.try_get_file_entry(file_id)
    }

    pub fn try_get_file_bytes(
        &self,
        file_id: u32,
        auto_decompress: bool,
    ) -> io::Result<Option<Vec<u8>>> {
        self.inner.try_get_file_bytes(file_id, auto_decompress)
    }
}

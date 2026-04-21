use std::io;

use crate::{
    DBObjs::{MasterProperty::MasterProperty, Region::Region},
    DatDatabase::DatDatabase,
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::{DatBTree::DatBTreeFile::DatBTreeFile, IDBObj::IDBObj, IPackable::IPackable},
    Options::{DatAccessType::DatAccessType, DatDatabaseOptions::DatDatabaseOptions},
};

pub struct PortalDatabase {
    pub inner: DatDatabase,
}

impl PortalDatabase {
    pub fn new(options: DatDatabaseOptions) -> io::Result<Self> {
        let inner = DatDatabase::new(options)?;
        if inner.block_allocator.has_header_data() && inner.header().r#type != DatFileType::Portal {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Tried to open {} as a portal database, but its type is {:?}",
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

    pub async fn try_get_async<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default + Send,
    {
        self.inner.try_get_async::<T>(file_id).await
    }

    pub fn get_cached<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default + Clone + Send + 'static,
    {
        self.inner.get_cached::<T>(file_id)
    }

    pub async fn get_cached_async<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default + Clone + Send + 'static,
    {
        self.inner.get_cached_async::<T>(file_id).await
    }

    pub fn master_property(&self) -> io::Result<Option<MasterProperty>> {
        self.try_get::<MasterProperty>(0x3900_0001)
    }

    pub fn region(&self) -> io::Result<Option<Region>> {
        self.try_get::<Region>(0x1300_0000)
    }

    pub fn try_write_file<T>(&self, value: &T) -> io::Result<bool>
    where
        T: IDBObj + IPackable,
    {
        self.inner.try_write_file(value)
    }

    pub async fn try_write_file_async<T>(&self, value: &T) -> io::Result<bool>
    where
        T: IDBObj + IPackable + Sync,
    {
        self.inner.try_write_file_async(value).await
    }

    pub fn try_write_file_with_template<T>(
        &self,
        value: &T,
        template: DatBTreeFile,
    ) -> io::Result<bool>
    where
        T: IDBObj + IPackable,
    {
        self.inner.try_write_file_with_template(value, template)
    }

    pub async fn try_write_file_with_template_async<T>(
        &self,
        value: &T,
        template: DatBTreeFile,
    ) -> io::Result<bool>
    where
        T: IDBObj + IPackable + Sync,
    {
        self.inner
            .try_write_file_with_template_async(value, template)
            .await
    }

    pub fn try_write_compressed<T>(&self, value: &T) -> io::Result<bool>
    where
        T: IDBObj + IPackable,
    {
        self.inner.try_write_compressed(value)
    }

    pub async fn try_write_compressed_async<T>(&self, value: &T) -> io::Result<bool>
    where
        T: IDBObj + IPackable + Sync,
    {
        self.inner.try_write_compressed_async(value).await
    }

    pub fn try_write_compressed_with_template<T>(
        &self,
        value: &T,
        template: DatBTreeFile,
    ) -> io::Result<bool>
    where
        T: IDBObj + IPackable,
    {
        self.inner
            .try_write_compressed_with_template(value, template)
    }

    pub async fn try_write_compressed_with_template_async<T>(
        &self,
        value: &T,
        template: DatBTreeFile,
    ) -> io::Result<bool>
    where
        T: IDBObj + IPackable + Sync,
    {
        self.inner
            .try_write_compressed_with_template_async(value, template)
            .await
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

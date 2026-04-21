use std::{any::TypeId, io, sync::Arc};

use crate::{
    CellDatabase::CellDatabase,
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::{
        DatBTree::DatBTreeFile::DatBTreeFile, DatHeader::DatHeader, IDBObj::IDBObj,
        IPackable::IPackable,
    },
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
            typed_object_cache_budget_bytes: options.cell_typed_object_cache_budget_bytes(),
        })?;

        let portal = PortalDatabase::new(crate::Options::DatDatabaseOptions::DatDatabaseOptions {
            file_path: options.portal_dat_path(),
            access_type: options.access_type,
            index_caching_strategy: options.portal_index_caching_strategy(),
            file_caching_strategy: options.portal_file_caching_strategy(),
            typed_object_cache_budget_bytes: options.portal_typed_object_cache_budget_bytes(),
        })?;

        let local = LocalDatabase::new(crate::Options::DatDatabaseOptions::DatDatabaseOptions {
            file_path: options.local_dat_path(),
            access_type: options.access_type,
            index_caching_strategy: options.local_index_caching_strategy(),
            file_caching_strategy: options.local_file_caching_strategy(),
            typed_object_cache_budget_bytes: options.local_typed_object_cache_budget_bytes(),
        })?;

        let high_res =
            PortalDatabase::new(crate::Options::DatDatabaseOptions::DatDatabaseOptions {
                file_path: options.high_res_dat_path(),
                access_type: options.access_type,
                index_caching_strategy: options.high_res_index_caching_strategy(),
                file_caching_strategy: options.high_res_file_caching_strategy(),
                typed_object_cache_budget_bytes: options.high_res_typed_object_cache_budget_bytes(),
            })?;

        Ok(Self {
            options,
            cell,
            portal,
            local,
            high_res,
        })
    }

    pub fn from_directory(
        dat_directory: impl Into<String>,
        dat_access_type: DatAccessType,
    ) -> io::Result<Self> {
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

    pub fn try_get_file_entry(
        &self,
        dat_file_type: DatFileType,
        file_id: u32,
    ) -> io::Result<Option<DatBTreeFile>> {
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
            DatFileType::Undefined => {
                let portal = self.portal.inner.try_get_file_entry(file_id)?;
                if portal.is_some() {
                    return Ok(portal);
                }
                let high_res = self.high_res.inner.try_get_file_entry(file_id)?;
                if high_res.is_some() {
                    return Ok(high_res);
                }
                let local = self.local.inner.try_get_file_entry(file_id)?;
                if local.is_some() {
                    return Ok(local);
                }
                self.cell.inner.try_get_file_entry(file_id)
            }
        }
    }

    pub fn try_get_file_bytes(
        &self,
        dat_file_type: DatFileType,
        file_id: u32,
        auto_decompress: bool,
    ) -> io::Result<Option<Vec<u8>>> {
        match dat_file_type {
            DatFileType::Cell => self.cell.inner.try_get_file_bytes(file_id, auto_decompress),
            DatFileType::Portal => {
                let portal = self
                    .portal
                    .inner
                    .try_get_file_bytes(file_id, auto_decompress)?;
                if portal.is_some() {
                    Ok(portal)
                } else {
                    self.high_res
                        .inner
                        .try_get_file_bytes(file_id, auto_decompress)
                }
            }
            DatFileType::Local => self
                .local
                .inner
                .try_get_file_bytes(file_id, auto_decompress),
            DatFileType::Undefined => {
                let portal = self
                    .portal
                    .inner
                    .try_get_file_bytes(file_id, auto_decompress)?;
                if portal.is_some() {
                    return Ok(portal);
                }
                let high_res = self
                    .high_res
                    .inner
                    .try_get_file_bytes(file_id, auto_decompress)?;
                if high_res.is_some() {
                    return Ok(high_res);
                }
                let local = self
                    .local
                    .inner
                    .try_get_file_bytes(file_id, auto_decompress)?;
                if local.is_some() {
                    return Ok(local);
                }
                self.cell.inner.try_get_file_bytes(file_id, auto_decompress)
            }
        }
    }

    pub fn header_for(&self, dat_file_type: DatFileType) -> Option<DatHeader> {
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
        if TypeId::of::<T>() == TypeId::of::<crate::DBObjs::Iteration::Iteration>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Iteration is not a valid type to read from a dat collection; use a specific database instead",
            ));
        }

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
            DatFileType::Local => self.local.inner.try_get_with_base_property_types(
                file_id,
                self.portal.inner.base_property_types()?,
            ),
            DatFileType::Undefined => {
                let portal = self.portal.try_get::<T>(file_id)?;
                if portal.is_some() {
                    return Ok(portal);
                }
                let high_res = self.high_res.try_get::<T>(file_id)?;
                if high_res.is_some() {
                    return Ok(high_res);
                }
                let local = self.local.inner.try_get_with_base_property_types(
                    file_id,
                    self.portal.inner.base_property_types()?,
                )?;
                if local.is_some() {
                    return Ok(local);
                }
                self.cell.try_get::<T>(file_id)
            }
        }
    }

    pub fn get<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default,
    {
        self.try_get::<T>(file_id)
    }

    pub async fn try_get_async<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default + Send,
    {
        if TypeId::of::<T>() == TypeId::of::<crate::DBObjs::Iteration::Iteration>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Iteration is not a valid type to read from a dat collection; use a specific database instead",
            ));
        }

        match self.type_to_dat_file_type::<T>() {
            DatFileType::Cell => self.cell.try_get_async::<T>(file_id).await,
            DatFileType::Portal => {
                let portal = self.portal.try_get_async::<T>(file_id).await?;
                if portal.is_some() {
                    Ok(portal)
                } else {
                    self.high_res.try_get_async::<T>(file_id).await
                }
            }
            DatFileType::Local => self.local.inner.try_get_with_base_property_types(
                file_id,
                self.portal.inner.base_property_types()?,
            ),
            DatFileType::Undefined => {
                let portal = self.portal.try_get_async::<T>(file_id).await?;
                if portal.is_some() {
                    return Ok(portal);
                }
                let high_res = self.high_res.try_get_async::<T>(file_id).await?;
                if high_res.is_some() {
                    return Ok(high_res);
                }
                let local = self.local.inner.try_get_with_base_property_types(
                    file_id,
                    self.portal.inner.base_property_types()?,
                )?;
                if local.is_some() {
                    return Ok(local);
                }
                self.cell.try_get_async::<T>(file_id).await
            }
        }
    }

    pub async fn get_async<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default + Send,
    {
        self.try_get_async::<T>(file_id).await
    }

    pub fn get_cached<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default + Clone + Send + 'static,
    {
        if TypeId::of::<T>() == TypeId::of::<crate::DBObjs::Iteration::Iteration>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Iteration is not a valid type to read from a dat collection; use a specific database instead",
            ));
        }

        match self.type_to_dat_file_type::<T>() {
            DatFileType::Cell => self.cell.get_cached::<T>(file_id),
            DatFileType::Portal => {
                let portal = self.portal.get_cached::<T>(file_id)?;
                if portal.is_some() {
                    Ok(portal)
                } else {
                    self.high_res.get_cached::<T>(file_id)
                }
            }
            DatFileType::Local => self.local.inner.get_cached_with_base_property_types(
                file_id,
                self.portal.inner.base_property_types()?,
            ),
            DatFileType::Undefined => {
                let portal = self.portal.get_cached::<T>(file_id)?;
                if portal.is_some() {
                    return Ok(portal);
                }
                let high_res = self.high_res.get_cached::<T>(file_id)?;
                if high_res.is_some() {
                    return Ok(high_res);
                }
                let local = self.local.inner.get_cached_with_base_property_types(
                    file_id,
                    self.portal.inner.base_property_types()?,
                )?;
                if local.is_some() {
                    return Ok(local);
                }
                self.cell.get_cached::<T>(file_id)
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_cached_shared<T>(&self, file_id: u32) -> io::Result<Option<Arc<T>>>
    where
        T: IDBObj + Default + Send + Sync + 'static,
    {
        if TypeId::of::<T>() == TypeId::of::<crate::DBObjs::Iteration::Iteration>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Iteration is not a valid type to read from a dat collection; use a specific database instead",
            ));
        }

        match self.type_to_dat_file_type::<T>() {
            DatFileType::Cell => self.cell.inner.get_cached_shared::<T>(file_id),
            DatFileType::Portal => {
                let portal = self.portal.inner.get_cached_shared::<T>(file_id)?;
                if portal.is_some() {
                    Ok(portal)
                } else {
                    self.high_res.inner.get_cached_shared::<T>(file_id)
                }
            }
            DatFileType::Local => self.local.inner.get_cached_shared_with_base_property_types(
                file_id,
                self.portal.inner.base_property_types()?,
            ),
            DatFileType::Undefined => {
                let portal = self.portal.inner.get_cached_shared::<T>(file_id)?;
                if portal.is_some() {
                    return Ok(portal);
                }
                let high_res = self.high_res.inner.get_cached_shared::<T>(file_id)?;
                if high_res.is_some() {
                    return Ok(high_res);
                }
                let local = self
                    .local
                    .inner
                    .get_cached_shared_with_base_property_types(
                        file_id,
                        self.portal.inner.base_property_types()?,
                    )?;
                if local.is_some() {
                    return Ok(local);
                }
                self.cell.inner.get_cached_shared::<T>(file_id)
            }
        }
    }

    pub async fn get_cached_async<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default + Clone + Send + 'static,
    {
        if TypeId::of::<T>() == TypeId::of::<crate::DBObjs::Iteration::Iteration>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Iteration is not a valid type to read from a dat collection; use a specific database instead",
            ));
        }

        match self.type_to_dat_file_type::<T>() {
            DatFileType::Cell => self.cell.get_cached_async::<T>(file_id).await,
            DatFileType::Portal => {
                let portal = self.portal.get_cached_async::<T>(file_id).await?;
                if portal.is_some() {
                    Ok(portal)
                } else {
                    self.high_res.get_cached_async::<T>(file_id).await
                }
            }
            DatFileType::Local => self.local.inner.get_cached_with_base_property_types(
                file_id,
                self.portal.inner.base_property_types()?,
            ),
            DatFileType::Undefined => {
                let portal = self.portal.get_cached_async::<T>(file_id).await?;
                if portal.is_some() {
                    return Ok(portal);
                }
                let high_res = self.high_res.get_cached_async::<T>(file_id).await?;
                if high_res.is_some() {
                    return Ok(high_res);
                }
                let local = self.local.inner.get_cached_with_base_property_types(
                    file_id,
                    self.portal.inner.base_property_types()?,
                )?;
                if local.is_some() {
                    return Ok(local);
                }
                self.cell.get_cached_async::<T>(file_id).await
            }
        }
    }

    pub fn try_write_file<T>(&self, value: &T) -> io::Result<bool>
    where
        T: IDBObj + IPackable,
    {
        if TypeId::of::<T>() == TypeId::of::<crate::DBObjs::Iteration::Iteration>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Iteration is not a valid type to write through a dat collection; use a specific database instead",
            ));
        }

        match self.type_to_dat_file_type::<T>() {
            DatFileType::Cell => self.cell.try_write_file(value),
            DatFileType::Portal => self.portal.try_write_file(value),
            DatFileType::Local => self.local.try_write_file(value),
            DatFileType::Undefined => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unable to determine dat file type for write",
            )),
        }
    }

    pub async fn try_write_file_async<T>(&self, value: &T) -> io::Result<bool>
    where
        T: IDBObj + IPackable + Sync,
    {
        if TypeId::of::<T>() == TypeId::of::<crate::DBObjs::Iteration::Iteration>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Iteration is not a valid type to write through a dat collection; use a specific database instead",
            ));
        }

        match self.type_to_dat_file_type::<T>() {
            DatFileType::Cell => self.cell.try_write_file_async(value).await,
            DatFileType::Portal => self.portal.try_write_file_async(value).await,
            DatFileType::Local => self.local.try_write_file_async(value).await,
            DatFileType::Undefined => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unable to determine dat file type for write",
            )),
        }
    }

    pub fn try_write_file_with_template<T>(
        &self,
        value: &T,
        template: DatBTreeFile,
    ) -> io::Result<bool>
    where
        T: IDBObj + IPackable,
    {
        if TypeId::of::<T>() == TypeId::of::<crate::DBObjs::Iteration::Iteration>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Iteration is not a valid type to write through a dat collection; use a specific database instead",
            ));
        }

        match self.type_to_dat_file_type::<T>() {
            DatFileType::Cell => self.cell.try_write_file_with_template(value, template),
            DatFileType::Portal => self.portal.try_write_file_with_template(value, template),
            DatFileType::Local => self.local.try_write_file_with_template(value, template),
            DatFileType::Undefined => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unable to determine dat file type for write",
            )),
        }
    }

    pub async fn try_write_file_with_template_async<T>(
        &self,
        value: &T,
        template: DatBTreeFile,
    ) -> io::Result<bool>
    where
        T: IDBObj + IPackable + Sync,
    {
        if TypeId::of::<T>() == TypeId::of::<crate::DBObjs::Iteration::Iteration>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Iteration is not a valid type to write through a dat collection; use a specific database instead",
            ));
        }

        match self.type_to_dat_file_type::<T>() {
            DatFileType::Cell => {
                self.cell
                    .try_write_file_with_template_async(value, template)
                    .await
            }
            DatFileType::Portal => {
                self.portal
                    .try_write_file_with_template_async(value, template)
                    .await
            }
            DatFileType::Local => {
                self.local
                    .try_write_file_with_template_async(value, template)
                    .await
            }
            DatFileType::Undefined => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unable to determine dat file type for write",
            )),
        }
    }

    pub fn try_write_compressed<T>(&self, value: &T) -> io::Result<bool>
    where
        T: IDBObj + IPackable,
    {
        if TypeId::of::<T>() == TypeId::of::<crate::DBObjs::Iteration::Iteration>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Iteration is not a valid type to write through a dat collection; use a specific database instead",
            ));
        }

        match self.type_to_dat_file_type::<T>() {
            DatFileType::Cell => self.cell.try_write_compressed(value),
            DatFileType::Portal => self.portal.try_write_compressed(value),
            DatFileType::Local => self.local.try_write_compressed(value),
            DatFileType::Undefined => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unable to determine dat file type for write",
            )),
        }
    }

    pub async fn try_write_compressed_async<T>(&self, value: &T) -> io::Result<bool>
    where
        T: IDBObj + IPackable + Sync,
    {
        if TypeId::of::<T>() == TypeId::of::<crate::DBObjs::Iteration::Iteration>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Iteration is not a valid type to write through a dat collection; use a specific database instead",
            ));
        }

        match self.type_to_dat_file_type::<T>() {
            DatFileType::Cell => self.cell.try_write_compressed_async(value).await,
            DatFileType::Portal => self.portal.try_write_compressed_async(value).await,
            DatFileType::Local => self.local.try_write_compressed_async(value).await,
            DatFileType::Undefined => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unable to determine dat file type for write",
            )),
        }
    }

    pub fn try_write_compressed_with_template<T>(
        &self,
        value: &T,
        template: DatBTreeFile,
    ) -> io::Result<bool>
    where
        T: IDBObj + IPackable,
    {
        if TypeId::of::<T>() == TypeId::of::<crate::DBObjs::Iteration::Iteration>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Iteration is not a valid type to write through a dat collection; use a specific database instead",
            ));
        }

        match self.type_to_dat_file_type::<T>() {
            DatFileType::Cell => self
                .cell
                .try_write_compressed_with_template(value, template),
            DatFileType::Portal => self
                .portal
                .try_write_compressed_with_template(value, template),
            DatFileType::Local => self
                .local
                .try_write_compressed_with_template(value, template),
            DatFileType::Undefined => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unable to determine dat file type for write",
            )),
        }
    }

    pub async fn try_write_compressed_with_template_async<T>(
        &self,
        value: &T,
        template: DatBTreeFile,
    ) -> io::Result<bool>
    where
        T: IDBObj + IPackable + Sync,
    {
        if TypeId::of::<T>() == TypeId::of::<crate::DBObjs::Iteration::Iteration>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Iteration is not a valid type to write through a dat collection; use a specific database instead",
            ));
        }

        match self.type_to_dat_file_type::<T>() {
            DatFileType::Cell => {
                self.cell
                    .try_write_compressed_with_template_async(value, template)
                    .await
            }
            DatFileType::Portal => {
                self.portal
                    .try_write_compressed_with_template_async(value, template)
                    .await
            }
            DatFileType::Local => {
                self.local
                    .try_write_compressed_with_template_async(value, template)
                    .await
            }
            DatFileType::Undefined => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unable to determine dat file type for write",
            )),
        }
    }

    pub fn get_all_ids_of_type<T>(&self) -> io::Result<Vec<u32>>
    where
        T: IDBObj,
    {
        if TypeId::of::<T>() == TypeId::of::<crate::DBObjs::Iteration::Iteration>() {
            return Ok(Vec::new());
        }

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
            DatFileType::Undefined => {
                let mut ids = self.portal.get_all_ids_of_type::<T>()?;
                ids.extend(self.high_res.get_all_ids_of_type::<T>()?);
                ids.extend(self.local.get_all_ids_of_type::<T>()?);
                ids.extend(self.cell.get_all_ids_of_type::<T>()?);
                ids.sort_unstable();
                ids.dedup();
                Ok(ids)
            }
        }
    }
}

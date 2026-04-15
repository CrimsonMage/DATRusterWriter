use std::io;
use std::sync::Arc;

use crate::{
    DBObjs::{
        Animation::Animation, EnvCell::EnvCell, Font::Font, GfxObj::GfxObj,
        LandBlock::LandBlock, LandBlockInfo::LandBlockInfo, LayoutDesc::LayoutDesc,
        MasterProperty::MasterProperty, MotionTable::MotionTable, Palette::Palette,
        Region::Region, RenderTexture::RenderTexture, Setup::Setup, StringTable::StringTable,
    },
    DatCollection::DatCollection,
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::{DatBTree::DatBTreeFile::DatBTreeFile, DatHeader::DatHeader, IDBObj::IDBObj},
    Options::{
        DatAccessType::DatAccessType, DatCollectionOptions::DatCollectionOptions,
    },
};

pub struct ClientDatStore {
    collection: DatCollection,
}

impl ClientDatStore {
    pub fn new(options: DatCollectionOptions) -> io::Result<Self> {
        Ok(Self {
            collection: DatCollection::new(options)?,
        })
    }

    pub fn open(
        dat_directory: impl Into<String>,
        dat_access_type: DatAccessType,
    ) -> io::Result<Self> {
        Ok(Self {
            collection: DatCollection::from_directory(dat_directory, dat_access_type)?,
        })
    }

    pub fn collection(&self) -> &DatCollection {
        &self.collection
    }

    pub fn collection_mut(&mut self) -> &mut DatCollection {
        &mut self.collection
    }

    pub fn clear_cache(&mut self) {
        self.collection.clear_cache();
    }

    pub fn header_for(&self, dat_file_type: DatFileType) -> Option<DatHeader> {
        self.collection.header_for(dat_file_type)
    }

    pub fn load<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default,
    {
        self.collection.get::<T>(file_id)
    }

    pub async fn load_async<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default + Send,
    {
        self.collection.get_async::<T>(file_id).await
    }

    pub fn load_cached<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default + Clone + Send + 'static,
    {
        self.collection.get_cached::<T>(file_id)
    }

    pub async fn load_cached_async<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default + Clone + Send + 'static,
    {
        self.collection.get_cached_async::<T>(file_id).await
    }

    #[allow(dead_code)]
    pub(crate) fn load_cached_shared<T>(&self, file_id: u32) -> io::Result<Option<Arc<T>>>
    where
        T: IDBObj + Default + Send + Sync + 'static,
    {
        self.collection.get_cached_shared::<T>(file_id)
    }

    pub fn load_ids<T>(&self) -> io::Result<Vec<u32>>
    where
        T: IDBObj,
    {
        self.collection.get_all_ids_of_type::<T>()
    }

    pub fn load_file_entry(
        &self,
        dat_file_type: DatFileType,
        file_id: u32,
    ) -> io::Result<Option<DatBTreeFile>> {
        self.collection.try_get_file_entry(dat_file_type, file_id)
    }

    pub fn load_file_bytes(
        &self,
        dat_file_type: DatFileType,
        file_id: u32,
        auto_decompress: bool,
    ) -> io::Result<Option<Vec<u8>>> {
        self.collection
            .try_get_file_bytes(dat_file_type, file_id, auto_decompress)
    }

    pub fn master_property(&self) -> io::Result<Option<MasterProperty>> {
        self.collection.portal.master_property()
    }

    pub fn region(&self) -> io::Result<Option<Region>> {
        self.collection.portal.region()
    }

    pub fn layout(&self, file_id: u32) -> io::Result<Option<LayoutDesc>> {
        self.load::<LayoutDesc>(file_id)
    }

    pub fn setup(&self, file_id: u32) -> io::Result<Option<Setup>> {
        self.load::<Setup>(file_id)
    }

    pub fn env_cell(&self, file_id: u32) -> io::Result<Option<EnvCell>> {
        self.load::<EnvCell>(file_id)
    }

    pub fn land_block(&self, file_id: u32) -> io::Result<Option<LandBlock>> {
        self.load::<LandBlock>(file_id)
    }

    pub fn land_block_info(&self, file_id: u32) -> io::Result<Option<LandBlockInfo>> {
        self.load::<LandBlockInfo>(file_id)
    }

    pub fn palette(&self, file_id: u32) -> io::Result<Option<Palette>> {
        self.load::<Palette>(file_id)
    }

    pub fn render_texture(&self, file_id: u32) -> io::Result<Option<RenderTexture>> {
        self.load::<RenderTexture>(file_id)
    }

    pub fn font(&self, file_id: u32) -> io::Result<Option<Font>> {
        self.load::<Font>(file_id)
    }

    pub fn string_table(&self, file_id: u32) -> io::Result<Option<StringTable>> {
        self.load::<StringTable>(file_id)
    }

    pub fn gfx_obj(&self, file_id: u32) -> io::Result<Option<GfxObj>> {
        self.load::<GfxObj>(file_id)
    }

    pub fn animation(&self, file_id: u32) -> io::Result<Option<Animation>> {
        self.load::<Animation>(file_id)
    }

    pub fn motion_table(&self, file_id: u32) -> io::Result<Option<MotionTable>> {
        self.load::<MotionTable>(file_id)
    }
}

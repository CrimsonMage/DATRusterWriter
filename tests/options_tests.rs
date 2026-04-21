use dat_ruster_writer::Options::{
    DatAccessType::DatAccessType, DatCollectionOptions::DatCollectionOptions,
    DatDatabaseOptions::DatDatabaseOptions, FileCachingStrategy::FileCachingStrategy,
    IndexCachingStrategy::IndexCachingStrategy,
};

#[test]
fn dat_database_options_defaults_to_read_on_demand() {
    let options = DatDatabaseOptions::default();
    assert_eq!(DatAccessType::Read, options.access_type);
    assert_eq!(
        IndexCachingStrategy::OnDemand,
        options.index_caching_strategy
    );
    assert_eq!(FileCachingStrategy::OnDemand, options.file_caching_strategy);
    assert_eq!(8 * 1024 * 1024, options.typed_object_cache_budget_bytes);
    assert!(options.file_path.ends_with("client_portal.dat"));
}

#[test]
fn dat_collection_options_builds_default_paths() {
    let options = DatCollectionOptions::default();
    assert!(options.portal_dat_path().ends_with("client_portal.dat"));
    assert!(options.cell_dat_path().ends_with("client_cell_1.dat"));
    assert!(
        options
            .local_dat_path()
            .ends_with("client_local_English.dat")
    );
    assert!(options.high_res_dat_path().ends_with("client_highres.dat"));
}

#[test]
fn dat_collection_options_honors_overrides() {
    let mut options = DatCollectionOptions::default();
    options.set_portal_dat_path(r"D:\custom\portal.dat");
    options.set_high_res_index_caching_strategy(IndexCachingStrategy::Upfront);
    options.set_local_file_caching_strategy(FileCachingStrategy::Never);
    options.set_high_res_typed_object_cache_budget_bytes(1024);

    assert_eq!(r"D:\custom\portal.dat", options.portal_dat_path());
    assert_eq!(
        IndexCachingStrategy::Upfront,
        options.high_res_index_caching_strategy()
    );
    assert_eq!(
        FileCachingStrategy::Never,
        options.local_file_caching_strategy()
    );
    assert_eq!(1024, options.high_res_typed_object_cache_budget_bytes());
    assert_eq!(
        IndexCachingStrategy::OnDemand,
        options.portal_index_caching_strategy()
    );
    assert_eq!(
        8 * 1024 * 1024,
        options.portal_typed_object_cache_budget_bytes()
    );
}

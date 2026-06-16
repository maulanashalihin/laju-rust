use rocksdb::{DB, Options};

/// Buka koneksi RocksDB di `path` yang ditentukan.
/// Database akan dibuat otomatis kalo belum ada.
pub fn init(path: &str) -> Result<DB, rocksdb::Error> {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    let db = DB::open(&opts, path)?;
    Ok(db)
}

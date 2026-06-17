use std::sync::Arc;
use std::time::Instant;
use chrono::Utc;
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePoolOptions;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Record {
    id: String,
    data: Vec<u8>,
    created_at: i64,
}

fn make_record(key: usize, size: usize) -> Record {
    Record {
        id: format!("{:016x}", key),
        data: vec![b'X'; size],
        created_at: Utc::now().timestamp(),
    }
}

struct RocksDbHandle {
    db: Arc<DB>,
    _dir: tempfile::TempDir,
}

fn rocks_init() -> RocksDbHandle {
    let dir = tempfile::tempdir().unwrap();
    let mut opts = rocksdb::Options::default();
    opts.create_if_missing(true);
    let db = Arc::new(DB::open(&opts, dir.path()).unwrap());
    RocksDbHandle { db, _dir: dir }
}

fn rocks_write_batch(db: &DB, batch: &[Record]) {
    let mut wb = rocksdb::WriteBatch::default();
    for r in batch {
        wb.put(format!("rec:{}", r.id).as_bytes(), &serde_json::to_vec(r).unwrap());
    }
    db.write(wb).unwrap();
}

fn rocks_read_batch(db: &DB, ids: &[String]) {
    for id in ids {
        let _ = db.get(format!("rec:{}", id).as_bytes());
    }
}

fn rocks_scan(db: &DB) -> u64 {
    db.iterator(rocksdb::IteratorMode::From(b"rec:", rocksdb::Direction::Forward))
        .take_while(|r| r.as_ref().map(|(k, _)| k.starts_with(b"rec:")).unwrap_or(false))
        .count() as u64
}

fn rocks_delete_batch(db: &DB, ids: &[String]) {
    for id in ids {
        let _ = db.delete(format!("rec:{}", id).as_bytes());
    }
}

async fn sqlite_init() -> sqlx::SqlitePool {
    let pool = SqlitePoolOptions::new().max_connections(5).connect("sqlite::memory:").await.unwrap();
    sqlx::query("CREATE TABLE IF NOT EXISTS records (id TEXT PRIMARY KEY, data BLOB NOT NULL, created_at INTEGER NOT NULL)")
        .execute(&pool).await.unwrap();
    pool
}

async fn sqlite_write_batch(pool: &sqlx::SqlitePool, batch: &[Record]) {
    let mut tx = pool.begin().await.unwrap();
    for r in batch {
        sqlx::query("INSERT INTO records (id, data, created_at) VALUES (?, ?, ?)")
            .bind(&r.id).bind(&r.data).bind(r.created_at)
            .execute(&mut *tx).await.unwrap();
    }
    tx.commit().await.unwrap();
}

async fn sqlite_read_batch(pool: &sqlx::SqlitePool, ids: &[String]) {
    for id in ids {
        let _: Option<(Vec<u8>,)> = sqlx::query_as("SELECT data FROM records WHERE id = ?")
            .bind(id).fetch_optional(pool).await.unwrap();
    }
}

async fn sqlite_scan(pool: &sqlx::SqlitePool) -> u64 {
    let rows = sqlx::query_as::<_, (String,)>("SELECT id FROM records").fetch_all(pool).await.unwrap();
    rows.len() as u64
}

async fn sqlite_delete_batch(pool: &sqlx::SqlitePool, ids: &[String]) {
    for id in ids {
        sqlx::query("DELETE FROM records WHERE id = ?")
            .bind(id).execute(pool).await.unwrap();
    }
}

fn fmt(n: u128) -> String {
    if n >= 1_000_000 { format!("{:.1}M", n as f64 / 1_000_000.0) }
    else if n >= 1_000 { format!("{:.0}K", n as f64 / 1_000.0) }
    else { n.to_string() }
}

fn run(label: &str, total_ops: u64, rocks_fn: impl Fn(), sqlite_fn: impl Fn() -> u64) {
    let t = Instant::now();
    rocks_fn();
    let r_us = t.elapsed().as_micros().max(1) as u128;

    let t = Instant::now();
    let s_rows = sqlite_fn();
    let s_us = t.elapsed().as_micros().max(1) as u128;

    let r_ops = (total_ops as u128) * 1_000_000 / r_us;
    let s_ops = (s_rows as u128) * 1_000_000 / s_us;
    println!("  {:<18} {:>12} {:>12}", label, fmt(r_ops), fmt(s_ops));
}

fn seed_rocks(db: &DB, records: &[Record]) {
    for chunk in records.chunks(1000) { rocks_write_batch(db, chunk); }
}

fn seed_sqlite(pool: &sqlx::SqlitePool, records: &[Record], rt: &tokio::runtime::Runtime) {
    rt.block_on(async {
        for chunk in records.chunks(500) { sqlite_write_batch(pool, chunk).await; }
    });
}

fn main() {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║     Laju Rust — DB Benchmark                ║");
    println!("║     RocksDB vs SQLite (via sqlx)            ║");
    println!("╚══════════════════════════════════════════════╝\n");

    let configs: Vec<(&str, usize, usize)> = vec![
        ("100Kx100B", 100_000, 100),
        ("200Kx100B", 200_000, 100),
        ("100Kx1KB",  100_000, 1024),
    ];

    for &(label, count, size) in &configs {
        println!("── {} ────────────────────────────", label);
        println!("  {:<18} {:>12} {:>12}", "", "RocksDB", "SQLite");
        println!("  {}", "─".repeat(44));

        let records: Vec<Record> = (0..count).map(|i| make_record(i, size)).collect();
        let ids: Vec<String> = records.iter().map(|r| r.id.clone()).collect();
        let mut rng = rand::thread_rng();
        let read_ids: Vec<String> = (0..count).map(|_| ids.choose(&mut rng).unwrap().clone()).collect();

        let rt = tokio::runtime::Runtime::new().unwrap();

        // Write
        let rocks = rocks_init();
        let pool = rt.block_on(sqlite_init());
        run("Write", count as u64,
            || seed_rocks(&rocks.db, &records),
            || { seed_sqlite(&pool, &records, &rt); count as u64 },
        );

        // Random Read
        let rocks2 = rocks_init();
        seed_rocks(&rocks2.db, &records);
        let pool2 = rt.block_on(sqlite_init());
        seed_sqlite(&pool2, &records, &rt);
        run("Random Read", count as u64,
            || rocks_read_batch(&rocks2.db, &read_ids),
            || { rt.block_on(sqlite_read_batch(&pool2, &read_ids)); count as u64 },
        );

        // Scan
        let rocks3 = rocks_init();
        seed_rocks(&rocks3.db, &records);
        let pool3 = rt.block_on(sqlite_init());
        seed_sqlite(&pool3, &records, &rt);
        run("Scan", count as u64,
            || { rocks_scan(&rocks3.db); },
            || rt.block_on(sqlite_scan(&pool3)),
        );

        // Delete
        let rocks4 = rocks_init();
        seed_rocks(&rocks4.db, &records);
        let pool4 = rt.block_on(sqlite_init());
        seed_sqlite(&pool4, &records, &rt);
        run("Delete", count as u64,
            || rocks_delete_batch(&rocks4.db, &ids),
            || { rt.block_on(sqlite_delete_batch(&pool4, &ids)); count as u64 },
        );

        println!();
    }
}

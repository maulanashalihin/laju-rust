use std::sync::Arc;
use std::time::Instant;
use serde::{Deserialize, Serialize};

struct Handle {
    db: Arc<rocksdb::DB>,
    _dir: tempfile::TempDir,
}

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: String,
    name: String,
    value: String,
}

fn row(label: &str, n: usize, lat_us: u128) {
    let ops = if lat_us > 0 { (n as u128) * 1_000_000 / lat_us } else { n as u128 };
    let f = |n: u128| -> String {
        if n >= 1_000_000 { format!("{:.1}M", n as f64 / 1_000_000.0) }
        else if n >= 1_000 { format!("{:.0}K", n as f64 / 1_000.0) }
        else { n.to_string() }
    };
    println!("  {:<25} {:>8}   total:{:>7}μs  avg:{:>4}μs", label, f(ops), lat_us, lat_us / n as u128);
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║     Laju Rust — API Benchmark              ║");
    println!("║     RocksDB vs SQLite                      ║");
    println!("╚══════════════════════════════════════════════╝\n");

    let n = 5000;
    let items: Vec<Item> = (0..n).map(|i| Item {
        id: format!("{:016x}", i), name: format!("n-{}", i), value: "x".repeat(100),
    }).collect();
    let ids: Vec<String> = items.iter().map(|i| i.id.clone()).collect();
    let updated: Vec<Item> = (0..n).map(|i| Item {
        id: format!("{:016x}", i), name: format!("u-{}", i), value: "y".repeat(100),
    }).collect();

    // ── RocksDB ──
    {
        println!("── RocksDB ────────────────────────────────\n");
        let dir = tempfile::tempdir().unwrap();
        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);
        let db = Arc::new(rocksdb::DB::open(&opts, dir.path()).unwrap());
        let _h = Handle { db: db.clone(), _dir: dir };

        let s = Instant::now();
        for item in &items {
            let mut batch = rocksdb::WriteBatch::default();
            batch.put(format!("item:{}", item.id).as_bytes(), &serde_json::to_vec(item).unwrap());
            db.write(batch).unwrap();
        }
        row("Create (POST)", n, s.elapsed().as_micros());

        let s = Instant::now();
        for id in &ids { let _ = db.get(format!("item:{}", id).as_bytes()); }
        row("Read (GET)", n, s.elapsed().as_micros());

        let s = Instant::now();
        for item in &updated {
            let mut batch = rocksdb::WriteBatch::default();
            batch.put(format!("item:{}", item.id).as_bytes(), &serde_json::to_vec(item).unwrap());
            db.write(batch).unwrap();
        }
        row("Update (PUT)", n, s.elapsed().as_micros());

        let s = Instant::now();
        for id in &ids { let _ = db.delete(format!("item:{}", id).as_bytes()); }
        row("Delete (DELETE)", n, s.elapsed().as_micros());
        println!();
    }

    // ── SQLite ──
    {
        println!("── SQLite ────────────────────────────────\n");
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(2).connect("sqlite::memory:").await.unwrap();
        sqlx::query("CREATE TABLE items (id TEXT PRIMARY KEY, name TEXT, value TEXT)")
            .execute(&pool).await.unwrap();

        let s = Instant::now();
        for chunk in items.chunks(25) {
            let mut tx = pool.begin().await.unwrap();
            for item in chunk {
                sqlx::query("INSERT INTO items (id, name, value) VALUES (?, ?, ?)")
                    .bind(&item.id).bind(&item.name).bind(&item.value)
                    .execute(&mut *tx).await.unwrap();
            }
            tx.commit().await.unwrap();
        }
        row("Create (POST)", n, s.elapsed().as_micros());

        let s = Instant::now();
        for id in &ids {
            let _: Option<(String,)> = sqlx::query_as("SELECT id FROM items WHERE id = ?")
                .bind(id).fetch_optional(&pool).await.unwrap();
        }
        row("Read (GET)", n, s.elapsed().as_micros());

        let s = Instant::now();
        for chunk in updated.chunks(25) {
            let mut tx = pool.begin().await.unwrap();
            for item in chunk {
                sqlx::query("UPDATE items SET name = ?, value = ? WHERE id = ?")
                    .bind(&item.name).bind(&item.value).bind(&item.id)
                    .execute(&mut *tx).await.unwrap();
            }
            tx.commit().await.unwrap();
        }
        row("Update (PUT)", n, s.elapsed().as_micros());

        let s = Instant::now();
        for id in &ids {
            sqlx::query("DELETE FROM items WHERE id = ?").bind(id).execute(&pool).await.unwrap();
        }
        row("Delete (DELETE)", n, s.elapsed().as_micros());
        println!();
    }
}

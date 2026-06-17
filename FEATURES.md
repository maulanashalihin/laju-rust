# Panduan Build Fitur Baru

## Flow

```
models/  →  repositories/  →  services/  →  handlers/  →  routes/  →  ui/Pages/
   ↑          ↑               logic         HTTP           path         props
struct       query            call repo     render page    URL mapping  Inertia page
```


## Contoh: Nambahhalaman `/artikel`

### Step 1 — Model (`src/models/`)

Buat file `src/models/artikel.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artikel {
    pub id: String,
    pub judul: String,
    pub isi: String,
    pub penulis_id: String,
    pub created_at: i64,
}
```

Daftarkan di `src/models/mod.rs`:

```rust
mod user;
mod session;
mod artikel;       // tambah
pub use user::*;
pub use session::*;
pub use artikel::*;  // tambah
```

### Step 2 — Repository (`src/repositories/`)

Buat `src/repositories/artikel.rs` dengan async trait:

```rust
use async_trait::async_trait;
use std::sync::Arc;
use rocksdb::DB;
use crate::app::DbPool;
use crate::models::Artikel;

#[async_trait]
pub trait ArtikelRepository: Send + Sync {
    async fn save(&self, artikel: &Artikel) -> Result<(), String>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Artikel>, String>;
    async fn list_all(&self) -> Result<Vec<Artikel>, String>;
}

pub struct RocksDbArtikelRepository {
    pub db: Arc<DB>,
}

impl RocksDbArtikelRepository {
    pub fn new(db: Arc<DB>) -> Self { Self { db } }
}

#[async_trait]
impl ArtikelRepository for RocksDbArtikelRepository {
    async fn save(&self, artikel: &Artikel) -> Result<(), String> {
        let db = self.db.clone();
        let artikel = artikel.clone();
        tokio::task::spawn_blocking(move || {
            let key = format!("artikel:{}", artikel.id);
            let value = serde_json::to_vec(&artikel).map_err(|e| format!("Ser: {}", e))?;
            db.put(key.as_bytes(), &value).map_err(|e| format!("DB: {}", e))
        }).await.map_err(|e| format!("Join: {}", e))?
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Artikel>, String> {
        let db = self.db.clone();
        let id = id.to_string();
        tokio::task::spawn_blocking(move || {
            let key = format!("artikel:{}", id);
            match db.get(key.as_bytes()) {
                Ok(Some(data)) => Ok(Some(serde_json::from_slice(&data).map_err(|e| format!("Deser: {}", e))?)),
                Ok(None) => Ok(None),
                Err(e) => Err(format!("DB: {}", e)),
            }
        }).await.map_err(|e| format!("Join: {}", e))?
    }

    async fn list_all(&self) -> Result<Vec<Artikel>, String> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let mut all = vec![];
            let iter = db.iterator(rocksdb::IteratorMode::From(b"artikel:", rocksdb::Direction::Forward));
            for item in iter {
                match item {
                    Ok((key, value)) => {
                        if !key.starts_with(b"artikel:") { break; }
                        all.push(serde_json::from_slice(&value).map_err(|e| format!("Deser: {}", e))?);
                    }
                    Err(e) => return Err(format!("Iter: {}", e)),
                }
            }
            Ok(all)
        }).await.map_err(|e| format!("Join: {}", e))?
    }
}
```

Daftarkan di `src/repositories/mod.rs`:

```rust
pub mod user;
pub mod session;
pub mod artikel;
```

### Step 3 — Service (`src/services/`)

Buat `src/services/artikel.rs` — service terima `DbPool`, panggil repository via trait:

```rust
use crate::app::DbPool;
use crate::models::Artikel;
use crate::repositories::{ArtikelRepository, RocksDbArtikelRepository};

pub struct ArtikelService {
    repo: Box<dyn ArtikelRepository>,
}

impl ArtikelService {
    pub fn new(pool: &DbPool) -> Self {
        match pool {
            DbPool::RocksDb(db) => Self {
                repo: Box::new(RocksDbArtikelRepository::new(db.clone())),
            },
            DbPool::Sqlite(pool) => {
                // TODO: SqliteArtikelRepository impl
                todo!("SQLite not implemented for this example")
            },
        }
    }

    pub async fn buat(&self, judul: &str, isi: &str, penulis_id: &str) -> Result<Artikel, String> {
        let artikel = Artikel {
            id: uuid::Uuid::new_v4().to_string(), judul: judul.into(), isi: isi.into(),
            penulis_id: penulis_id.into(), created_at: chrono::Utc::now().timestamp(),
        };
        self.repo.save(&artikel).await?;
        Ok(artikel)
    }

    pub async fn get(&self, id: &str) -> Result<Option<Artikel>, String> {
        self.repo.find_by_id(id).await
    }

    pub async fn list_all(&self) -> Result<Vec<Artikel>, String> {
        self.repo.list_all().await
    }
}
```

Daftarkan di `src/services/mod.rs`:

```rust
pub mod auth;
pub mod artikel;
```

### Step 4 — Handler (`src/handlers/`)

Buat `src/handlers/artikel.rs`:

```rust
use std::sync::Arc;
use axum::{Extension, response::{IntoResponse, Redirect, Response}, Form};
use axum_inertia::Inertia;
use serde::Deserialize;
use serde_json::json;
use crate::app::AppState;
use crate::models::User;
use crate::services::artikel::ArtikelService;

#[derive(Deserialize)]
pub struct BuatForm {
    pub judul: String,
    pub isi: String,
}

pub async fn index(
    inertia: Inertia,
    Extension(_user): Extension<Option<User>>,
    Extension(state): Extension<Arc<AppState>>,
) -> Response {
    let svc = ArtikelService::new(&state.db);
    let artikel = svc.list_all().await.unwrap_or_default();
    inertia.render("Artikel", json!({ "artikel": artikel })).into_response()
}

pub async fn create(
    Extension(state): Extension<Arc<AppState>>,
    Form(form): Form<BuatForm>,
) -> Response {
    let svc = ArtikelService::new(&state.db);
    match svc.buat(&form.judul, &form.isi, "").await {
        Ok(_) => Redirect::to("/artikel").into_response(),
        Err(_) => Redirect::to("/artikel").into_response(),
    }
}
```

Daftarkan di `src/handlers/mod.rs`:

```rust
pub mod home;
pub mod about;
pub mod auth;
pub mod artikel;
```

### Step 5 — Route (`src/routes/mod.rs`)

```rust
.route("/artikel", get(crate::handlers::artikel::index)
    .post(crate::handlers::artikel::create))
```

### Step 6 — Page (`ui/Pages/`)

Buat `ui/Pages/Artikel.svelte`:

```svelte
<script lang="ts">
  import { usePage, Link } from '@inertiajs/svelte'
  import Layout from '../Layout.svelte'

  let page = usePage()
  let artikel = $derived((page.props as any).artikel || [])
</script>

<svelte:head>
  <title>Artikel - Laju Rust</title>
</svelte:head>

<Layout>
  <div class="max-w-4xl mx-auto space-y-6">
    <h1 class="text-3xl font-bold tracking-tight">Artikel</h1>
    {#each artikel as item}
      <div class="rounded-2xl border border-stone-200 dark:border-stone-800 bg-white dark:bg-stone-900/60 p-6">
        <h2 class="text-lg font-semibold">{item.judul}</h2>
        <p class="text-sm text-stone-600 dark:text-stone-400 mt-2">{item.isi}</p>
      </div>
    {/each}
  </div>
</Layout>
```

### Step 7 — Verify

```sh
npm run check    # svelte-check
cargo check      # Rust type check
npm run dev:all  # jalanin, test manual
```

## Checklist tiap fitur baru

- [ ] Model: struct + derive Serialize/Deserialize + daftar di `mod.rs`
- [ ] Repository: async trait + dua impl (RocksDb/Sqlite) + daftar di `src/repositories/mod.rs`
- [ ] Service: panggil trait repository via `&DbPool`, async methods + daftar di `src/services/mod.rs`
- [ ] Handler: handler function + daftar di `src/handlers/mod.rs`
- [ ] Route: mapping URL-nya di `routes/mod.rs`
- [ ] Page: Svelte file di `ui/Pages/` + layout + props sesuai handler
- [ ] Forms: `name` attribute cocok sama field di Rust struct
- [ ] Warna: pakai `stone-*` bukan `gray-*`, `amber-*` bukan `indigo-*`
- [ ] Dark mode: tiap elemen visual ada `dark:` variant
- [ ] `cargo check` + `npm run check` lulus

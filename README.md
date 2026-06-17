# Laju Rust

A modern full-stack boilerplate built with **Axum**, **Inertia.js**, and **Svelte 5**.

"Laju" means speed in Indonesian.

## Tech Stack

### Backend
- **Axum 0.8** — async HTTP framework
- **RocksDB 0.24** — embedded key-value storage (default, unggul 3-28x dari SQLite via sqlx)
- **SQLite via sqlx 0.8** — SQL backend opsional, switch via `DB_BACKEND=sqlite`
- **Sailfish 0.10** — compile-time templates for SSR pages
- **Argon2** — password hashing
- **tokio** — async runtime

### Frontend
- **Svelte 5** — reactive UI with runes
- **Inertia.js v3** — server-driven SPA (no REST API boilerplate)
- **Vite 8** — HMR and builds
- **Tailwind CSS 4** — CSS-first config, class-based dark mode
- **TypeScript 6**

## Quick Start

```bash
npm install
npm run dev:all
```

Buka `http://localhost:3000`. Frontend Vite di `:5173`, Axum backend di `:3000`.

### Commands

| Perintah | Fungsi |
|---|---|
| `npm run dev:all` | Frontend + backend bersamaan |
| `npm run dev` | Frontend Vite saja |
| `npm run dev:backend` | Backend Rust (auto-reload via cargo-watch) |
| `npm run build` | Build frontend ke `dist/` |
| `npm run check` | svelte-check (TypeScript diagnostics) |
| `cargo test` | Rust unit tests (6 tests, service + repository) |

### Production Build

```bash
npm run build                  # Frontend
cargo build --release          # Binary ~15MB
DB_BACKEND=rocksdb ./target/release/laju-rust
```

Deploy cukup binary + `dist/` folder. Set `DEV_MODE=false` untuk production.

## Project Structure

```
laju-rust/
├── src/                    # Rust backend
│   ├── bin/                # Benchmark binaries
│   │   ├── bench.rs        # DB throughput (raw ops)
│   │   ├── http_bench.rs   # HTTP endpoint (full Axum stack)
│   │   └── api_bench.rs    # API CRUD (POST/GET/PUT/DELETE)
│   ├── main.rs             # Entry point
│   ├── lib.rs              # Module registry (test crate root)
│   ├── app.rs              # AppState, DbPool enum, router builder
│   ├── config.rs           # Env vars: DEV_MODE, DB_BACKEND, DB_PATH, etc.
│   ├── db/
│   │   ├── rocksdb.rs      # RocksDB init
│   │   └── sqlite.rs       # SQLite init + migration runner
│   ├── models/             # User, Session structs (Serialize + Deserialize)
│   ├── repositories/       # Async trait + dual impl (RocksDB / SQLite)
│   ├── services/           # Business logic (panggil trait repository)
│   ├── handlers/           # HTTP handlers (home, about, auth, profile)
│   ├── middleware/          # Auth guard — resolve user from cookie
│   ├── routes/             # URL → handler mapping
│   └── inertia/            # Inertia server config (Sailfish root shell)
├── ui/                     # Svelte 5 frontend
│   ├── main.ts             # Vite entry — mounts Inertia app
│   ├── app.svelte          # Root shell
│   ├── app.css             # Global styles, grid bg, keyframes, dark variant
│   ├── Layout.svelte       # Navbar (dark toggle) + footer
│   └── Pages/              # One Svelte file per Inertia page
├── templates/
│   ├── root.stpl           # Inertia HTML shell
│   ├── home.stpl           # SSR home page
│   └── about.stpl          # SSR about page
├── migrations/             # SQLite schema migrations
├── data/                   # Runtime data (RocksDB)
├── vite.config.ts
├── sailfish.toml
└── tsconfig.json
```

## Architecture

### Layer Flow

```
Request → middleware/auth (resolve user) → routes → handlers
  → services → repositories → models → RocksDB / SQLite
```

Handler bisa pilih rendering method:

| Method | Output | Cocok untuk |
|---|---|---|
| `inertia.render("Page", json!({...}))` | SPA via `ui/Pages/` | Auth, dashboard, halaman interaktif |
| Render `.stpl` langsung | SSR HTML | Landing, SEO, halaman statis |

### Dual Rendering

Marketing pages (Home, About) adalah SSR via Sailfish templates — zero JavaScript. Halaman authenticated (Dashboard, Profile, Login, Register) pake Inertia SPA. Handler decide per-route.

### Props Contract

| Page | Handler | Props |
|---|---|---|
| Home | `home.rs` | `title`, `description` |
| About | `about.rs` | `title`, `stack[]` |
| Login | `auth.rs` | `{}` + `errors`, `flash` |
| Register | `auth.rs` | `{}` + `errors`, `flash` |
| Dashboard | `auth.rs` | `user: {name, email, role}` |
| Profile | `profile.rs` | `user: {name, email, role}` + `errors`, `flash` |

### Dual Database

Backend bisa switch RocksDB ↔ SQLite via `DB_BACKEND`. Repository layer pake `async_trait`:

```rust
let svc = AuthService::new(&state.db);
let user = svc.register(&name, &email, &pass).await?;
```

Kedua backend implement trait yang sama — handler nggak peduli database apa yang dipake.

## Auth (Login, Register)

- **Real HTML POST forms** — bukan Inertia `useForm`. Handler redirect kalo sukses, render page dengan error props kalo gagal.
- **Terminal-style UI**: dark card, 3 dots title bar, `>` prompt, mono font, emerald execute button.
- **Form field names** harus cocok sama Rust struct: `email`, `password`, `name`, `password_confirmation`, `remember`, `terms`.
- **Logout**: `<form method="POST" action="/logout">`.

## Design System ("Velocity")

| Token | Warna | Pemakaian |
|---|---|---|
| Brand gradient | `orange-600 → amber-500 → yellow-400` | Headline, CTA, logo |
| Speed accent | `amber-400/500` | Speedometer, icons, stats |
| Live / success | `emerald-400/500` | Pulse dot, session indicator |
| Neutrals | `stone-*` (BUKAN `gray-*`) | Background, card, border, text |
| Code / syntax | `cyan-400` | Terminal npx, URL |
| Error | `red-400/500` | Validasi |

**JANGAN pakai**: `indigo-*`, `violet-*`, `fuchsia-*`, `gray-*`.

## Dark Mode

- Class-based: `@custom-variant dark (&:where(.dark, .dark *))` di `app.css`.
- Inline script di `root.stpl` set `.dark` sebelum paint — zero flash.
- Toggle di navbar → simpan ke `localStorage.theme`.

## Benchmark

Semua benchmark ada di `src/bin/`. Jalanin dengan:

```sh
cargo run --bin bench --release       # DB throughput mentah
cargo run --bin http_bench --release  # HTTP endpoint (full stack)
cargo run --bin api_bench --release   # API CRUD (POST/GET/PUT/DELETE)
```

### 1. API CRUD (raw ops)

| Operasi | RocksDB | SQLite via sqlx | Gap |
|---|---|---|---|
| Create (POST) | 484K ops/s | 159K | 3x |
| Read (GET) | **2.0M** | 71K | **28x** |
| Update (PUT) | 452K | 148K | 3x |
| Delete (DELETE) | 527K | 68K | 7.7x |

### 2. HTTP Endpoint (full Axum stack)

| Endpoint | RocksDB | SQLite | Keterangan |
|---|---|---|---|
| Register (POST) | 43 ops/s | 44 ops/s | Bottleneck Argon2 (22ms) |
| Login (POST) | 88 ops/s | 85 ops/s | Bottleneck Argon2 (11ms) |
| Dashboard (GET) | **73K** | 27K | RocksDB 2.7x |
| Profile (GET) | **73K** | 28K | RocksDB 2.6x |

### 3. DB Throughput (100Kx100B)

| Workload | RocksDB | SQLite | Gap |
|---|---|---|---|
| Write | **1.6M** ops/s | 184K | 8.7x |
| Random Read | **1.2M** | 62K | 19x |
| Scan | **10.2M** | 1.8M | 5.7x |
| Delete | **567K** | 59K | 9.6x |

### Takeaway

RocksDB unggul di semua workload yang relevan buat web backend. SQLite lebih lambat di benchmark ini karena kena overhead sqlx async (pool checkout, query prepare, type mapping). Di HTTP level, gap-nya mengecil karena Argon2 dan serialisasi HTTP dominan.

## License

MIT

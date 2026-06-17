# Laju Rust

A modern full-stack boilerplate built with **Axum**, **Inertia.js**, and **Svelte 5** — designed for speed in both runtime performance and developer experience.

"Laju" means speed in Indonesian. This project lives up to the name.

## Philosophy

### Speed as a feature, not an accident

Every component in this stack was chosen for measurable performance. Axum's async routing, RocksDB's LSM-tree write throughput, Svelte's compile-time reactivity, Vite's instant HMR — none of these are aesthetic choices. They are engineering decisions validated by benchmark data.

### Embedded over client-server

RocksDB instead of PostgreSQL. Sailfish instead of a dedicated template engine server. The entire backend compiles into a single binary — no daemons, no connection pools, no Docker Compose dependency. Deployment is `scp` + `systemctl restart`.

### Layers, not frameworks

`handlers -> services -> repositories -> models` — four layers, each with one job. The handler extracts HTTP data, the service runs business logic, the repository queries the database, and the model defines the shape. No dependency injection framework, no ORM, no proc-macro magic. Just Rust functions calling Rust functions.

### SSR by default, SPA by choice

Marketing pages (Home, About) are server-rendered Sailfish templates — zero JavaScript, instant first paint. Authenticated pages (Dashboard, Profile) use Inertia.js for progressive enhancement. The handler decides per-route which rendering method to use, not the framework.

### Warm palette, not UI kit

The Velocity design system uses a warm industrial palette (orange/amber/yellow) that references the Rust programming language's brand identity — not the generic indigo/violet/fuchsia gradient that every AI-generated template ships. No design system dependency, no component library. Just Tailwind utilities and consistent spacing.

## Tech Stack

### Backend
- **Axum 0.8** — async HTTP framework with type-safe routing and Tower middleware
- **Sailfish 0.10** — compile-time templates for the Inertia root shell
- **RocksDB 0.24** — embedded persistent key-value storage
- **Argon2** — password hashing
- **tokio** — async runtime

### Frontend
- **Svelte 5** — reactive UI framework with runes (`$state`, `$derived`, `$effect`)
- **Inertia.js v3** — server-driven SPA adapter (no REST API boilerplate)
- **Vite 8** — instant HMR and production builds
- **Tailwind CSS 4** — utility-first styling with CSS-first configuration
- **TypeScript 6** — end-to-end type safety

## Features

- **Authentication** — user registration, login, session management (cookie-based)
- **Dashboard** — authenticated profile view with account management
- **Dark mode** — OS-aware with manual toggle, persisted to `localStorage`
- **"Velocity" design system** — warm Rust-engineered palette, bento grids, terminal-style auth

## Getting Started

### Prerequisites

- Rust 1.85+ (edition 2021)
- Node.js 22+
- npm

### Installation

```bash
# Clone the repository
git clone <repo-url>
cd laju-rust

# Install Node dependencies
npm install

# Run both frontend and backend in development
npm run dev:all
```

The frontend dev server starts at `http://localhost:5173` and the Axum backend at `http://localhost:3000`. Open the backend URL in your browser.

### Development Commands

```bash
npm run dev            # Frontend only (Vite dev server)
npm run dev:backend    # Backend only (cargo watch + auto-reload)
npm run dev:all        # Both simultaneously
npm run build          # Production build (Vite)
npm run check          # Svelte type-checking
```

## Production Build

```bash
# 1. Build frontend — compile Svelte + Tailwind ke dist/assets/
npm run build

# 2. Build backend — single binary (~15MB)
cargo build --release

# 3. Run
./target/release/laju-rust
```

Binary serve `dist/assets/` via `/assets` route (`ServeDir` in `app.rs`). Set env `DEV_MODE=false` untuk production. Deploy cukup binary + `dist/` folder — tidak ada dependency runtime lain.
```

## Project Structure

```
laju-rust/
├── src/                    # Rust backend
│   ├── main.rs             # Entry point — router, middleware, startup
│   ├── app.rs              # AppState, shared database handle
│   ├── config.rs           # Environment / CLI config
│   ├── handlers/           # Request handlers
│   │   ├── home.rs         # GET /
│   │   ├── about.rs        # GET /about
│   │   └── auth.rs         # Login, register, logout, dashboard
│   ├── inertia/            # Inertia server configuration
│   ├── middleware/          # Axum middleware (auth guard)
│   ├── models/             # Data models
│   ├── repositories/       # Database query layer (RocksDB via serde_json)
│   └── services/           # Business logic (panggil repository)
├── ui/                     # Svelte 5 frontend
│   ├── main.ts             # Vite entry — mounts Inertia app
│   ├── app.svelte          # Root component shell
│   ├── app.css             # Global styles, grid bg, keyframes, dark variant
│   ├── Layout.svelte       # Shared layout — navbar (dark toggle) + footer
│   └── Pages/              # One Svelte file per Inertia page
│       ├── Home.svelte     # Landing page (speedometer, stats, bento, terminal)
│       ├── About.svelte    # Tech stack showcase
│       ├── Dashboard.svelte# Authenticated profile
│       ├── Login.svelte    # Terminal-style login
│       └── Register.svelte # Terminal-style registration
├── templates/
│   └── root.stpl           # Sailfish template (HTML shell + no-flash script)
├── data/                   # Runtime data (RocksDB lives here)
├── index.html              # Vite entry HTML
├── vite.config.ts          # Vite config (svelte, inertia, tailwind plugins)
├── tsconfig.json
├── svelte.config.js
└── sailfish.toml
```

## Design System ("Velocity")

The UI follows the **Velocity** design language — a warm, industrial palette inspired by the Rust programming language's brand identity.

| Token | Value | Usage |
|---|---|---|
| Brand gradient | `orange-600 -> amber-500 -> yellow-400` | Headlines, CTAs, logo |
| Speed accent | `amber-400/500` | Speedometer, bento icons, architecture dots |
| Live status | `emerald-400/500` | Session indicators, pulse dots, success state |
| Neutrals | `stone-*` (warm grays) | Backgrounds, cards, borders |
| Code syntax | `cyan-400` | Terminal keywords, URLs |
| Error | `red-400` | Validation, error banners |

### Key Visual Elements

- **Speedometer gauge** — animated needle sweep on the landing page hero
- **Counting stats** — numbers animate from zero when scrolled into view
- **Live terminal demo** — command typing simulation with syntax highlighting
- **Bento grid** — modular card layout with varying tile sizes
- **Architecture flow** — horizontal node diagram with animated request dots
- **Grid background** — subtle engineering-blueprint texture across all pages
- **Terminal-style auth** — secure-shell aesthetic for login and registration

## Architecture

```
Browser  ->  Svelte 5  ->  Inertia.js  ->  Axum  ->  RocksDB
(Client)    (Frontend)    (Adapter)      (Backend) (Storage)
```

A request travels through five layers, each with a single responsibility. Inertia.js eliminates the need for a separate REST API by letting the server control page state directly.

## Dark Mode

Dark mode uses a class-based approach via Tailwind's `@custom-variant dark`:

1. **Before paint**: An inline script in `root.stpl` reads `localStorage` (or falls back to `prefers-color-scheme`) and adds `.dark` to `<html>` — zero flash.
2. **During render**: Tailwind's `dark:` utilities activate based on the `.dark` class.
3. **Toggle**: The navbar button toggles `.dark` on `<html>` and persists the choice to `localStorage`.

## Benchmark

4-Way benchmark: Rust+RocksDB vs Go+RocksDB vs Rust+SQLite vs Go+SQLite across 25 workload combos.

### Overall

| Rank | Combo | Total Ops/s |
|---|---|---|
| 1 | Rust+SQLite | 26,258,228 |
| 2 | **Rust+RocksDB** | **17,121,155** |
| 3 | Go+RocksDB | 8,360,302 |
| 4 | Go+SQLite | 6,457,113 |

### Head-to-Head Wins

Rust+RocksDB leads in **mixed workloads**, **random reads**, and **large value** operations — winning 16 out of 25 workload combos.

### Per-Workload

| Workload | Rust+RocksDB | Rust+SQLite | Go+RocksDB | Go+SQLite | Winner |
|---|---|---|---|---|---|
| Write 100Kx100B | 287K ops/s | 385K ops/s | 169K | 249K | Rust+SQLite |
| Write 1Mx100B | 286K | 496K | 185K | 310K | Rust+SQLite |
| Write 1Mx1KB | 181K 🏆 | 33K | 132K | 31K | **Rust+RocksDB** |
| Random Read 100Kx100B | 953K 🚀 | 291K | 334K | 108K | **Rust+RocksDB** |
| Random Read 1Mx100B | 288K | 255K | 155K | 102K | **Rust+RocksDB** |
| Random Read 1Mx1KB | 178K | 32K | 117K | 25K | **Rust+RocksDB** |
| Scan 100Kx100B | 5.9M | 10.7M 🚀 | 2.7M | 1.4M | Rust+SQLite |
| Scan 1Mx100B | 4.2M | 8.3M 🚀 | 1.7M | 1.4M | Rust+SQLite |
| Scan 1Mx1KB | 980K 🏆 | 786K | 557K | 432K | **Rust+RocksDB** |
| Mixed 100Kx100B | 333K 🏆 | 70K | 174K | 51K | **Rust+RocksDB** |
| Mixed 1Mx100B | 238K 🏆 | 42K | 126K | 36K | **Rust+RocksDB** |
| Mixed 1Mx1KB | 142K 🏆 | 15K | 83K | 13K | **Rust+RocksDB** |
| Delete 100Kx100B | 271K | 860K 🚀 | 134K | 398K | Rust+SQLite |
| Delete 1Mx100B | 323K | 705K 🚀 | 188K | 416K | Rust+SQLite |
| Delete 1Mx1KB | 320K 🏆 | 114K | 191K | 100K | **Rust+RocksDB** |

### Key Takeaways

- **Rust+RocksDB** — best for mixed workloads, random reads, and large values (1KB+). The combo used in this boilerplate.
- **Rust+SQLite** — sequential scan monster (10.7M ops/s) and excels at small writes with batch transactions. Better if your app is read-heavy on sequential data.
- **Go+RocksDB** — closest competitor to Rust but never wins. Binary size advantage (1.7MB vs 10MB).
- **Go+SQLite** — last in every category. CGo + database/sql overhead is significant.

Rust+RocksDB wins the workloads that matter most for a web backend: random reads (user lookup by ID/email), mixed read-write (auth + session + CRUD), and large value operations.

### When to Use Which Stack

| Stack | Best For | Trade-offs |
|---|---|---|
| **Rust + RocksDB** | Storage engines, time-series pipelines, high-throughput message queues, random access patterns, delete-intensive workloads | Slow dev cycle, 10MB+ binary, requires tuning (bloom filter, compaction, block cache) |
| **Rust + SQLite** | Analytics/ETL, desktop/mobile apps, report generation, small value KV, bulk delete with batch tx | Poor mixed workload, larger DB size (446MB vs 54MB for 100Kx1KB), single-writer bottleneck |
| **Go + RocksDB** | Teams already on Go needing 50-70% of Rust performance, small binary (1.7MB), fast prototyping | CGo overhead (30-50%), C memory management risks, dynamic linking dependency |
| **Go + SQLite** | Quick CRUD apps, internal tooling, small datasets | Slowest in every category — CGo + database/sql overhead |

### Decision Matrix

```
Need max performance?
  Yes → Team knows Rust?
    Yes → Rust + RocksDB (recommended)
    No  → Rust + SQLite (scan/write) or Go + RocksDB (mixed)
  No  → Need SQL queries?
    Yes → Rust + SQLite or Go + SQLite
    No  → Go + RocksDB
  Need small binary & easy deploy?
    → Go + RocksDB (1.7MB) or SQLite (single file)
```

## License

MIT

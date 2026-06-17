# Laju Rust

A modern full-stack boilerplate built with **Axum**, **Inertia.js**, and **Svelte 5** — designed for speed in both runtime performance and developer experience.

"Laju" means speed in Indonesian. This project lives up to the name.

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

## License

MIT

# Laju Rust — Agent Guidance

## Quick start

```sh
npm run dev:all            # Frontend (Vite 5173) + Backend (Axum 3000) concurrently
npm run dev                # Frontend only
npm run dev:backend        # Backend only (cargo watch + auto-reload)
npm run build              # Vite production build → dist/
npm run check              # svelte-check (TypeScript + Svelte diagnostics)
```

Always open `http://localhost:3000` (not 5173) in the browser — the backend serves the Inertia pages.

## Architecture

```
root.stpl (Sailfish) → <body> → #app → Svelte entry (main.ts)
  → createInertiaApp() → app.svelte (wrapper) → Layout.svelte (navbar/footer)
    → Page.svelte (content via {@render children()})
```

Backend layer flow: `handlers` → `services` → `repositories` → `models` (RocksDB via serde_json, or SQLite via sqlx)

**Default database adalah RocksDB** — unggul 6-19x di write, random read, delete dibanding SQLite via sqlx. SQLite tersedia sebagai opsi via `DB_BACKEND=sqlite` untuk project yang butuh SQL query.

Services now use **async trait repositories** for dual backend support:
```rust
// Service receives &DbPool, not Arc<DB>
let svc = AuthService::new(&state.db);
svc.register(&name, &email, &pass).await?;
```

`DbPool` is an enum (`RocksDb(Arc<DB>)` | `Sqlite(Pool)`) in `app.rs`. Default: `RocksDb`. Switch via `DB_BACKEND=sqlite`.

### Dual rendering: Inertia SPA vs full SSR

Handler bisa memilih rendering method kapan saja:

| Method | Template | Output |
|---|---|---|
| `inertia.render("Home", json!({...}))` | `ui/Pages/Home.svelte` | Inertia SPA — progressive enhancement, navigasi client-side |
| Render Sailfish template langsung | `templates/*.stpl` | Full SSR — halaman statis, SEO maksimal, tanpa JS |

Keduanya bisa hidup berdampingan — handler yang render SSR nggak perlu Inertia di frontend.



Key flow: `Axum handler` → `inertia.render("PageName", json!({...}))` → `Svelte page component`
consumes `usePage().props`.

## Props contract (Rust → Svelte, exactly as defined in handlers)

| Page | Handler file | Props |
|---|---|---|
| `Home` | `src/handlers/home.rs` | `title: string`, `description: string` |
| `About` | `src/handlers/about.rs` | `title: string`, `stack: string[]` |
| `Login` | `src/handlers/auth.rs` | `{}` + shared `errors`, `flash` |
| `Register` | `src/handlers/auth.rs` | `{}` + shared `errors`, `flash` |
| `Dashboard` | `src/handlers/auth.rs` | `user: {name, email, role}` |
| `Profile` | `src/handlers/profile.rs` | `user: {name, email, role}` + shared `errors`, `flash` |

Shared Inertia props (always present): `errors: Record<string, string[]>`, `flash: {success?: string, message?: string}`.

## Design system — "Velocity"

**NEVER use these color prefixes** (they are the generic Tailwind UI palette): `indigo-*`, `violet-*`, `fuchsia-*`, `gray-*`.

**ALWAYS use**:
| Token | Classes | When |
|---|---|---|
| Brand gradient | `from-orange-600 via-amber-500 to-yellow-400` | Headlines, CTAs, logo |
| Speed accent | `amber-400/500`, `bg-amber-500/10`, `border-amber-500/30` | Speedometer, icons, stats, architecture dots |
| Live / success | `emerald-400/500`, `bg-emerald-500/10`, `border-emerald-500/30` | Pulse dots, session indicators, checkmarks |
| Neutrals | `stone-*` (not `gray-*`) | All backgrounds, cards, borders, text |
| Code / syntax | `cyan-400` | Terminal keywords (`npx`), URLs |
| Error | `red-400/500`, `bg-red-500/10`, `border-red-500/30` | Validation, errors |
| Button primary | `bg-amber-600 hover:bg-amber-500` | CTAs ("Get started", "Create account") |
| Button execute | `bg-emerald-600 hover:bg-emerald-500 text-stone-950` | Login/Register terminal submit |
| Button danger | `bg-red-600 hover:bg-red-700` | Logout |

## Dark mode

- **Class-based** (not media query): `@custom-variant dark (&:where(.dark, .dark *))` in `app.css`.
- `.dark` class lives on `<html>`, set before paint by inline script in `root.stpl`, toggled by button in `Layout.svelte`, persisted to `localStorage.theme`.
- Every visual element needs both `... ` and `dark:...` variants.

## Tailwind v4 specifics

- CSS-first config: no `tailwind.config.js`. All config in `app.css`.
- Opacity: use `/50` slash syntax (`bg-stone-900/60`), NOT `bg-opacity-*`.
- Custom variants: `@custom-variant dark (...)`. Inline arbitrary variants: `supports-[backdrop-filter]:`.
- Keyframes and utility classes defined in `app.css` under `@keyframes` / `.animate-*`.

## Svelte 5 patterns (mandatory)

- Runes only: `$state`, `$derived`, `$derived.by`, `$effect`, `$props`, `{@render children()}`.
- All pages import `Layout.svelte` and wrap content: `<Layout>...</Layout>`.
- Nav links use `<Link href="/..." class={...}>` from `@inertiajs/svelte`.
- Layout's main slot: `max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8 sm:py-12`. Pages work inside this.
- Page titles via `<svelte:head><title>...</title></svelte:head>`.
- Event handlers: `onclick={fn}`, not `on:click={fn}` (Svelte 5).

## Auth forms (Login, Register)

- **Real HTML POST forms**: `<form method="POST" action="/login">` — NOT Inertia `useForm`.
- Form `name` attributes must match Rust struct fields exactly: `email`, `password`, `name`, `password_confirmation`, `remember`, `terms`.
- Submitting state: `let submitting = $state(false)` + `onsubmit={() => (submitting = true)}`.
- Handlers redirect to `/dashboard` on success, back to form on failure.
- Error feedback: from `page.props.errors`.
- **Terminal-style**: dark card (`bg-stone-950`), 3 dots title bar (`zsh — laju-rust --auth`), `>` prompt in `text-emerald-400`, mono font, `// LABEL` labels, emerald submit button with uppercase mono text.
- Logout: `<form method="POST" action="/logout">` with danger button.

## app.css global utilities

Use these classes instead of redefining keyframes:
`animate-fade-in`, `animate-slide-up`, `animate-pulse-slow`, `animate-gradient`, `animate-orbit`, `animate-blink`.

Available keyframes: `fade-in`, `slide-up`, `pulse-slow`, `gradient-shift`, `orbit`, `flow` (offset-distance), `blink`.

## Grid background

Applied on `<body>` via `@layer base` in `app.css` (48px grid, 4-5% opacity). Pages do NOT add their own background — the grid shows through. Use `bg-white/80` or `bg-stone-900/60` (with transparency) for cards so the grid is subtly visible.

## root.stpl (Sailfish template)

- Body classes: `bg-stone-50 text-stone-900 antialiased dark:bg-stone-950 dark:text-stone-100`.
- Inline script for dark mode runs before paint: reads `localStorage.theme`, falls back to `prefers-color-scheme`.
- In dev mode: CSS loaded via JS import (no `<link>`). In production: `<link rel="stylesheet" href="/assets/app.css">`.
- Inertia JSON data: `<script data-page="app" type="application/json">`.
- Template variables from Rust: `page_json`, `asset_url`, `title`, `dev_mode`.

## Animation patterns (Home.svelte)

**Counter counting**: `$effect` + `IntersectionObserver` + `requestAnimationFrame` with ease-out cubic. Create `$state` for each counter, animate from 0 to target when scrolled into view (threshold 0.3). Format large numbers with `toLocaleString()`, decimals with `toFixed(1)`.

**Terminal typing**: `$effect` with `setTimeout` cascade. Add one character every 30-70ms (randomized for realism). After command is typed, show output lines sequentially (280ms apart). Cleanup: cancel all timeouts on unmount.

**Speedometer**: SVG with `transform-origin` CSS, needle rotates via `@keyframes` (1.5s, ease-out). Needle is amber with `drop-shadow` glow.

## Known inconsistencies

- `app.svelte` uses `bg-gray-50 dark:bg-gray-950` (old palette) but it's essentially dead code — the actual body comes from `root.stpl`. If `app.svelte` is ever actually mounted, its classes should be `bg-stone-50 dark:bg-stone-950`.
- `root.stpl` inline script uses `(stored === null && prefersDark)` — note `null` not `undefined`. This is correct: `localStorage.getItem` returns `null` for missing keys.
- New module (`mod repositories`) must be registered in BOTH `main.rs` AND `lib.rs` — the binary target uses `main.rs` as crate root, while tests build from `lib.rs`.
- The Rust `register_submit` handler (line 31) redirects to `/register` when passwords don't match, without setting any flash/error message. The frontend shows no feedback for this case.
- All `mod` declarations MUST be added to BOTH `main.rs` AND `lib.rs` (binary vs test crate roots).
- Repositories use `async_trait` — methods are `async fn`. Service calls must `.await`.

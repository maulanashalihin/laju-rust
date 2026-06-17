# Notepad
<!-- Auto-managed by OMC. Manual edits preserved in MANUAL section. -->

## Priority Context
<!-- ALWAYS loaded. Keep under 500 chars. Critical discoveries only. -->

## Working Memory
<!-- Session notes. Auto-pruned after 7 days. -->

## MANUAL
<!-- User content. Never auto-pruned. -->
### 2026-06-17 09:15
Project "Laju Rust" technology decisions confirmed:
- Stack: Rust + RocksDB (embedded KV) — chosen for mixed workload + random read performance
- Benchmark shows Rust+RocksDB wins 16/25 workload combos
- Frontend: Svelte 5 + Inertia.js + Tailwind CSS 4 + Vite 8
- Design: "Velocity" warm palette (orange/amber/yellow brand, stone neutrals, emerald live indicators)
- Architecture: handlers → services → repositories → models (RocksDB via serde_json)
- Dual rendering: Inertia SPA (auth, dashboard) + SSR Sailfish (home, about)



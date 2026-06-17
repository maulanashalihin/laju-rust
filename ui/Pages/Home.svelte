<script lang="ts">
  import { onMount } from 'svelte'
  import { Link, usePage } from '@inertiajs/svelte'
  import Layout from '../Layout.svelte'

  // ─── Props (contract unchanged) ──────────────────────────────────────────
  let page = usePage()
  let title = $derived(page.props.title as string)
  let description = $derived(page.props.description as string)

  // ─── Speedometer geometry (pre-computed once) ────────────────────────────
  // Semicircular gauge in a 280×180 viewBox. Pivot sits at the bottom
  // centre of the arc. Angles are in SVG terms: 0° = +x (right),
  // 90° = +y (down), 180° = −x (left), 270° = −y (up).
  type Tick = {
    x1: number
    y1: number
    x2: number
    y2: number
    labelX: number
    labelY: number
    label: string
    anchor: 'start' | 'middle' | 'end'
  }
  const GAUGE_CX = 140
  const GAUGE_CY = 140
  const GAUGE_R = 120
  const TICK_LEN = 10
  const LABEL_R = GAUGE_R - 28
  const TICK_ANGLES = [180, 225, 270, 315, 360] as const
  const TICK_LABELS = ['0', '12K', '25K', '50K', 'MAX'] as const

  const ticks: Tick[] = TICK_ANGLES.map((angle, i) => {
    const rad = (angle * Math.PI) / 180
    const ax = GAUGE_CX + GAUGE_R * Math.cos(rad)
    const ay = GAUGE_CY + GAUGE_R * Math.sin(rad)
    const ix = GAUGE_CX + (GAUGE_R - TICK_LEN) * Math.cos(rad)
    const iy = GAUGE_CY + (GAUGE_R - TICK_LEN) * Math.sin(rad)
    const lx = GAUGE_CX + LABEL_R * Math.cos(rad)
    const ly = GAUGE_CY + LABEL_R * Math.sin(rad)
    const anchor: Tick['anchor'] =
      i === 0 ? 'end' : i === TICK_LABELS.length - 1 ? 'start' : 'middle'
    return { x1: ax, y1: ay, x2: ix, y2: iy, labelX: lx, labelY: ly, label: TICK_LABELS[i], anchor }
  })

  // Arc path: from the leftmost tick point to the rightmost, sweeping over
  // the top. Radius matches GAUGE_R so the tick endpoints land on the arc.
  const arcPath = `M ${GAUGE_CX - GAUGE_R} ${GAUGE_CY} A ${GAUGE_R} ${GAUGE_R} 0 0 1 ${GAUGE_CX + GAUGE_R} ${GAUGE_CY}`

  // ─── Live Stats counters ─────────────────────────────────────────────────
  let throughputCount = $state(0)
  let latencyCount = $state(0)
  const THROUGHPUT_TARGET = 12847
  const LATENCY_TARGET = 0.3

  let throughputEl: HTMLElement | undefined = $state()
  let latencyEl: HTMLElement | undefined = $state()

  function animateCount(
    target: number,
    duration: number,
    onUpdate: (v: number) => void
  ): () => void {
    let raf = 0
    const start = performance.now()
    const tick = (now: number) => {
      const t = Math.min((now - start) / duration, 1)
      const eased = 1 - Math.pow(1 - t, 3)
      onUpdate(eased * target)
      if (t < 1) raf = requestAnimationFrame(tick)
    }
    raf = requestAnimationFrame(tick)
    return () => cancelAnimationFrame(raf)
  }

  onMount(() => {
    const throughputObserver = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting) {
          animateCount(THROUGHPUT_TARGET, 2000, (v) => {
            throughputCount = Math.floor(v)
          })
          throughputObserver.disconnect()
        }
      },
      { threshold: 0.3 }
    )
    const latencyObserver = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting) {
          animateCount(LATENCY_TARGET, 1800, (v) => {
            latencyCount = v
          })
          latencyObserver.disconnect()
        }
      },
      { threshold: 0.3 }
    )
    if (throughputEl) throughputObserver.observe(throughputEl)
    if (latencyEl) latencyObserver.observe(latencyEl)
    return () => {
      throughputObserver.disconnect()
      latencyObserver.disconnect()
    }
  })

  // ─── Terminal typing animation ───────────────────────────────────────────
  const COMMAND = '$ npx create-laju-rust my-app'

  // Tokenised for syntax-highlighted progressive render. Whitespace tokens
  // intentionally have no colour so they fall back to terminal text colour.
  type CmdToken = { text: string; color: string }
  const commandTokens: CmdToken[] = [
    { text: '$', color: 'text-emerald-400' },
    { text: ' ', color: '' },
    { text: 'npx', color: 'text-cyan-400' },
    { text: ' ', color: '' },
    { text: 'create-laju-rust', color: 'text-white' },
    { text: ' ', color: '' },
    { text: 'my-app', color: 'text-amber-400' },
  ]
  // Cumulative start index per token — handy for slicing the typed string.
  const tokenStarts: number[] = commandTokens.reduce<number[]>((acc, t, i) => {
    acc.push(i === 0 ? 0 : acc[i - 1] + commandTokens[i - 1].text.length)
    return acc
  }, [])

  type OutputLine = { id: number; text: string; kind: 'ok' | 'info' }

  let typedCommand = $state('')
  let commandComplete = $state(false)
  let outputLines = $state<OutputLine[]>([])

  $effect(() => {
    const timeouts = new Set<ReturnType<typeof setTimeout>>()
    let cancelled = false

    const schedule = (fn: () => void, delay: number) => {
      const id = setTimeout(() => {
        timeouts.delete(id)
        if (!cancelled) fn()
      }, delay)
      timeouts.add(id)
    }

    let i = 0
    const typeNext = () => {
      if (cancelled) return
      if (i < COMMAND.length) {
        typedCommand = COMMAND.slice(0, i + 1)
        i++
        // 35–60 ms feels human without dragging on too long.
        schedule(typeNext, 35 + Math.random() * 25)
      } else {
        commandComplete = true
        runOutput()
      }
    }

    const output: ReadonlyArray<{ text: string; kind: 'ok' | 'info' }> = [
      { text: '✓ Creating project structure...', kind: 'ok' },
      { text: '✓ Installing dependencies...', kind: 'ok' },
      { text: '✓ Compiling Rust workspace...', kind: 'ok' },
      { text: '→ Dev server running at http://localhost:5173', kind: 'info' },
    ]
    const runOutput = () => {
      let j = 0
      const next = () => {
        if (cancelled) return
        if (j < output.length) {
          const line = output[j]
          outputLines = [...outputLines, { id: j, text: line.text, kind: line.kind }]
          j++
          schedule(next, 420)
        }
      }
      schedule(next, 420)
    }

    // Small lead-in so the user has a beat to read the section header.
    schedule(typeNext, 600)

    return () => {
      cancelled = true
      for (const id of timeouts) clearTimeout(id)
      timeouts.clear()
    }
  })

  // ─── Stack data ──────────────────────────────────────────────────────────
  const technologies = [
    { name: 'Axum', desc: 'Async web framework', mono: 'Ax', span: 2, long: 'Tokio-powered HTTP server with type-safe routing and middleware.' },
    { name: 'Sailfish', desc: 'Compile-time templates', mono: 'Sf', span: 1 },
    { name: 'RocksDB', desc: 'Embedded key-value store', mono: 'Rb', span: 1 },
    { name: 'Inertia.js', desc: 'Server-driven SPA', mono: 'Ij', span: 1 },
    { name: 'Svelte 5', desc: 'Reactive UI with runes', mono: 'Sv', span: 2, long: 'Compiled, tiny bundles. The fastest way to ship interactive UI.' },
    { name: 'Vite 8', desc: 'Instant HMR', mono: 'Vt', span: 1 },
    { name: 'Tailwind 4', desc: 'Zero-config CSS', mono: 'Tw', span: 2, long: 'No config file, no PostCSS dance. Just write CSS that scales.' },
    { name: 'TypeScript', desc: 'End-to-end types', mono: 'Ts', span: 2, long: 'Strict mode by default. Catch shape mismatches at compile time, not in prod.' },
  ]

  // ─── Architecture flow ───────────────────────────────────────────────────
  const flowNodes = [
    { mono: 'CLIENT', name: 'Browser', desc: 'User clicks a link', icon: 'globe' },
    { mono: 'FRONTEND', name: 'Svelte 5', desc: 'Reactive UI updates', icon: 'code' },
    { mono: 'ADAPTER', name: 'Inertia.js', desc: 'Fetches the next page', icon: 'arrows' },
    { mono: 'BACKEND', name: 'Axum', desc: 'Routes the request', icon: 'server' },
    { mono: 'STORAGE', name: 'RocksDB', desc: 'Reads/writes data', icon: 'database' },
  ] as const

  // ─── Bottom CTA copy button ──────────────────────────────────────────────
  let copied = $state(false)
  let copyTimer: ReturnType<typeof setTimeout> | undefined
  async function copyCommand() {
    try {
      await navigator.clipboard.writeText('npx create-laju-rust my-app')
      copied = true
      clearTimeout(copyTimer)
      copyTimer = setTimeout(() => {
        copied = false
      }, 1800)
    } catch {
      // Clipboard blocked — silently fail; the command is visible regardless.
    }
  }

  // ─── Architecture-flow icon paths (Lucide-style strokes) ─────────────────
  function iconPath(name: string): string {
    switch (name) {
      case 'globe':
        return 'M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18ZM3.6 9h16.8M3.6 15h16.8M12 3a14.7 14.7 0 0 1 0 18M12 3a14.7 14.7 0 0 0 0 18'
      case 'code':
        return 'm16 18 6-6-6-6M8 6l-6 6 6 6'
      case 'arrows':
        return 'M17 1l4 4-4 4M3 11V9a4 4 0 0 1 4-4h14M7 23l-4-4 4-4M21 13v2a4 4 0 0 1-4 4H3'
      case 'server':
        return 'M5 4h14a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2ZM5 14h14a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2ZM7 8h.01M7 18h.01'
      case 'database':
        return 'M12 3c5 0 9 1.3 9 3v12c0 1.7-4 3-9 3s-9-1.3-9-3V6c0-1.7 4-3 9-3ZM3 6c0 1.7 4 3 9 3s9-1.3 9-3M3 12c0 1.7 4 3 9 3s9-1.3 9-3'
      default:
        return ''
    }
  }
</script>

<svelte:head>
  <title>{title} - Laju Rust</title>
</svelte:head>

<Layout>
  <!-- ════════════════════════════════════════════════════════════════════
       Section 1 — Hero
       ════════════════════════════════════════════════════════════════════ -->
  <section class="relative min-h-[90vh] flex flex-col items-center justify-center pt-20 pb-16">
    <!-- Background orbs (blurred gradient blobs that drift) -->
    <div class="absolute inset-0 -z-10 overflow-hidden pointer-events-none" aria-hidden="true">
      <div
        class="absolute -top-20 -left-20 w-96 h-96 rounded-full bg-amber-500 opacity-20 dark:opacity-30 blur-3xl animate-orbit"
        style="animation-delay: 0s"
      ></div>
      <div
        class="absolute -top-10 -right-24 w-96 h-96 rounded-full bg-amber-500 opacity-20 dark:opacity-30 blur-3xl animate-orbit"
        style="animation-delay: -6s"
      ></div>
      <div
        class="absolute -bottom-24 -left-16 w-96 h-96 rounded-full bg-yellow-500 opacity-20 dark:opacity-30 blur-3xl animate-orbit"
        style="animation-delay: -12s"
      ></div>
      <div
        class="absolute -bottom-20 -right-20 w-96 h-96 rounded-full bg-emerald-500 opacity-15 dark:opacity-25 blur-3xl animate-orbit"
        style="animation-delay: -18s"
      ></div>
    </div>

    <div class="relative z-10 w-full max-w-5xl mx-auto text-center px-4 sm:px-0">
      <!-- Eyebrow -->
      <div
        class="inline-flex items-center gap-2 px-3 py-1.5 rounded-full text-xs font-mono uppercase tracking-widest bg-emerald-500/10 border border-emerald-500/30 text-emerald-600 dark:text-emerald-400"
      >
        <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse-slow" aria-hidden="true"></span>
        v0.1.0 · Tokio + Axum + Svelte 5
      </div>

      <!-- Headline -->
      <h1
        class="mt-8 text-6xl sm:text-7xl lg:text-8xl font-black tracking-tighter leading-[0.95] text-balance text-stone-900 dark:text-white"
      >
        Built for <span
          class="bg-gradient-to-r from-orange-500 via-amber-500 to-yellow-400 bg-clip-text text-transparent"
          >speed</span
        >.
      </h1>

      <!-- Subhead -->
      <p
        class="mt-6 text-lg sm:text-xl text-stone-600 dark:text-stone-300 max-w-2xl mx-auto text-pretty"
      >
        {description}
      </p>

      <!-- CTA group -->
      <div class="mt-10 flex flex-wrap items-center justify-center gap-4">
        <Link
          href="/register"
          class="inline-flex items-center justify-center gap-2 px-6 py-3 bg-amber-600 hover:bg-amber-700 text-white text-sm font-semibold rounded-lg transition-all shadow-sm hover:shadow-lg hover:shadow-amber-500/20 active:scale-[0.98]"
        >
          Get started
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2.5"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="M5 12h14" />
            <path d="m12 5 7 7-7 7" />
          </svg>
        </Link>
        <a
          href="/about"
          class="inline-flex items-center justify-center gap-2 px-6 py-3 bg-white dark:bg-stone-800 border border-stone-300 dark:border-stone-700 hover:bg-stone-50 dark:hover:bg-stone-700 text-stone-900 dark:text-stone-100 text-sm font-semibold rounded-lg transition-colors"
        >
          Read the docs
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2.5"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
            <path d="M14 2v6h6" />
            <path d="M9 13h6" />
            <path d="M9 17h6" />
          </svg>
        </a>
      </div>

      <!-- Speedometer centerpiece -->
      <div class="mt-16 mx-auto max-w-md relative">
        <!-- Live badge -->
        <div class="absolute top-0 right-2 sm:right-4 flex items-center gap-1.5 z-10">
          <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse-slow" aria-hidden="true"></span>
          <span class="font-mono text-[10px] uppercase tracking-widest text-emerald-600 dark:text-emerald-400">Live</span>
        </div>

        <svg
          viewBox="0 0 280 180"
          class="w-full max-w-sm mx-auto block"
          role="img"
          aria-label="Throughput gauge — 12,847 requests per second"
        >
          <!-- Arc track -->
          <path
            d={arcPath}
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            class="text-stone-700 dark:text-stone-700"
            stroke-linecap="round"
          />

          <!-- Tick marks and labels -->
          {#each ticks as tick}
            <line
              x1={tick.x1}
              y1={tick.y1}
              x2={tick.x2}
              y2={tick.y2}
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              class="text-stone-600 dark:text-stone-600"
            />
            <text
              x={tick.labelX}
              y={tick.labelY}
              text-anchor={tick.anchor}
              dominant-baseline="middle"
              class="fill-stone-500 dark:fill-stone-400 font-mono"
              style="font-size: 9px; letter-spacing: 0.05em;"
            >
              {tick.label}
            </text>
          {/each}

          <!-- Animated needle (sweeps from left to right on page load) -->
          <g class="speedometer-needle" style="filter: drop-shadow(0 0 8px rgba(16, 185, 129, 0.45));">
            <line
              x1={GAUGE_CX}
              y1={GAUGE_CY}
              x2={GAUGE_CX}
              y2={GAUGE_CY - GAUGE_R + 15}
              stroke="currentColor"
              stroke-width="3"
              stroke-linecap="round"
              class="text-emerald-500"
            />
          </g>

          <!-- Centre pivot dot -->
          <circle
            cx={GAUGE_CX}
            cy={GAUGE_CY}
            r="6"
            fill="currentColor"
            class="text-emerald-500"
            style="filter: drop-shadow(0 0 6px rgba(16, 185, 129, 0.6));"
          />
          <circle cx={GAUGE_CX} cy={GAUGE_CY} r="2" class="fill-stone-950 dark:fill-stone-950" />
        </svg>

        <p
          class="mt-2 font-mono text-xs uppercase tracking-widest text-stone-500 dark:text-stone-400"
        >
          Requests / sec
        </p>
      </div>
    </div>
  </section>

  <!-- ════════════════════════════════════════════════════════════════════
       Section 2 — Live Stats
       ════════════════════════════════════════════════════════════════════ -->
  <section class="py-20 sm:py-32">
    <div class="text-center max-w-2xl mx-auto">
      <p
        class="font-mono text-xs uppercase tracking-widest text-emerald-600 dark:text-emerald-400"
      >
        Benchmarks
      </p>
      <h2 class="mt-3 text-3xl sm:text-4xl font-bold tracking-tight text-balance text-stone-900 dark:text-white">
        Numbers that speak for themselves
      </h2>
      <p class="mt-3 text-base text-stone-600 dark:text-stone-300 text-pretty">
        Measured on a standard Linux box. Your numbers will vary. Mostly they'll
        be faster.
      </p>
    </div>

    <div class="mt-12 grid grid-cols-1 sm:grid-cols-2 gap-3 sm:gap-4">
      <!-- Throughput (counting) -->
      <div
        bind:this={throughputEl}
        class="rounded-2xl border border-stone-200 dark:border-stone-800 bg-white/80 dark:bg-stone-900/60 p-6 sm:p-8 transition-all hover:border-amber-500/50 hover:shadow-lg hover:shadow-amber-500/5"
      >
        <p
          class="font-mono text-xs uppercase tracking-widest text-stone-500 dark:text-stone-400"
        >
          Throughput
        </p>
        <p
          class="mt-3 font-mono text-4xl sm:text-5xl font-bold text-stone-900 dark:text-white tabular-nums"
        >
          {throughputCount.toLocaleString()}<span class="text-stone-400 dark:text-stone-500 text-2xl sm:text-3xl ml-1">req/s</span>
        </p>
        <p class="mt-3 text-sm text-stone-500 dark:text-stone-400">
          Concurrent connections on a single Axum worker.
        </p>
      </div>

      <!-- Latency (counting, decimal) -->
      <div
        bind:this={latencyEl}
        class="rounded-2xl border border-stone-200 dark:border-stone-800 bg-white/80 dark:bg-stone-900/60 p-6 sm:p-8 transition-all hover:border-amber-500/50 hover:shadow-lg hover:shadow-amber-500/5"
      >
        <p
          class="font-mono text-xs uppercase tracking-widest text-stone-500 dark:text-stone-400"
        >
          Latency P99
        </p>
        <p
          class="mt-3 font-mono text-4xl sm:text-5xl font-bold text-stone-900 dark:text-white tabular-nums"
        >
          {latencyCount.toFixed(1)}<span class="text-stone-400 dark:text-stone-500 text-2xl sm:text-3xl ml-1">ms</span>
        </p>
        <p class="mt-3 text-sm text-stone-500 dark:text-stone-400">
          Tail latency at the 99th percentile. Cold cache.
        </p>
      </div>

      <!-- Type Safety (static) -->
      <div
        class="rounded-2xl border border-stone-200 dark:border-stone-800 bg-white/80 dark:bg-stone-900/60 p-6 sm:p-8 transition-all hover:border-amber-500/50 hover:shadow-lg hover:shadow-amber-500/5"
      >
        <p
          class="font-mono text-xs uppercase tracking-widest text-stone-500 dark:text-stone-400"
        >
          Type Safety
        </p>
        <p
          class="mt-3 font-mono text-4xl sm:text-5xl font-bold text-stone-900 dark:text-white tabular-nums"
        >
          100<span class="text-emerald-500 dark:text-emerald-400">%</span>
        </p>
        <p class="mt-3 text-sm text-stone-500 dark:text-stone-400">
          Strict TS on the client, strict Rust on the server. Zero <code class="font-mono text-xs">any</code>.
        </p>
      </div>

      <!-- Core Deps (static) -->
      <div
        class="rounded-2xl border border-stone-200 dark:border-stone-800 bg-white/80 dark:bg-stone-900/60 p-6 sm:p-8 transition-all hover:border-amber-500/50 hover:shadow-lg hover:shadow-amber-500/5"
      >
        <p
          class="font-mono text-xs uppercase tracking-widest text-stone-500 dark:text-stone-400"
        >
          Core Deps
        </p>
        <p
          class="mt-3 font-mono text-4xl sm:text-5xl font-bold text-stone-900 dark:text-white tabular-nums"
        >
          0<span class="text-stone-400 dark:text-stone-500 text-2xl sm:text-3xl ml-1">bloat</span>
        </p>
        <p class="mt-3 text-sm text-stone-500 dark:text-stone-400">
          Nothing you didn't ask for. Every crate earns its place.
        </p>
      </div>
    </div>
  </section>

  <!-- ════════════════════════════════════════════════════════════════════
       Section 3 — Stack Bento
       ════════════════════════════════════════════════════════════════════ -->
  <section class="py-20 sm:py-32">
    <div class="text-center max-w-2xl mx-auto">
      <p
        class="font-mono text-xs uppercase tracking-widest text-emerald-600 dark:text-emerald-400"
      >
        The Stack
      </p>
      <h2 class="mt-3 text-3xl sm:text-4xl font-bold tracking-tight text-balance text-stone-900 dark:text-white">
        Every layer, chosen for speed
      </h2>
      <p class="mt-3 text-base text-stone-600 dark:text-stone-300 text-pretty">
        No bloat. No surprises. Just the tools you'd pick yourself.
      </p>
    </div>

    <div class="mt-12 grid grid-cols-2 sm:grid-cols-4 gap-3 sm:gap-4">
      {#each technologies as tech (tech.name)}
        <div
          class="group rounded-2xl border border-stone-200 dark:border-stone-800 bg-white/80 dark:bg-stone-900/60 p-5 sm:p-6 transition-all hover:border-amber-500/50 hover:shadow-lg hover:shadow-amber-500/5 hover:-translate-y-0.5"
          class:col-span-2={tech.span === 2}
        >
          <div
            class="w-10 h-10 rounded-lg bg-amber-500/10 flex items-center justify-center text-amber-600 dark:text-amber-400 mb-3 group-hover:bg-amber-500/20 transition-colors"
            aria-hidden="true"
          >
            <span class="font-mono text-sm font-bold tracking-tight">{tech.mono}</span>
          </div>
          <p class="font-semibold text-sm text-stone-900 dark:text-white">
            {tech.name}
          </p>
          <p class="text-xs text-stone-500 dark:text-stone-400 mt-0.5">
            {tech.desc}
          </p>
          {#if tech.long}
            <p class="mt-2 text-xs text-stone-600 dark:text-stone-300">
              {tech.long}
            </p>
          {/if}
        </div>
      {/each}
    </div>
  </section>

  <!-- ════════════════════════════════════════════════════════════════════
       Section 4 — Live Terminal
       ════════════════════════════════════════════════════════════════════ -->
  <section class="py-20 sm:py-32">
    <div class="text-center max-w-2xl mx-auto">
      <p
        class="font-mono text-xs uppercase tracking-widest text-emerald-600 dark:text-emerald-400"
      >
        See it in action
      </p>
      <h2 class="mt-3 text-3xl sm:text-4xl font-bold tracking-tight text-balance text-stone-900 dark:text-white">
        One command. That's it.
      </h2>
      <p class="mt-3 text-base text-stone-600 dark:text-stone-300 text-pretty">
        Scaffold, install, run. The whole loop in under thirty seconds.
      </p>
    </div>

    <div class="mt-12 max-w-3xl mx-auto">
      <div
        class="rounded-2xl border border-stone-800 bg-stone-950 shadow-2xl shadow-amber-500/10 overflow-hidden"
      >
        <!-- Title bar -->
        <div
          class="flex items-center gap-2 px-4 py-3 border-b border-stone-800 bg-stone-900/50"
        >
          <span class="w-3 h-3 rounded-full bg-red-500/80" aria-hidden="true"></span>
          <span class="w-3 h-3 rounded-full bg-yellow-500/80" aria-hidden="true"></span>
          <span class="w-3 h-3 rounded-full bg-green-500/80" aria-hidden="true"></span>
          <span class="font-mono text-xs text-stone-400 ml-2">zsh — laju-rust</span>
        </div>

        <!-- Body -->
        <div class="p-6 font-mono text-sm text-stone-300 min-h-[260px]">
          <!-- Command line (typed progressively) -->
          <div class="flex items-start gap-0 whitespace-pre-wrap break-all">
            {#each commandTokens as token, i}
              {#if typedCommand.length > tokenStarts[i]}
                {@const visible = typedCommand.slice(
                  tokenStarts[i],
                  Math.min(typedCommand.length, tokenStarts[i] + token.text.length)
                )}
                <span class={token.color}>{visible}</span>
              {/if}
            {/each}
            {#if !commandComplete}
              <span
                class="inline-block w-2 h-4 bg-current ml-0.5 animate-blink"
                aria-hidden="true"
              ></span>
            {/if}
          </div>

          <!-- Output lines (added one by one) -->
          <div class="mt-2 space-y-1">
            {#each outputLines as line (line.id)}
              {#if line.kind === 'ok'}
                <p class="text-emerald-400">{line.text}</p>
              {:else}
                <p class="text-stone-300">
                  <span class="text-amber-400">→</span>
                  Dev server running at
                  <a
                    href="http://localhost:5173"
                    class="text-amber-400 underline underline-offset-2 hover:text-amber-300"
                    >http://localhost:5173</a
                  >
                </p>
              {/if}
            {/each}

            <!-- Final prompt with blinking cursor (only after output ends) -->
            {#if commandComplete && outputLines.length >= 4}
              <p class="mt-2">
                <span class="text-emerald-400">$</span>
                <span class="text-stone-300 animate-blink" aria-hidden="true"> _</span>
              </p>
            {/if}
          </div>
        </div>
      </div>
    </div>
  </section>

  <!-- ════════════════════════════════════════════════════════════════════
       Section 5 — Architecture Flow
       ════════════════════════════════════════════════════════════════════ -->
  <section class="py-20 sm:py-32">
    <div class="text-center max-w-2xl mx-auto">
      <p
        class="font-mono text-xs uppercase tracking-widest text-emerald-600 dark:text-emerald-400"
      >
        Architecture
      </p>
      <h2 class="mt-3 text-3xl sm:text-4xl font-bold tracking-tight text-balance text-stone-900 dark:text-white">
        How a request flows
      </h2>
      <p class="mt-3 text-base text-stone-600 dark:text-stone-300 text-pretty">
        Five layers, one round trip. The whole thing takes milliseconds.
      </p>
    </div>

    <div class="mt-14 max-w-5xl mx-auto px-2 sm:px-0">
      <!-- Desktop: horizontal flow with connecting lines -->
      <div class="hidden sm:flex items-start justify-between gap-0">
        {#each flowNodes as node, i}
          <div class="flex flex-col items-center text-center flex-1 min-w-0 px-1">
            <div
              class="w-12 h-12 rounded-xl bg-stone-900/80 border border-stone-800 flex items-center justify-center text-amber-400"
              aria-hidden="true"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="1.8"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="w-6 h-6"
              >
                <path d={iconPath(node.icon)} />
              </svg>
            </div>
            <p
              class="mt-3 font-mono text-[10px] uppercase tracking-widest text-stone-400 dark:text-stone-500"
            >
              {node.mono}
            </p>
            <p class="mt-1 font-semibold text-sm text-stone-900 dark:text-white">
              {node.name}
            </p>
            <p class="mt-1 text-xs text-stone-500 dark:text-stone-400 max-w-[10rem]">
              {node.desc}
            </p>
          </div>
          {#if i < flowNodes.length - 1}
            <div
              class="relative flex-1 h-px bg-gradient-to-r from-stone-700 via-stone-600 to-stone-700 dark:from-stone-700 dark:via-stone-600 dark:to-stone-700 mx-2 mt-6 shrink-0 min-w-[1.5rem] max-w-[3rem]"
              aria-hidden="true"
            >
              <span
                class="flow-dot absolute top-1/2 -translate-y-1/2 left-0 w-2 h-2 rounded-full bg-amber-400"
                style="animation-delay: {i * 0.6}s; box-shadow: 0 0 8px rgba(99, 102, 241, 0.7);"
              ></span>
            </div>
          {/if}
        {/each}
      </div>

      <!-- Mobile: stacked vertical with connecting dots -->
      <ol class="sm:hidden flex flex-col gap-4">
        {#each flowNodes as node, i}
          <li>
            <div class="flex items-center gap-4">
              <div
                class="w-12 h-12 rounded-xl bg-stone-900/80 border border-stone-800 flex items-center justify-center text-amber-400 shrink-0"
                aria-hidden="true"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="1.8"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  class="w-6 h-6"
                >
                  <path d={iconPath(node.icon)} />
                </svg>
              </div>
              <div class="min-w-0">
                <p
                  class="font-mono text-[10px] uppercase tracking-widest text-stone-400 dark:text-stone-500"
                >
                  {node.mono}
                </p>
                <p class="mt-0.5 font-semibold text-sm text-stone-900 dark:text-white">
                  {node.name}
                </p>
                <p class="text-xs text-stone-500 dark:text-stone-400">{node.desc}</p>
              </div>
            </div>
            {#if i < flowNodes.length - 1}
              <div
                class="ml-6 my-2 w-px h-4 bg-gradient-to-b from-stone-700 to-stone-600"
                aria-hidden="true"
              ></div>
            {/if}
          </li>
        {/each}
      </ol>
    </div>
  </section>

  <!-- ════════════════════════════════════════════════════════════════════
       Section 6 — Bottom CTA
       ════════════════════════════════════════════════════════════════════ -->
  <section class="py-12 sm:py-16">
    <div
      class="rounded-2xl border border-stone-200 dark:border-stone-800 bg-gradient-to-br from-stone-900 to-stone-950 p-8 sm:p-12 text-center relative overflow-hidden"
    >
      <!-- Decorative orb -->
      <div
        class="absolute -top-20 -right-20 w-64 h-64 rounded-full bg-amber-500/20 blur-3xl pointer-events-none"
        aria-hidden="true"
      ></div>
      <div
        class="absolute -bottom-24 -left-16 w-64 h-64 rounded-full bg-yellow-500/15 blur-3xl pointer-events-none"
        aria-hidden="true"
      ></div>

      <div class="relative">
        <h2
          class="text-3xl sm:text-4xl font-bold tracking-tight text-balance text-white"
        >
          Ready to ship at the speed of Rust?
        </h2>
        <p class="mt-3 text-stone-400 text-pretty max-w-xl mx-auto">
          Scaffold, install, run. One command, zero config.
        </p>

        <!-- Command + copy -->
        <div
          class="mt-8 inline-flex items-center gap-2 px-4 py-2.5 rounded-lg bg-stone-950 border border-stone-800 font-mono text-sm max-w-full"
        >
          <span class="text-emerald-400 shrink-0">$</span>
          <span class="text-white truncate">npx create-laju-rust my-app</span>
          <button
            type="button"
            onclick={copyCommand}
            aria-label={copied ? 'Copied' : 'Copy command to clipboard'}
            class="ml-1 inline-flex items-center justify-center w-7 h-7 rounded-md text-stone-400 hover:text-white hover:bg-stone-800 transition-colors shrink-0"
          >
            {#if copied}
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2.5"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="w-3.5 h-3.5 text-emerald-400"
                aria-hidden="true"
              >
                <polyline points="20 6 9 17 4 12" />
              </svg>
            {:else}
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="w-3.5 h-3.5"
                aria-hidden="true"
              >
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
              </svg>
            {/if}
          </button>
        </div>

        <div class="mt-8">
          <Link
            href="/register"
            class="inline-flex items-center justify-center gap-2 px-6 py-3 bg-amber-600 hover:bg-amber-700 text-white text-sm font-semibold rounded-lg transition-all shadow-sm hover:shadow-lg hover:shadow-amber-500/20 active:scale-[0.98]"
          >
            Create your account
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2.5"
              stroke-linecap="round"
              stroke-linejoin="round"
              aria-hidden="true"
            >
              <path d="M5 12h14" />
              <path d="m12 5 7 7-7 7" />
            </svg>
          </Link>
        </div>
      </div>
    </div>
  </section>
</Layout>

<style>
  /* Speedometer needle: pivot at (140, 140), sweep from −90° to 70°. The
     visual hits the upper-right of the gauge on load. */
  @keyframes needle-sweep {
    from {
      transform: rotate(-90deg);
    }
    to {
      transform: rotate(70deg);
    }
  }
  .speedometer-needle {
    transform-origin: 140px 140px;
    transform-box: fill-box;
    animation: needle-sweep 1.6s cubic-bezier(0.16, 1, 0.3, 1) 0.3s both;
  }
  /* Override fill-box default for the needle — we want the SVG user
     coordinate pivot, not the bounding-box pivot. fill-box makes the
     transform-origin relative to the element's own bbox, which is what
     makes the rotation look right here. */

  /* Flow-dot travels left → right inside its parent line. The parent is
     relatively positioned so `left: 0` → `left: 100%` traces its width. */
  @keyframes flow-x {
    0% {
      left: 0%;
      opacity: 0;
    }
    15%,
    85% {
      opacity: 1;
    }
    100% {
      left: 100%;
      opacity: 0;
    }
  }
  .flow-dot {
    animation: flow-x 3s ease-in-out infinite;
  }
</style>
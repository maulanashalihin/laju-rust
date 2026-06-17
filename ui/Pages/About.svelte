<script lang="ts">
  import { usePage } from '@inertiajs/svelte'
  import Layout from '../Layout.svelte'

  let page = usePage()
  let title = $derived(page.props.title as string)
  let stack = $derived(page.props.stack as string[])

  // Split the title into a leading word and the rest. The leading word is
  // rendered with the brand gradient for visual continuity with Home.
  let titleParts = $derived.by(() => {
    const match = title.match(/^(\S+)(\s+.+)$/)
    if (match) {
      return { lead: match[1], rest: match[2] }
    }
    return { lead: title, rest: '' }
  })

  const stackMeta: Record<string, { desc: string; mono: string }> = {
    Axum: { desc: 'Async web framework', mono: 'Ax' },
    Sailfish: { desc: 'Compile-time templates', mono: 'Sf' },
    RocksDB: { desc: 'Embedded key-value store', mono: 'Rb' },
    'Inertia.js v3': { desc: 'Server-driven SPA', mono: 'Ij' },
    Svelte: { desc: 'Reactive UI framework', mono: 'Sv' },
    'Vite 8': { desc: 'Lightning build tool', mono: 'Vt' },
    'Tailwind CSS v4': { desc: 'Utility-first CSS', mono: 'Tw' },
  }
</script>

<svelte:head>
  <title>{title} - Laju Rust</title>
</svelte:head>

<Layout>
  <div class="space-y-20 sm:space-y-24">
    <header class="pt-8 sm:pt-12">
      <p
        class="text-xs text-stone-500 dark:text-stone-400 uppercase tracking-wider font-medium"
      >
        Technology
      </p>
      <h1
        class="mt-3 text-4xl sm:text-5xl lg:text-6xl font-bold tracking-tight text-balance"
      >
        <span
          class="bg-gradient-to-r from-orange-600 via-amber-500 to-yellow-500 bg-clip-text text-transparent dark:from-amber-400 dark:via-orange-400 dark:to-yellow-300"
          >{titleParts.lead}</span
        >{titleParts.rest}
      </h1>
      <p
        class="mt-5 text-lg text-stone-600 dark:text-stone-300 max-w-2xl text-pretty"
      >
        Laju Rust is a carefully curated boilerplate combining battle-tested
        Rust backend with a modern reactive frontend. Here's what powers it.
      </p>
    </header>

    <section>
      <div class="mb-6 flex items-end justify-between gap-4">
        <div>
          <p
            class="text-xs text-stone-500 dark:text-stone-400 uppercase tracking-wider font-medium"
          >
            Stack
          </p>
          <h2 class="mt-2 text-2xl sm:text-3xl font-bold tracking-tight text-balance">
            Every layer of the app
          </h2>
        </div>
        <span
          class="hidden sm:inline-flex items-center px-2.5 py-1 rounded-full text-xs font-medium bg-stone-100 dark:bg-stone-800 text-stone-600 dark:text-stone-300"
        >
          {stack.length} technologies
        </span>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3 sm:gap-4">
        {#each stack as item (item)}
          {@const meta = stackMeta[item]}
          <div
            class="group rounded-2xl border border-stone-200 dark:border-stone-800 bg-white dark:bg-stone-900 p-6 transition-all hover:shadow-md hover:-translate-y-0.5 hover:border-amber-300 dark:hover:border-amber-700"
          >
            <div
              class="w-10 h-10 rounded-lg bg-amber-50 dark:bg-amber-950/50 flex items-center justify-center text-amber-600 dark:text-amber-400 mb-3 group-hover:bg-amber-100 dark:group-hover:bg-amber-950 transition-colors"
              aria-hidden="true"
            >
              <span class="text-base font-bold">{meta?.mono ?? item.slice(0, 2)}</span>
            </div>
            <p class="font-semibold text-sm text-stone-900 dark:text-stone-100">
              {item}
            </p>
            <p class="text-xs text-stone-500 dark:text-stone-400 mt-0.5">
              {meta?.desc ?? '—'}
            </p>
          </div>
        {/each}
      </div>
    </section>

    <section>
      <div class="mb-6">
        <p
          class="text-xs text-stone-500 dark:text-stone-400 uppercase tracking-wider font-medium"
        >
          Architecture
        </p>
        <h2 class="mt-2 text-2xl sm:text-3xl font-bold tracking-tight text-balance">
          How it fits together
        </h2>
        <p class="mt-2 text-base text-stone-600 dark:text-stone-300 max-w-2xl">
          A request travels from the browser through five layers, each with a
          single responsibility.
        </p>
      </div>

      <div
        class="rounded-2xl border border-stone-200 dark:border-stone-800 bg-white dark:bg-stone-900 divide-y divide-stone-200 dark:divide-stone-800 overflow-hidden"
      >
        <div class="flex items-center gap-4 p-5">
          <div
            class="w-10 h-10 rounded-lg bg-gradient-to-br from-orange-500 to-amber-500 flex items-center justify-center text-white shrink-0 shadow-sm shadow-amber-500/20"
            aria-hidden="true"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <circle cx="12" cy="12" r="10" />
              <circle cx="12" cy="12" r="4" />
              <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10" />
              <path d="M12 2a15.3 15.3 0 0 0-4 10 15.3 15.3 0 0 0 4 10" />
            </svg>
          </div>
          <div class="flex-1 min-w-0">
            <p class="font-semibold text-stone-900 dark:text-stone-100">Browser</p>
            <p class="text-sm text-stone-500 dark:text-stone-400 mt-0.5">
              The user's window into the app.
            </p>
          </div>
          <span
            class="text-xs text-stone-500 dark:text-stone-400 uppercase tracking-wider font-medium shrink-0"
          >
            Client
          </span>
        </div>

        <div class="flex items-center gap-4 p-5">
          <div
            class="w-10 h-10 rounded-lg bg-gradient-to-br from-orange-500 to-amber-500 flex items-center justify-center text-white shrink-0 shadow-sm shadow-amber-500/20"
            aria-hidden="true"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <path d="m18 16 4-4-4-4" />
              <path d="m6 8-4 4 4 4" />
              <path d="m14.5 4-5 16" />
            </svg>
          </div>
          <div class="flex-1 min-w-0">
            <p class="font-semibold text-stone-900 dark:text-stone-100">
              Svelte 5 + Vite
            </p>
            <p class="text-sm text-stone-500 dark:text-stone-400 mt-0.5">
              Reactive UI compiled to tiny, fast bundles.
            </p>
          </div>
          <span
            class="text-xs text-stone-500 dark:text-stone-400 uppercase tracking-wider font-medium shrink-0"
          >
            Frontend
          </span>
        </div>

        <div class="flex items-center gap-4 p-5">
          <div
            class="w-10 h-10 rounded-lg bg-gradient-to-br from-orange-500 to-amber-500 flex items-center justify-center text-white shrink-0 shadow-sm shadow-amber-500/20"
            aria-hidden="true"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <path d="m17 1 4 4-4 4" />
              <path d="M3 11V9a4 4 0 0 1 4-4h14" />
              <path d="m7 23-4-4 4-4" />
              <path d="M21 13v2a4 4 0 0 1-4 4H3" />
            </svg>
          </div>
          <div class="flex-1 min-w-0">
            <p class="font-semibold text-stone-900 dark:text-stone-100">
              Inertia.js v3
            </p>
            <p class="text-sm text-stone-500 dark:text-stone-400 mt-0.5">
              Glues the SPA to the server with zero API boilerplate.
            </p>
          </div>
          <span
            class="text-xs text-stone-500 dark:text-stone-400 uppercase tracking-wider font-medium shrink-0"
          >
            Adapter
          </span>
        </div>

        <div class="flex items-center gap-4 p-5">
          <div
            class="w-10 h-10 rounded-lg bg-gradient-to-br from-orange-500 to-amber-500 flex items-center justify-center text-white shrink-0 shadow-sm shadow-amber-500/20"
            aria-hidden="true"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <rect x="2" y="3" width="20" height="8" rx="2" />
              <rect x="2" y="13" width="20" height="8" rx="2" />
              <path d="M6 7h.01" />
              <path d="M6 17h.01" />
            </svg>
          </div>
          <div class="flex-1 min-w-0">
            <p class="font-semibold text-stone-900 dark:text-stone-100">Axum</p>
            <p class="text-sm text-stone-500 dark:text-stone-400 mt-0.5">
              Async Rust web framework with type-safe routing.
            </p>
          </div>
          <span
            class="text-xs text-stone-500 dark:text-stone-400 uppercase tracking-wider font-medium shrink-0"
          >
            Backend
          </span>
        </div>

        <div class="flex items-center gap-4 p-5">
          <div
            class="w-10 h-10 rounded-lg bg-gradient-to-br from-orange-500 to-amber-500 flex items-center justify-center text-white shrink-0 shadow-sm shadow-amber-500/20"
            aria-hidden="true"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <ellipse cx="12" cy="5" rx="9" ry="3" />
              <path d="M3 5v14a9 3 0 0 0 18 0V5" />
              <path d="M3 12a9 3 0 0 0 18 0" />
            </svg>
          </div>
          <div class="flex-1 min-w-0">
            <p class="font-semibold text-stone-900 dark:text-stone-100">RocksDB</p>
            <p class="text-sm text-stone-500 dark:text-stone-400 mt-0.5">
              Embedded persistent storage with predictable latency.
            </p>
          </div>
          <span
            class="text-xs text-stone-500 dark:text-stone-400 uppercase tracking-wider font-medium shrink-0"
          >
            Storage
          </span>
        </div>
      </div>
    </section>
  </div>
</Layout>
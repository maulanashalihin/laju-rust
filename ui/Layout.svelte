<script lang="ts">
  import { onMount } from 'svelte'
  import { Link, usePage } from '@inertiajs/svelte'

  let { children } = $props()

  let page = usePage()
  let currentPath = $derived(page.url)

  // Dark mode: the .dark class lives on <html> (set by the no-flash script
  // in templates/root.stpl and toggled by the button below).
  let isDark = $state(false)

  onMount(() => {
    isDark = document.documentElement.classList.contains('dark')
  })

  function toggleTheme() {
    isDark = !isDark
    const root = document.documentElement
    if (isDark) {
      root.classList.add('dark')
      localStorage.setItem('theme', 'dark')
    } else {
      root.classList.remove('dark')
      localStorage.setItem('theme', 'light')
    }
  }

  function isActive(href: string, exact = false): boolean {
    if (exact) return currentPath === href
    return currentPath === href || currentPath.startsWith(href + '/')
  }

  function linkClass(active: boolean): string {
    const base = 'px-3 py-2 text-sm font-medium rounded-lg transition-colors'
    return active
      ? `${base} text-stone-900 dark:text-white bg-stone-100 dark:bg-stone-800/60`
      : `${base} text-stone-600 dark:text-stone-400 hover:text-stone-900 dark:hover:text-white hover:bg-stone-100/60 dark:hover:bg-stone-800/40`
  }

  let homeClass = $derived(linkClass(isActive('/', true)))
  let aboutClass = $derived(linkClass(isActive('/about')))
  let homeAriaCurrent = $derived(isActive('/', true) ? 'page' : undefined)
  let aboutAriaCurrent = $derived(isActive('/about') ? 'page' : undefined)
</script>

<div class="min-h-screen flex flex-col">
  <header
    class="sticky top-0 z-40 w-full border-b border-stone-200/80 dark:border-stone-800/80 backdrop-blur supports-[backdrop-filter]:bg-white/70 dark:supports-[backdrop-filter]:bg-stone-950/70"
  >
    <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex h-16 items-center justify-between">
        <div class="flex items-center gap-8">
          <Link href="/" class="flex items-center gap-2.5" aria-label="Laju Rust home">
            <span
              class="inline-flex items-center justify-center w-8 h-8 rounded-lg bg-gradient-to-br from-orange-500 via-amber-500 to-yellow-400 shadow-sm shadow-amber-500/20"
              aria-hidden="true"
            >
              <svg viewBox="0 0 24 24" class="w-4 h-4 text-white" fill="currentColor" aria-hidden="true">
                <path d="M13 2L4 14h7l-2 8 9-12h-7l2-8z" />
              </svg>
            </span>
            <span
              class="text-base font-semibold tracking-tight bg-gradient-to-r from-orange-600 via-amber-500 to-yellow-500 bg-clip-text text-transparent dark:from-amber-400 dark:via-orange-400 dark:to-yellow-300"
            >
              Laju Rust
            </span>
          </Link>
          <nav class="flex items-center gap-1" aria-label="Primary">
            <Link href="/" class={homeClass} aria-current={homeAriaCurrent}>Home</Link>
            <Link href="/about" class={aboutClass} aria-current={aboutAriaCurrent}>About</Link>
          </nav>
        </div>
        <button
          type="button"
          onclick={toggleTheme}
          aria-label={isDark ? 'Switch to light mode' : 'Switch to dark mode'}
          title={isDark ? 'Switch to light mode' : 'Switch to dark mode'}
          class="inline-flex items-center justify-center w-9 h-9 rounded-lg text-stone-600 dark:text-stone-400 hover:text-stone-900 dark:hover:text-white hover:bg-stone-100 dark:hover:bg-stone-800/60 transition-colors"
        >
          {#if isDark}
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="w-4 h-4"
              aria-hidden="true"
            >
              <circle cx="12" cy="12" r="4" />
              <path d="M12 2v2" />
              <path d="M12 20v2" />
              <path d="m4.93 4.93 1.41 1.41" />
              <path d="m17.66 17.66 1.41 1.41" />
              <path d="M2 12h2" />
              <path d="M20 12h2" />
              <path d="m6.34 17.66-1.41 1.41" />
              <path d="m19.07 4.93-1.41 1.41" />
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
              class="w-4 h-4"
              aria-hidden="true"
            >
              <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" />
            </svg>
          {/if}
        </button>
      </div>
    </div>
  </header>

  <main class="flex-1 w-full">
    <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8 sm:py-12">
      {@render children()}
    </div>
  </main>

  <footer class="border-t border-stone-200 dark:border-stone-800 py-8 mt-auto">
    <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex flex-col sm:flex-row items-center justify-between gap-3">
        <div class="flex items-center gap-2.5">
          <span
            class="inline-flex items-center justify-center w-6 h-6 rounded-md bg-gradient-to-br from-orange-500 via-amber-500 to-yellow-400"
            aria-hidden="true"
          >
            <svg viewBox="0 0 24 24" class="w-3 h-3 text-white" fill="currentColor" aria-hidden="true">
              <path d="M13 2L4 14h7l-2 8 9-12h-7l2-8z" />
            </svg>
          </span>
          <span class="text-sm font-medium text-stone-700 dark:text-stone-300">Laju Rust</span>
        </div>
        <p class="text-sm text-stone-500 dark:text-stone-400 text-center sm:text-right">
          Built with Axum + Inertia.js + Svelte 5 + Tailwind CSS 4
        </p>
      </div>
    </div>
  </footer>
</div>

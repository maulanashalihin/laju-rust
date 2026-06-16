<script lang="ts">
  import { Link } from '@inertiajs/svelte'
  let { children } = $props()
</script>

<div class="min-h-screen flex flex-col">
  <!-- Navbar -->
  <nav class="bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-800">
    <div class="max-w-4xl mx-auto px-4 py-4 flex items-center gap-6">
      <Link href="/" class="text-lg font-bold text-indigo-600 dark:text-indigo-400">
        Laju Rust
      </Link>
      <div class="flex gap-4">
        <Link href="/" class="text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors">
          Home
        </Link>
        <Link href="/about" class="text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors">
          About
        </Link>
      </div>
    </div>
  </nav>

  <!-- Page content -->
  <main class="flex-1 max-w-4xl mx-auto px-4 py-8 w-full">
    {@render children()}
  </main>

  <!-- Footer -->
  <footer class="border-t border-gray-200 dark:border-gray-800 py-6 text-center text-sm text-gray-500">
    Built with Axum + Inertia.js + Svelte
  </footer>
</div>

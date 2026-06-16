<script lang="ts">
  import { page } from '@inertiajs/svelte'
  import Layout from '../Layout.svelte'

  let title = $derived($page.props.title as string)
  let description = $derived($page.props.description as string)
</script>

<svelte:head>
  <title>{title} - Laju Rust</title>
</svelte:head>

<Layout>
  <div class="space-y-8">
    <!-- Hero -->
    <div class="text-center py-12">
      <h1 class="text-4xl font-bold tracking-tight mb-4">{title}</h1>
      <p class="text-lg text-gray-600 dark:text-gray-400 max-w-2xl mx-auto">
        {description}
      </p>
    </div>

    <!-- Stack Cards -->
    <div class="grid grid-cols-2 md:grid-cols-3 gap-4">
      {#each ['Axum', 'Sailfish', 'RocksDB', 'Inertia.js', 'Svelte', 'Vite', 'Tailwind CSS'] as tech}
        <div class="bg-white dark:bg-gray-900 rounded-xl p-4 border border-gray-200 dark:border-gray-800 shadow-sm">
          <p class="font-medium text-center">{tech}</p>
        </div>
      {/each}
    </div>

    <!-- Auth CTAs -->
    <div class="text-center mt-8 space-x-4">
      <a href="/register"
        class="inline-block py-2 px-6 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-lg transition-colors">
        Get Started
      </a>
      <a href="/login"
        class="inline-block py-2 px-6 border border-gray-300 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-800 font-medium rounded-lg transition-colors">
        Login
      </a>
    </div>
  </div>
</Layout>

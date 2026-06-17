<script lang="ts">
  import { usePage } from '@inertiajs/svelte'

  let { children } = $props()
</script>

<svelte:head>
  <title>{usePage().props.title || 'Laju Rust'}</title>
</svelte:head>

<div class="min-h-screen bg-gray-50 text-gray-900 dark:bg-gray-950 dark:text-gray-100">
  {@render children()}
</div>

<script lang="ts">
  import { page } from '@inertiajs/svelte'
  import Layout from '../Layout.svelte'

  let email = $state('')
  let password = $state('')

  let errors = $derived(($page.props as any).errors || {})
</script>

<svelte:head>
  <title>Login - Laju Rust</title>
</svelte:head>

<Layout>
  <div class="max-w-md mx-auto mt-12">
    <h1 class="text-2xl font-bold mb-6">Login</h1>

    {#if Object.keys(errors).length > 0}
      <div class="bg-red-50 dark:bg-red-950 border border-red-200 dark:border-red-800 rounded-lg p-4 mb-4 text-sm text-red-700 dark:text-red-300">
        {#each Object.values(errors) as err}
          <p>{err as string}</p>
        {/each}
      </div>
    {/if}

    <form method="POST" action="/login" class="space-y-4">
      <div>
        <label for="email" class="block text-sm font-medium mb-1">Email</label>
        <input id="email" name="email" type="email" required
          bind:value={email}
          class="w-full px-3 py-2 border border-gray-300 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-900">
      </div>
      <div>
        <label for="password" class="block text-sm font-medium mb-1">Password</label>
        <input id="password" name="password" type="password" required
          bind:value={password}
          class="w-full px-3 py-2 border border-gray-300 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-900">
      </div>
      <button type="submit"
        class="w-full py-2 px-4 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-lg transition-colors">
        Login
      </button>
      <p class="text-center text-sm text-gray-500">
        Don't have an account? <a href="/register" class="text-indigo-600 hover:underline">Register</a>
      </p>
    </form>
  </div>
</Layout>

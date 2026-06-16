<script lang="ts">
  import { page } from '@inertiajs/svelte'
  import Layout from '../Layout.svelte'

  let user = $derived(($page.props as any).user || {})
</script>

<svelte:head>
  <title>Dashboard - Laju Rust</title>
</svelte:head>

<Layout>
  <div class="max-w-2xl mx-auto mt-12">
    <h1 class="text-3xl font-bold mb-6">Dashboard</h1>

    <div class="bg-white dark:bg-gray-900 rounded-xl p-6 border border-gray-200 dark:border-gray-800">
      <h2 class="text-lg font-semibold mb-4">Welcome, {user.name}!</h2>

      <div class="space-y-3 text-sm">
        <div class="flex items-center gap-2">
          <span class="text-gray-500 w-20">Name:</span>
          <span class="font-medium">{user.name}</span>
        </div>
        <div class="flex items-center gap-2">
          <span class="text-gray-500 w-20">Email:</span>
          <span class="font-medium">{user.email}</span>
        </div>
        <div class="flex items-center gap-2">
          <span class="text-gray-500 w-20">Role:</span>
          <span class="font-medium capitalize">{user.role}</span>
        </div>
      </div>

      <form method="POST" action="/logout" class="mt-6">
        <button type="submit"
          class="py-2 px-4 bg-red-600 hover:bg-red-700 text-white text-sm font-medium rounded-lg transition-colors">
          Logout
        </button>
      </form>
    </div>
  </div>
</Layout>

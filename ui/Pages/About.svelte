<script lang="ts">
  import { page } from '@inertiajs/svelte'
  import Layout from '../Layout.svelte'

  let title = $derived($page.props.title as string)
  let stack = $derived($page.props.stack as string[])
</script>

<Layout>
  <div class="space-y-8">
    <h1 class="text-3xl font-bold tracking-tight">{title}</h1>
    
    <div class="bg-white dark:bg-gray-900 rounded-xl p-6 border border-gray-200 dark:border-gray-800">
      <h2 class="text-lg font-semibold mb-4">Tech Stack</h2>
      <ul class="space-y-2">
        {#each stack as item}
          <li class="flex items-center gap-2">
            <span class="w-2 h-2 rounded-full bg-indigo-500"></span>
            {item}
          </li>
        {/each}
      </ul>
    </div>
  </div>
</Layout>

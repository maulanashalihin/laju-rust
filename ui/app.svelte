<script lang="ts">
  import { page } from '@inertiajs/svelte'
  let { children } = $props()
</script>

<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width" />
    <title>{$page.props.title || 'Laju Rust'}</title>
  </head>
  <body class="min-h-screen bg-gray-50 dark:bg-gray-950 text-gray-900 dark:text-gray-100">
    {@render children()}
  </body>
</html>

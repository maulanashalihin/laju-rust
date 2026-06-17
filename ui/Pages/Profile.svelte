<script lang="ts">
  import { usePage, Link } from '@inertiajs/svelte'
  import Layout from '../Layout.svelte'

  let page = usePage()
  let user = $derived((page.props as any).user || {})
  let initialName = $derived(user.name || '')
  let initialEmail = $derived(user.email || '')
  let errors: Record<string, string[]> = $derived((page.props as any).errors || {})
  let flash: { success?: string; message?: string } | null = $derived(
    (page.props as any).flash || null
  )

  let name = $state(initialName)
  let email = $state(initialEmail)
  let submitting = $state(false)

  function fieldError(field: string): string | null {
    const v = errors[field]
    return Array.isArray(v) && v.length > 0 ? v[0] : null
  }

  function hasError(field: string): boolean {
    return fieldError(field) !== null
  }
</script>

<svelte:head>
  <title>Profile - Laju Rust</title>
</svelte:head>

<Layout>
  <div class="max-w-2xl mx-auto space-y-8">
    <div class="flex items-center justify-between">
      <div>
        <p class="text-xs font-mono uppercase tracking-wider text-amber-600 dark:text-amber-400">Settings</p>
        <h1 class="mt-1 text-3xl sm:text-4xl font-bold tracking-tight text-stone-900 dark:text-stone-100 text-balance">Edit Profile</h1>
        <p class="mt-1 text-sm text-stone-500 dark:text-stone-400">Update your account information</p>
      </div>
      <Link
        href="/dashboard"
        class="inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-mono font-medium rounded-lg text-stone-600 dark:text-stone-400 hover:text-stone-900 dark:hover:text-white border border-stone-300 dark:border-stone-700 hover:bg-stone-100 dark:hover:bg-stone-800 transition-colors"
      >
        <svg viewBox="0 0 24 24" class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <path d="M19 12H5M12 19l-7-7 7-7"/>
        </svg>
        Back
      </Link>
    </div>

    {#if flash?.success}
      <div class="flex items-start gap-2.5 rounded-lg border bg-emerald-950/50 border-emerald-500/30 p-3 text-sm text-emerald-300 font-mono">
        <span class="text-emerald-400 select-none" aria-hidden="true">$</span>
        <span>{flash.success}</span>
      </div>
    {/if}

    <div class="rounded-2xl border border-stone-200 dark:border-stone-800 bg-white dark:bg-stone-900/60 p-6 sm:p-8">
      <form method="POST" action="/profile" class="space-y-5" onsubmit={() => (submitting = true)}>
        <div>
          <label for="name" class="block text-sm font-medium text-stone-700 dark:text-stone-300 mb-1.5">Name</label>
          <input
            id="name" name="name" type="text" required
            bind:value={name}
            class="w-full px-3.5 py-2.5 text-sm border rounded-lg bg-white dark:bg-stone-900 text-stone-900 dark:text-stone-100 placeholder-stone-400 dark:placeholder-stone-500 focus:outline-none focus:ring-2 focus:ring-amber-500/50 focus:border-amber-500/50 transition {hasError('name') ? 'border-red-400 dark:border-red-700' : 'border-stone-300 dark:border-stone-700'}"
          />
          {#if fieldError('name')}
            <p class="mt-1 text-xs text-red-500">{fieldError('name')}</p>
          {/if}
        </div>

        <div>
          <label for="email" class="block text-sm font-medium text-stone-700 dark:text-stone-300 mb-1.5">Email</label>
          <input
            id="email" name="email" type="email" required
            bind:value={email}
            class="w-full px-3.5 py-2.5 text-sm border rounded-lg bg-white dark:bg-stone-900 text-stone-900 dark:text-stone-100 placeholder-stone-400 dark:placeholder-stone-500 focus:outline-none focus:ring-2 focus:ring-amber-500/50 focus:border-amber-500/50 transition {hasError('email') ? 'border-red-400 dark:border-red-700' : 'border-stone-300 dark:border-stone-700'}"
          />
          {#if fieldError('email')}
            <p class="mt-1 text-xs text-red-500">{fieldError('email')}</p>
          {/if}
        </div>

        <div class="flex items-center gap-3 pt-2">
          <button
            type="submit"
            disabled={submitting}
            class="inline-flex items-center justify-center gap-2 px-5 py-2.5 bg-amber-600 hover:bg-amber-500 text-white text-sm font-semibold rounded-lg transition-all shadow-sm hover:shadow-lg hover:shadow-amber-500/20 active:scale-[0.98] disabled:opacity-60 disabled:cursor-not-allowed"
          >
            {#if submitting}
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4 animate-spin" aria-hidden="true">
                <path d="M21 12a9 9 0 1 1-6.219-8.56" />
              </svg>
              <span>Saving...</span>
            {:else}
              <span>Save changes</span>
            {/if}
          </button>
          <Link
            href="/dashboard"
            class="inline-flex items-center gap-1.5 px-5 py-2.5 text-sm font-medium rounded-lg border border-stone-300 dark:border-stone-700 text-stone-700 dark:text-stone-300 hover:bg-stone-100 dark:hover:bg-stone-800 transition-colors"
          >
            Cancel
          </Link>
        </div>
      </form>
    </div>
  </div>
</Layout>

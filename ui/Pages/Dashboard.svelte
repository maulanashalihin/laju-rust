<script lang="ts">
  import { usePage, Link } from '@inertiajs/svelte'
  import Layout from '../Layout.svelte'

  let page = usePage()
  // The Rust dashboard handler attaches { user: { name, email, role } } to
  // page.props (see src/handlers/auth.rs). The Inertia props bag is untyped,
  // so we keep the existing `as any` cast to read the payload.
  let user = $derived((page.props as any).user || {})

  let initials = $derived(
    (user.name || '?')
      .split(/\s+/)
      .map((p: string) => p[0])
      .join('')
      .slice(0, 2)
      .toUpperCase(),
  )

  let capitalizedRole = $derived(
    user.role ? user.role.charAt(0).toUpperCase() + user.role.slice(1) : '',
  )
</script>

<svelte:head>
  <title>Dashboard - Laju Rust</title>
</svelte:head>

<Layout>
  <div class="max-w-4xl mx-auto space-y-8">
    <div
      class="relative rounded-2xl overflow-hidden border border-stone-200 dark:border-stone-800 bg-gradient-to-br from-orange-50 via-white to-amber-50 dark:from-orange-950/40 dark:via-stone-900 dark:to-amber-950/40 p-6 sm:p-8"
    >
      <div
        class="absolute -top-16 -right-16 w-56 h-56 rounded-full bg-amber-400/20 dark:bg-amber-500/10 blur-3xl pointer-events-none"
        aria-hidden="true"
      ></div>
      <div class="relative">
        <h1 class="text-3xl sm:text-4xl font-bold tracking-tight text-balance">
          Welcome back,
          <span
            class="bg-gradient-to-r from-orange-600 via-amber-500 to-yellow-500 bg-clip-text text-transparent dark:from-amber-400 dark:via-orange-400 dark:to-yellow-300"
          >
            {user.name}
          </span>
        </h1>
        <p class="mt-2 text-sm text-stone-600 dark:text-stone-300">
          Here's your account overview
        </p>
      </div>
    </div>

    <div
      class="rounded-2xl border border-stone-200 dark:border-stone-800 bg-white dark:bg-stone-900 p-6 sm:p-8"
    >
      <div class="flex items-start gap-5">
        <div
          class="w-16 h-16 rounded-full bg-gradient-to-br from-orange-500 to-amber-600 text-white text-xl font-semibold flex items-center justify-center shadow-lg shadow-amber-500/20 flex-shrink-0"
          aria-hidden="true"
        >
          {initials}
        </div>
        <div class="min-w-0 flex-1">
          <div class="flex items-center gap-2.5 flex-wrap">
            <h2 class="text-lg font-semibold text-stone-900 dark:text-white">
              {user.name}
            </h2>
            <span
              class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium bg-amber-50 dark:bg-amber-950/50 text-amber-700 dark:text-amber-300 border border-amber-200 dark:border-amber-800"
            >
              <svg
                viewBox="0 0 24 24"
                class="w-3 h-3"
                fill="none"
                stroke="currentColor"
                stroke-width="2.5"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
              >
                <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
              </svg>
              {capitalizedRole}
            </span>
          </div>
          <p class="mt-1 text-sm text-stone-500 dark:text-stone-400 truncate">
            {user.email}
          </p>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
      <div
        class="rounded-xl border border-stone-200 dark:border-stone-800 bg-white dark:bg-stone-900 p-4"
      >
        <p
          class="text-xs text-stone-500 dark:text-stone-400 uppercase tracking-wider font-medium"
        >
          Name
        </p>
        <p class="mt-2 text-sm font-medium text-stone-900 dark:text-white truncate">
          {user.name}
        </p>
      </div>
      <div
        class="rounded-xl border border-stone-200 dark:border-stone-800 bg-white dark:bg-stone-900 p-4"
      >
        <p
          class="text-xs text-stone-500 dark:text-stone-400 uppercase tracking-wider font-medium"
        >
          Email
        </p>
        <p class="mt-2 text-sm font-medium text-stone-900 dark:text-white truncate">
          {user.email}
        </p>
      </div>
      <div
        class="rounded-xl border border-stone-200 dark:border-stone-800 bg-white dark:bg-stone-900 p-4"
      >
        <p
          class="text-xs text-stone-500 dark:text-stone-400 uppercase tracking-wider font-medium"
        >
          Account Type
        </p>
        <div class="mt-2 flex items-center gap-2">
          <svg
            viewBox="0 0 24 24"
            class="w-4 h-4 text-amber-500 dark:text-amber-400"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
          </svg>
          <span class="text-sm font-medium text-stone-900 dark:text-white">
            {capitalizedRole}
          </span>
        </div>
      </div>
      <div
        class="rounded-xl border border-stone-200 dark:border-stone-800 bg-white dark:bg-stone-900 p-4"
      >
        <p
          class="text-xs text-stone-500 dark:text-stone-400 uppercase tracking-wider font-medium"
        >
          Status
        </p>
        <div class="mt-2 flex items-center gap-2">
          <span class="relative flex h-2.5 w-2.5" aria-hidden="true">
            <span
              class="absolute inline-flex h-full w-full animate-ping rounded-full bg-emerald-400 opacity-75"
            ></span>
            <span
              class="relative inline-flex rounded-full h-2.5 w-2.5 bg-emerald-500"
            ></span>
          </span>
          <span class="text-sm font-medium text-stone-900 dark:text-white">Active</span>
        </div>
      </div>
    </div>

    <div
      class="rounded-2xl border border-stone-200 dark:border-stone-800 bg-white dark:bg-stone-900 p-6 sm:p-8"
    >
      <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div>
          <h3 class="text-lg font-semibold text-stone-900 dark:text-white">Account</h3>
          <p class="mt-1 text-sm text-stone-500 dark:text-stone-400">
            Manage your profile settings
          </p>
          <Link
            href="/profile"
            class="inline-flex items-center gap-2 px-5 py-2.5 mt-3 bg-amber-600 hover:bg-amber-500 text-white text-sm font-semibold rounded-lg transition-all shadow-sm hover:shadow-lg hover:shadow-amber-500/20 active:scale-[0.98]"
          >
            <svg viewBox="0 0 24 24" class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
              <path d="M17 3a2.85 2.85 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"/>
            </svg>
            Edit profile
          </Link>
        </div>
        <form method="POST" action="/logout">
          <button
            type="submit"
            class="inline-flex items-center justify-center gap-2 px-5 py-2.5 bg-red-600 hover:bg-red-700 text-white text-sm font-medium rounded-lg transition-all shadow-sm hover:shadow active:scale-[0.98]"
          >
            <svg
              viewBox="0 0 24 24"
              class="w-4 h-4"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              aria-hidden="true"
            >
              <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
              <polyline points="16 17 21 12 16 7" />
              <line x1="21" y1="12" x2="9" y2="12" />
            </svg>
            Log out
          </button>
        </form>
      </div>
    </div>
  </div>
</Layout>

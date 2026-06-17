<script lang="ts">
  import { usePage, Link } from '@inertiajs/svelte'
  import Layout from '../Layout.svelte'

  let name = $state('')
  let email = $state('')
  let password = $state('')
  let password_confirmation = $state('')
  let terms = $state(false)
  let showPassword = $state(false)
  let showPasswordConfirmation = $state(false)
  let submitting = $state(false)

  let page = usePage()
  let errors: Record<string, string[]> = $derived((page.props as any).errors || {})
  let flash: { success?: string; message?: string } | null = $derived(
    (page.props as any).flash || null
  )

  function fieldError(field: string): string | null {
    const v = errors[field]
    return Array.isArray(v) && v.length > 0 ? v[0] : null
  }

  function hasError(field: string): boolean {
    return fieldError(field) !== null
  }

  function inputClass(field: string, extraPadRight = false): string {
    const base = 'w-full pl-8 py-2.5 font-mono text-sm border rounded-lg bg-stone-900 dark:bg-stone-900 text-stone-100 dark:text-stone-100 placeholder-stone-600 dark:placeholder-stone-600 focus:outline-none focus:ring-2 focus:ring-emerald-500/50 focus:border-emerald-500/50 transition border-stone-800 dark:border-stone-800'
    const padding = extraPadRight ? 'pr-10' : 'pr-3.5'
    const border = hasError(field) ? 'border-red-500/50 focus:ring-red-500/50 focus:border-red-500/50' : ''
    return `${base} ${padding} ${border}`.trim()
  }
</script>

<svelte:head>
  <title>Create account - Laju Rust</title>
</svelte:head>

<Layout>
  <div class="min-h-[calc(100vh-12rem)] flex items-center justify-center py-12 px-4 w-full">
    <div class="w-full max-w-md">
      <div class="rounded-2xl border border-stone-800 bg-stone-950 overflow-hidden shadow-2xl shadow-amber-500/10">
        <div class="flex items-center gap-2 px-4 py-3 border-b border-stone-800 bg-stone-900/50">
          <span class="w-3 h-3 rounded-full bg-red-500" aria-hidden="true"></span>
          <span class="w-3 h-3 rounded-full bg-yellow-500" aria-hidden="true"></span>
          <span class="w-3 h-3 rounded-full bg-emerald-500" aria-hidden="true"></span>
          <span class="font-mono text-xs text-stone-400 ml-2">zsh — laju-rust --auth register</span>
        </div>
        <div class="p-6 sm:p-8">
          <div class="flex items-center gap-2">
            <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse-slow" aria-hidden="true"></span>
            <span class="font-mono text-xs text-emerald-400 uppercase tracking-widest">CREATE NEW IDENTITY</span>
          </div>
          <h1 class="mt-4 text-2xl sm:text-3xl font-bold tracking-tight text-balance text-white dark:text-white">Create your account</h1>
          <p class="mt-2 text-sm text-stone-400 dark:text-stone-400">Initialize a new session. Choose your credentials.</p>

          {#if flash && (flash.success || flash.message)}
            <div class="mt-6 flex items-start gap-2.5 rounded-lg border bg-emerald-950/50 border-emerald-500/30 p-3 text-sm text-emerald-300 font-mono">
              <span class="text-emerald-400 select-none" aria-hidden="true">$</span>
              <span>{flash.success || flash.message}</span>
            </div>
          {/if}

          {#if Object.keys(errors).length > 0}
            <div class="mt-6 flex items-start gap-2.5 rounded-lg border bg-red-950/50 border-red-500/30 p-3 text-sm text-red-300 font-mono">
              <div class="flex-1">
                <p class="font-bold">// ERROR: Could not create account</p>
                <div class="mt-1.5 space-y-0.5">
                  {#each Object.values(errors).flat() as err}
                    <p>&gt; {err}</p>
                  {/each}
                </div>
              </div>
            </div>
          {/if}

          <form
            method="POST"
            action="/register"
            class="mt-6 space-y-4"
            onsubmit={() => (submitting = true)}
          >
            <div>
              <label for="name" class="block font-mono text-xs uppercase tracking-widest text-stone-500 dark:text-stone-400 mb-2">// IDENTITY</label>
              <div class="relative flex items-center">
                <span class="absolute left-3 font-mono text-emerald-400 text-sm pointer-events-none" aria-hidden="true">&gt;</span>
                <input
                  id="name"
                  name="name"
                  type="text"
                  autocomplete="name"
                  required
                  bind:value={name}
                  placeholder="Jane Doe"
                  class={inputClass('name', false)}
                />
              </div>
              {#if fieldError('name')}
                <p class="mt-1.5 font-mono text-xs text-red-400">&gt; {fieldError('name')}</p>
              {/if}
            </div>

            <div>
              <label for="email" class="block font-mono text-xs uppercase tracking-widest text-stone-500 dark:text-stone-400 mb-2">// EMAIL</label>
              <div class="relative flex items-center">
                <span class="absolute left-3 font-mono text-emerald-400 text-sm pointer-events-none" aria-hidden="true">&gt;</span>
                <input
                  id="email"
                  name="email"
                  type="email"
                  autocomplete="email"
                  required
                  bind:value={email}
                  placeholder="user@example.com"
                  class={inputClass('email', false)}
                />
              </div>
              {#if fieldError('email')}
                <p class="mt-1.5 font-mono text-xs text-red-400">&gt; {fieldError('email')}</p>
              {/if}
            </div>

            <div>
              <label for="password" class="block font-mono text-xs uppercase tracking-widest text-stone-500 dark:text-stone-400 mb-2">// PASSWORD</label>
              <div class="relative flex items-center">
                <span class="absolute left-3 font-mono text-emerald-400 text-sm pointer-events-none" aria-hidden="true">&gt;</span>
                <input
                  id="password"
                  name="password"
                  type={showPassword ? 'text' : 'password'}
                  autocomplete="new-password"
                  required
                  bind:value={password}
                  placeholder="••••••••"
                  class={inputClass('password', true)}
                />
                <button
                  type="button"
                  onclick={() => (showPassword = !showPassword)}
                  aria-label={showPassword ? 'Hide password' : 'Show password'}
                  aria-pressed={showPassword}
                  class="absolute right-2 top-1/2 -translate-y-1/2 p-1.5 text-stone-500 hover:text-stone-300 rounded-md hover:bg-stone-800/50 transition-colors"
                >
                  {#if showPassword}
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4" aria-hidden="true">
                      <path d="M9.88 9.88a3 3 0 1 0 4.24 4.24" />
                      <path d="M10.73 5.08A11 11 0 0 1 12 5c6.5 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68" />
                      <path d="M6.61 6.61A13.5 13.5 0 0 0 2 12s3.5 7 10 7a9.7 9.7 0 0 0 5.39-1.61" />
                      <line x1="2" y1="2" x2="22" y2="22" />
                    </svg>
                  {:else}
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4" aria-hidden="true">
                      <path d="M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7-10-7-10-7Z" />
                      <circle cx="12" cy="12" r="3" />
                    </svg>
                  {/if}
                </button>
              </div>
              <p class="mt-1.5 font-mono text-xs text-stone-600 dark:text-stone-600">// hint: 8+ chars, mix of letters, numbers, symbols</p>
              {#if fieldError('password')}
                <p class="mt-1 font-mono text-xs text-red-400">&gt; {fieldError('password')}</p>
              {/if}
            </div>

            <div>
              <label for="password_confirmation" class="block font-mono text-xs uppercase tracking-widest text-stone-500 dark:text-stone-400 mb-2">// CONFIRM PASSWORD</label>
              <div class="relative flex items-center">
                <span class="absolute left-3 font-mono text-emerald-400 text-sm pointer-events-none" aria-hidden="true">&gt;</span>
                <input
                  id="password_confirmation"
                  name="password_confirmation"
                  type={showPasswordConfirmation ? 'text' : 'password'}
                  autocomplete="new-password"
                  required
                  bind:value={password_confirmation}
                  placeholder="••••••••"
                  class={inputClass('password_confirmation', true)}
                />
                <button
                  type="button"
                  onclick={() => (showPasswordConfirmation = !showPasswordConfirmation)}
                  aria-label={showPasswordConfirmation ? 'Hide password' : 'Show password'}
                  aria-pressed={showPasswordConfirmation}
                  class="absolute right-2 top-1/2 -translate-y-1/2 p-1.5 text-stone-500 hover:text-stone-300 rounded-md hover:bg-stone-800/50 transition-colors"
                >
                  {#if showPasswordConfirmation}
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4" aria-hidden="true">
                      <path d="M9.88 9.88a3 3 0 1 0 4.24 4.24" />
                      <path d="M10.73 5.08A11 11 0 0 1 12 5c6.5 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68" />
                      <path d="M6.61 6.61A13.5 13.5 0 0 0 2 12s3.5 7 10 7a9.7 9.7 0 0 0 5.39-1.61" />
                      <line x1="2" y1="2" x2="22" y2="22" />
                    </svg>
                  {:else}
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4" aria-hidden="true">
                      <path d="M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7-10-7-10-7Z" />
                      <circle cx="12" cy="12" r="3" />
                    </svg>
                  {/if}
                </button>
              </div>
              {#if fieldError('password_confirmation')}
                <p class="mt-1 font-mono text-xs text-red-400">&gt; {fieldError('password_confirmation')}</p>
              {/if}
            </div>

            <div class="flex items-start gap-2.5 pt-1">
              <input
                type="checkbox"
                id="terms"
                name="terms"
                bind:checked={terms}
                required
                class="mt-0.5 w-4 h-4 rounded border-stone-700 bg-stone-900 text-emerald-500 focus:ring-2 focus:ring-emerald-500/50 focus:ring-offset-0 focus:ring-offset-stone-950 transition"
              />
              <label for="terms" class="font-mono text-xs text-stone-500 dark:text-stone-400 select-none leading-relaxed">
                I agree to the
                <a href="/terms" class="text-amber-400 hover:text-amber-300 hover:underline">Terms of Service</a>
                and
                <a href="/privacy" class="text-amber-400 hover:text-amber-300 hover:underline">Privacy Policy</a>
              </label>
            </div>

            <button
              type="submit"
              disabled={submitting}
              class="w-full inline-flex items-center justify-center gap-2 px-5 py-3 bg-emerald-600 hover:bg-emerald-500 text-stone-950 text-sm font-mono font-semibold uppercase tracking-wider rounded-lg transition-all shadow-sm hover:shadow-lg hover:shadow-emerald-500/20 active:scale-[0.98] disabled:opacity-60 disabled:cursor-not-allowed"
            >
              {#if submitting}
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4 animate-spin" aria-hidden="true">
                  <path d="M21 12a9 9 0 1 1-6.219-8.56" />
                </svg>
                <span>CREATING IDENTITY...</span>
              {:else}
                <span>→ CREATE IDENTITY</span>
              {/if}
            </button>
          </form>
        </div>
      </div>

      <p class="mt-6 text-center font-mono text-sm text-stone-500 dark:text-stone-400">
        $ Already have an account?
        <Link href="/login" class="text-amber-400 hover:text-amber-300 hover:underline">sign in</Link>
      </p>
    </div>
  </div>
</Layout>

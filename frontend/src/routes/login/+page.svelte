<script lang="ts">
  import { login } from '$lib/stores/auth';
  import { addToast } from '$lib/stores/ui';
  import { goto } from '$app/navigation';

  let email = $state('');
  let password = $state('');
  let loading = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!email || !password) {
      addToast('Please fill in all fields', 'error');
      return;
    }
    loading = true;
    try {
      await login(email, password);
      addToast('Welcome back!', 'success');
      goto('/dashboard');
    } catch (err) {
      addToast(err instanceof Error ? err.message : 'Login failed', 'error');
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head>
  <title>Log In — PennyWise</title>
</svelte:head>

<main class="min-h-screen bg-background flex flex-col items-center justify-center px-8 mesh-background">
  <div class="w-full max-w-md space-y-10">
    <!-- Branding -->
    <div class="text-center">
      <div class="flex items-center justify-center gap-2 mb-8">
        <div class="w-12 h-12 editorial-gradient rounded-xl flex items-center justify-center">
          <span class="material-symbols-outlined text-on-primary-container text-2xl" style="font-variation-settings: 'FILL' 1;">account_balance_wallet</span>
        </div>
      </div>
      <h1 class="font-headline text-3xl font-extrabold tracking-tight text-on-background">Welcome Back</h1>
      <p class="text-on-surface-variant mt-2">Sign in to your financial dashboard</p>
    </div>

    <!-- Form -->
    <form onsubmit={handleSubmit} class="space-y-5">
      <div>
        <label for="email" class="text-xs uppercase tracking-widest text-on-surface-variant font-label mb-2 block">Email</label>
        <input
          id="email"
          type="email"
          bind:value={email}
          placeholder="you@example.com"
          class="w-full bg-surface-container-low border-none rounded-xl py-4 px-4 text-on-surface placeholder:text-outline/40 focus:ring-1 focus:ring-primary/40 transition-all outline-none"
        />
      </div>

      <div>
        <label for="password" class="text-xs uppercase tracking-widest text-on-surface-variant font-label mb-2 block">Password</label>
        <input
          id="password"
          type="password"
          bind:value={password}
          placeholder="••••••••"
          class="w-full bg-surface-container-low border-none rounded-xl py-4 px-4 text-on-surface placeholder:text-outline/40 focus:ring-1 focus:ring-primary/40 transition-all outline-none"
        />
      </div>

      <button
        type="submit"
        disabled={loading}
        class="w-full py-4 editorial-gradient rounded-full text-on-primary font-bold tracking-tight active:scale-95 duration-200 shadow-xl shadow-primary/10 disabled:opacity-50 mt-4"
      >
        {loading ? 'Signing In...' : 'Sign In'}
      </button>
    </form>

    <p class="text-center text-on-surface-variant text-sm">
      Don't have an account?
      <a href="/register" class="text-primary font-semibold hover:underline">Sign up</a>
    </p>
  </div>
</main>

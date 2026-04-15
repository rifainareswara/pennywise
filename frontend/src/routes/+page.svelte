<script lang="ts">
  import { isAuthenticated } from '$lib/stores/auth';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  onMount(() => {
    if ($isAuthenticated) {
      goto('/dashboard');
    }
  });
</script>

<svelte:head>
  <title>PennyWise — Master Your Money</title>
</svelte:head>

<main class="relative h-screen w-full flex flex-col items-center justify-between px-8 py-12 md:px-16 mesh-background">
  <!-- Top Section: Branding -->
  <header class="w-full flex justify-center items-center">
    <div class="flex items-center gap-2">
      <div class="w-10 h-10 editorial-gradient rounded-xl flex items-center justify-center">
        <span class="material-symbols-outlined text-on-primary-container" style="font-variation-settings: 'FILL' 1;">account_balance_wallet</span>
      </div>
      <h1 class="font-headline text-2xl font-extrabold tracking-tighter text-primary">PennyWise</h1>
    </div>
  </header>

  <!-- Middle Section: Visual & Editorial Messaging -->
  <section class="flex-1 flex flex-col items-center justify-center w-full max-w-xl text-center">
    <!-- Financial Illustration Hero -->
    <div class="relative w-full aspect-square max-h-[340px] mb-12 flex items-center justify-center">
      <div class="absolute inset-0 bg-primary/5 rounded-full blur-3xl"></div>
      <div class="relative z-10 w-full h-full p-4 rounded-[3rem] overflow-hidden bg-surface-container-low/40 backdrop-blur-sm border border-outline-variant/10">
        <div class="w-full h-full rounded-[2rem] bg-surface-container-low flex flex-col items-center justify-center gap-6">
          <div class="w-24 h-24 rounded-full editorial-gradient flex items-center justify-center">
            <span class="material-symbols-outlined text-on-primary text-5xl" style="font-variation-settings: 'FILL' 1;">savings</span>
          </div>
          <div class="flex gap-4">
            <div class="bg-surface-container-high px-4 py-2 rounded-xl flex items-center gap-2">
              <span class="material-symbols-outlined text-primary text-lg" style="font-variation-settings: 'FILL' 1;">trending_up</span>
              <span class="text-primary font-headline font-bold text-sm">+12.5%</span>
            </div>
            <div class="bg-surface-container-high px-4 py-2 rounded-xl flex items-center gap-2">
              <span class="material-symbols-outlined text-secondary text-lg" style="font-variation-settings: 'FILL' 1;">receipt_long</span>
              <span class="text-on-surface font-headline font-bold text-sm">Rp3,2Jt</span>
            </div>
          </div>
        </div>
      </div>
      <!-- Floating Accents (Asymmetry) -->
      <div class="absolute -top-4 -right-4 bg-surface-container-high p-4 rounded-2xl shadow-2xl border border-outline-variant/20 rotate-12">
        <span class="material-symbols-outlined text-primary" style="font-variation-settings: 'FILL' 1;">trending_up</span>
      </div>
      <div class="absolute -bottom-6 -left-2 bg-surface-container-high p-3 rounded-2xl shadow-2xl border border-outline-variant/20 -rotate-6">
        <span class="material-symbols-outlined text-secondary" style="font-variation-settings: 'FILL' 1;">receipt_long</span>
      </div>
    </div>
    <!-- Headlines -->
    <div class="space-y-4">
      <h2 class="font-headline text-4xl md:text-5xl font-extrabold tracking-tight text-on-background leading-[1.1]">
        Master your money with <span class="text-primary">PennyWise</span>
      </h2>
      <p class="text-on-surface-variant text-lg leading-relaxed max-w-md mx-auto">
        Cultivate your wealth with an editorial-grade financial companion. Track budgets and expenses effortlessly.
      </p>
    </div>
  </section>

  <!-- Bottom Section: Actions & Indicators -->
  <footer class="w-full max-w-md space-y-8">
    <div class="flex flex-col gap-4">
      <a href="/register" class="editorial-gradient text-on-primary-container h-16 rounded-full font-headline font-bold text-lg hover:opacity-90 active:scale-[0.98] transition-all flex items-center justify-center">
        Get Started
      </a>
      <a href="/login" class="bg-surface-container-high text-on-background h-16 rounded-full font-headline font-bold text-lg hover:bg-surface-bright active:scale-[0.98] transition-all flex items-center justify-center">
        Log In
      </a>
    </div>
    <div class="flex flex-col items-center gap-6">
      <div class="flex gap-2">
        <span class="w-8 h-1.5 rounded-full bg-primary"></span>
        <span class="w-1.5 h-1.5 rounded-full bg-surface-container-highest"></span>
        <span class="w-1.5 h-1.5 rounded-full bg-surface-container-highest"></span>
      </div>
      <p class="text-[10px] uppercase tracking-widest text-on-surface-variant/40 font-label font-medium">
        Financial Intelligence · v1.0.0
      </p>
    </div>
  </footer>
</main>

<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { user, logout } from '$lib/stores/auth';
  import { addToast } from '$lib/stores/ui';
  import { goto } from '$app/navigation';

  function handleLogout() {
    logout();
    addToast('Berhasil keluar', 'info');
    goto('/');
  }
</script>

<svelte:head>
  <title>Profil — PennyWise</title>
</svelte:head>

<Navbar title="Profil" showAvatar={false} actions={[{ icon: 'settings' }]} />

<main class="px-6 pt-4 max-w-lg mx-auto page-transition">
  <!-- Profile Header -->
  <section class="flex flex-col items-center text-center mb-10">
    <div class="w-24 h-24 rounded-full bg-surface-container-high flex items-center justify-center mb-4">
      <span class="material-symbols-outlined text-5xl text-primary" style="font-variation-settings: 'FILL' 1;">person</span>
    </div>
    <h2 class="font-headline text-2xl font-bold text-on-surface">{$user?.name || 'User'}</h2>
    <p class="text-on-surface-variant text-sm mt-1">{$user?.email || 'user@example.com'}</p>
  </section>

  <!-- Profile Options -->
  <section class="space-y-3">
    <div class="bg-surface-container-high rounded-xl p-4 flex items-center justify-between cursor-pointer hover:bg-surface-container-highest transition-colors">
      <div class="flex items-center gap-4">
        <div class="w-10 h-10 rounded-xl bg-surface-container-highest flex items-center justify-center text-on-surface-variant">
          <span class="material-symbols-outlined">person</span>
        </div>
        <span class="text-on-surface font-medium">Edit Profil</span>
      </div>
      <span class="material-symbols-outlined text-on-surface-variant">chevron_right</span>
    </div>

    <div class="bg-surface-container-high rounded-xl p-4 flex items-center justify-between cursor-pointer hover:bg-surface-container-highest transition-colors">
      <div class="flex items-center gap-4">
        <div class="w-10 h-10 rounded-xl bg-surface-container-highest flex items-center justify-center text-on-surface-variant">
          <span class="material-symbols-outlined">notifications</span>
        </div>
        <span class="text-on-surface font-medium">Notifikasi</span>
      </div>
      <span class="material-symbols-outlined text-on-surface-variant">chevron_right</span>
    </div>

    <div class="bg-surface-container-high rounded-xl p-4 flex items-center justify-between cursor-pointer hover:bg-surface-container-highest transition-colors">
      <div class="flex items-center gap-4">
        <div class="w-10 h-10 rounded-xl bg-surface-container-highest flex items-center justify-center text-on-surface-variant">
          <span class="material-symbols-outlined">lock</span>
        </div>
        <span class="text-on-surface font-medium">Keamanan</span>
      </div>
      <span class="material-symbols-outlined text-on-surface-variant">chevron_right</span>
    </div>

    <div class="bg-surface-container-high rounded-xl p-4 flex items-center justify-between cursor-pointer hover:bg-surface-container-highest transition-colors">
      <div class="flex items-center gap-4">
        <div class="w-10 h-10 rounded-xl bg-surface-container-highest flex items-center justify-center text-on-surface-variant">
          <span class="material-symbols-outlined">help</span>
        </div>
        <span class="text-on-surface font-medium">Bantuan</span>
      </div>
      <span class="material-symbols-outlined text-on-surface-variant">chevron_right</span>
    </div>
  </section>

  <!-- Logout -->
  <div class="mt-10 mb-8">
    <button
      onclick={handleLogout}
      class="w-full py-4 bg-error-container/20 rounded-full text-error font-bold tracking-tight active:scale-95 duration-200"
    >
      Keluar
    </button>
  </div>

  <!-- App Info -->
  <div class="text-center mb-8">
    <p class="text-[10px] uppercase tracking-widest text-on-surface-variant/40 font-label font-medium">
      PennyWise · v1.0.0
    </p>
  </div>
</main>

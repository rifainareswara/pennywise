<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { onMount } from 'svelte';
  import { addToast } from '$lib/stores/ui';

  let pushEnabled = $state(false);
  let emailEnabled = $state(true);
  let dailyReminder = $state(true);

  onMount(() => {
    // Load from local storage
    if (typeof window !== 'undefined') {
      const prefs = localStorage.getItem('pennywise_notif_prefs');
      if (prefs) {
        try {
          const parsed = JSON.parse(prefs);
          pushEnabled = !!parsed.pushEnabled;
          emailEnabled = !!parsed.emailEnabled;
          dailyReminder = !!parsed.dailyReminder;
        } catch(e) {}
      }
    }
  });

  function savePreferences() {
    if (typeof window !== 'undefined') {
      localStorage.setItem('pennywise_notif_prefs', JSON.stringify({
        pushEnabled,
        emailEnabled,
        dailyReminder
      }));
      addToast('Preferensi notifikasi disimpan', 'success');
    }
  }
</script>

<svelte:head>
  <title>Notifikasi — PennyWise</title>
</svelte:head>

<Navbar title="Pengaturan Notifikasi" showBack={true} />

<main class="px-6 pt-4 max-w-lg mx-auto page-transition">
  <div class="p-4 bg-secondary/10 rounded-2xl border border-secondary/20 flex flex-col gap-2 mt-6 mb-6">
    <div class="flex items-center gap-2 text-secondary font-bold">
      <span class="material-symbols-outlined text-[20px]">notifications_active</span>
      Pengingat Pintar
    </div>
    <p class="text-sm text-on-surface-variant font-medium leading-relaxed">
      Sesuaikan bagaimana Anda ingin diingatkan tentang tagihan jatuh tempo dan ringkasan keuangan harian.
    </p>
  </div>

  <section class="space-y-6">
    <!-- Push Notifications -->
    <div class="flex items-center justify-between">
      <div class="flex flex-col pr-4">
        <h3 class="text-on-surface font-bold text-[16px]">Notifikasi Push</h3>
        <p class="text-sm text-on-surface-variant mt-0.5 leading-tight">Terima peringatan langsung di perangkat Anda saat ada tagihan jatuh tempo.</p>
      </div>
      <label class="relative inline-flex items-center cursor-pointer shrink-0">
        <input type="checkbox" bind:checked={pushEnabled} onchange={savePreferences} class="sr-only peer">
        <div class="w-11 h-6 bg-surface-container-highest peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary"></div>
      </label>
    </div>

    <!-- Daily Reminders -->
    <div class="flex items-center justify-between">
      <div class="flex flex-col pr-4">
        <h3 class="text-on-surface font-bold text-[16px]">Update Harian</h3>
        <p class="text-sm text-on-surface-variant mt-0.5 leading-tight">Ringkasan kondisi finansial harian Anda setiap pagi.</p>
      </div>
      <label class="relative inline-flex items-center cursor-pointer shrink-0">
        <input type="checkbox" bind:checked={dailyReminder} onchange={savePreferences} class="sr-only peer">
        <div class="w-11 h-6 bg-surface-container-highest peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary"></div>
      </label>
    </div>

    <!-- Email Options -->
    <div class="flex items-center justify-between">
      <div class="flex flex-col pr-4">
        <h3 class="text-on-surface font-bold text-[16px]">Buletin Email</h3>
        <p class="text-sm text-on-surface-variant mt-0.5 leading-tight">Terima laporan analitik pengeluaran bulanan Anda via Email.</p>
      </div>
      <label class="relative inline-flex items-center cursor-pointer shrink-0">
        <input type="checkbox" bind:checked={emailEnabled} onchange={savePreferences} class="sr-only peer">
        <div class="w-11 h-6 bg-surface-container-highest peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary"></div>
      </label>
    </div>
  </section>
</main>

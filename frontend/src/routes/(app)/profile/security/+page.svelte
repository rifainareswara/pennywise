<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { addToast } from '$lib/stores/ui';
  import { authApi } from '$lib/api/client';
  import { goto } from '$app/navigation';

  let oldPassword = $state('');
  let newPassword = $state('');
  let confirmPassword = $state('');
  let isLoading = $state(false);

  async function handleSave(e: Event) {
    e.preventDefault();
    
    if (newPassword !== confirmPassword) {
      addToast('Konfirmasi sandi baru tidak cocok', 'error');
      return;
    }

    if (newPassword.length < 8) {
      addToast('Sandi baru harus minimal 8 karakter', 'error');
      return;
    }

    try {
      isLoading = true;
      await authApi.updatePassword({ 
        old_password: oldPassword, 
        new_password: newPassword 
      });
      
      addToast('Keamanan (Sandi) berhasil diperbaharui', 'success');
      goto('/profile');
    } catch (e: any) {
      addToast(e.message || 'Gagal mengubah sandi', 'error');
    } finally {
      isLoading = false;
    }
  }
</script>

<svelte:head>
  <title>Keamanan — PennyWise</title>
</svelte:head>

<Navbar title="Ubah Sandi" showBack={true} />

<main class="px-6 pt-4 max-w-lg mx-auto page-transition">
  <form onsubmit={handleSave} class="flex flex-col gap-5 mt-6">
    <div class="p-4 bg-primary/10 rounded-2xl border border-primary/20 flex flex-col gap-2 mb-2">
      <div class="flex items-center gap-2 text-primary font-bold">
        <span class="material-symbols-outlined text-[20px]">shield</span>
        Keamanan Akun
      </div>
      <p class="text-sm text-on-surface-variant font-medium leading-relaxed">
        Gunakan kata sandi yang kuat (minimal 8 karakter kombinasi huruf dan angka) agar data finansial Anda tetap aman.
      </p>
    </div>

    <label class="flex flex-col gap-1.5">
      <span class="text-sm text-on-surface-variant ml-2 font-medium">Sandi Saat Ini</span>
      <input 
        type="password" 
        bind:value={oldPassword}
        placeholder="Masukkan sandi saat ini"
        required
        class="px-5 py-4 rounded-[16px] bg-surface-container-highest/50 border border-outline/20 focus:border-primary focus:bg-surface focus:outline-none transition-colors text-on-surface font-medium"
      />
    </label>

    <label class="flex flex-col gap-1.5 mt-2">
      <span class="text-sm text-on-surface-variant ml-2 font-medium">Sandi Baru</span>
      <input 
        type="password" 
        bind:value={newPassword}
        placeholder="Masukkan sandi baru"
        required
        minlength="8"
        class="px-5 py-4 rounded-[16px] bg-surface-container-highest/50 border border-outline/20 focus:border-primary focus:bg-surface focus:outline-none transition-colors text-on-surface font-medium"
      />
    </label>

    <label class="flex flex-col gap-1.5">
      <span class="text-sm text-on-surface-variant ml-2 font-medium">Konfirmasi Sandi Baru</span>
      <input 
        type="password" 
        bind:value={confirmPassword}
        placeholder="Ulangi sandi baru"
        required
        minlength="8"
        class="px-5 py-4 rounded-[16px] bg-surface-container-highest/50 border border-outline/20 focus:border-primary focus:bg-surface focus:outline-none transition-colors text-on-surface font-medium"
      />
    </label>

    <div class="mt-8">
      <button 
        type="submit" 
        disabled={isLoading || !oldPassword || !newPassword || !confirmPassword}
        class="w-full bg-primary text-on-primary font-bold py-4 rounded-full active:scale-95 transition-all disabled:opacity-50 flex justify-center items-center gap-2"
      >
        {#if isLoading}
          <div class="w-5 h-5 rounded-full border-2 border-on-primary/30 border-t-on-primary animate-spin"></div>
          Memperbarui...
        {:else}
          Ubah Kata Sandi
        {/if}
      </button>
    </div>
  </form>
</main>

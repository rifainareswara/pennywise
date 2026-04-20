<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { user } from '$lib/stores/auth';
  import { addToast } from '$lib/stores/ui';
  import { authApi } from '$lib/api/client';
  import { goto } from '$app/navigation';

  let name = $state($user?.name || '');
  let email = $state($user?.email || '');
  let isLoading = $state(false);

  async function handleSave(e: Event) {
    e.preventDefault();
    if (!name.trim()) return;

    try {
      isLoading = true;
      const updatedUser = await authApi.updateProfile({ name });
      
      // Update store
      user.set(updatedUser);

      addToast('Profil berhasil diperbarui', 'success');
      goto('/profile');
    } catch (e: any) {
      addToast(e.message || 'Gagal menyimpan profil', 'error');
    } finally {
      isLoading = false;
    }
  }
</script>

<svelte:head>
  <title>Edit Profil — PennyWise</title>
</svelte:head>

<Navbar title="Edit Profil" showAvatar={false} />

<main class="px-6 pt-4 max-w-lg mx-auto page-transition">
  <form onsubmit={handleSave} class="flex flex-col gap-5 mt-6">
    <label class="flex flex-col gap-1.5">
      <span class="text-sm text-on-surface-variant ml-2 font-medium">Email Akun (Read-only)</span>
      <input 
        type="email" 
        value={email}
        disabled
        class="px-5 py-4 rounded-[16px] bg-surface-container-highest/30 border border-outline/10 text-on-surface-variant focus:outline-none transition-colors"
      />
    </label>

    <label class="flex flex-col gap-1.5">
      <span class="text-sm text-on-surface-variant ml-2 font-medium">Nama Lengkap</span>
      <input 
        type="text" 
        bind:value={name}
        placeholder="Masukkan nama Anda"
        required
        class="px-5 py-4 rounded-[16px] bg-surface-container-highest/50 border border-outline/20 focus:border-primary focus:bg-surface focus:outline-none transition-colors text-on-surface font-medium"
      />
    </label>

    <div class="mt-8">
      <button 
        type="submit" 
        disabled={isLoading || !name.trim()}
        class="w-full bg-primary text-on-primary font-bold py-4 rounded-full active:scale-95 transition-all disabled:opacity-50 flex justify-center items-center gap-2"
      >
        {#if isLoading}
          <div class="w-5 h-5 rounded-full border-2 border-on-primary/30 border-t-on-primary animate-spin"></div>
          Memperbarui...
        {:else}
          Simpan Perubahan
        {/if}
      </button>
    </div>
  </form>
</main>

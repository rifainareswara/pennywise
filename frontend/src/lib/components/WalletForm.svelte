<script lang="ts">
  import { fade, slide } from 'svelte/transition';
  import type { Wallet, WalletInput } from '$lib/api/client';

  let { isOpen = $bindable(false), wallet = null, onSave, onClose }: {
    isOpen: boolean;
    wallet?: Wallet | null;
    onSave: (data: WalletInput) => void;
    onClose: () => void;
  } = $props();

  type WalletType = 'bank' | 'e_wallet' | 'cash';

  let name = $state('');
  let walletType = $state<WalletType>('bank');
  let balance = $state('');

  const iconMap: Record<WalletType, string> = {
    bank: 'account_balance',
    e_wallet: 'account_balance_wallet',
    cash: 'payments',
  };

  const typeOptions: { id: WalletType; label: string; icon: string }[] = [
    { id: 'bank',     label: 'Bank',     icon: 'account_balance' },
    { id: 'e_wallet', label: 'E-Wallet', icon: 'account_balance_wallet' },
    { id: 'cash',     label: 'Tunai',    icon: 'payments' },
  ];

  $effect(() => {
    if (isOpen) {
      if (wallet) {
        name = wallet.name;
        walletType = wallet.wallet_type;
        balance = parseFloat(wallet.balance).toString();
      } else {
        name = '';
        walletType = 'bank';
        balance = '';
      }
    }
  });

  function handleSave() {
    if (!name.trim()) return;
    onSave({
      name: name.trim(),
      wallet_type: walletType,
      balance: parseFloat(balance) || 0,
      icon: iconMap[walletType],
    });
    isOpen = false;
  }
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 bg-background/80 backdrop-blur-sm z-[100]" transition:fade={{ duration: 200 }} onclick={onClose}></div>

  <div
    class="fixed inset-x-0 bottom-0 bg-surface rounded-t-[32px] z-[101] shadow-2xl safe-area-bottom pb-20 pt-6 px-6 max-h-[85vh] overflow-y-auto"
    transition:slide={{ duration: 300, axis: 'y' }}
  >
    <div class="w-12 h-1.5 bg-outline rounded-full mx-auto mb-6"></div>

    <div class="flex items-center justify-between mb-6">
      <h2 class="font-title text-2xl text-on-surface">
        {wallet ? 'Edit Rekening' : 'Tambah Rekening'}
      </h2>
      <button
        onclick={onClose}
        class="w-10 h-10 rounded-full bg-surface-container-highest flex items-center justify-center text-on-surface-variant hover:text-on-surface transition-colors"
        type="button"
        aria-label="Tutup"
      >
        <span class="material-symbols-outlined">close</span>
      </button>
    </div>

    <div class="flex flex-col gap-5">
      <!-- Type selector -->
      <div>
        <span class="text-sm text-on-surface-variant ml-2 mb-2 block">Jenis Rekening</span>
        <div class="flex gap-2">
          {#each typeOptions as opt}
            <button
              class="flex-1 flex flex-col items-center gap-1.5 py-3 rounded-[16px] border text-sm transition-all {walletType === opt.id ? 'bg-primary border-primary text-on-primary' : 'border-outline text-on-surface-variant hover:border-primary/50'}"
              onclick={() => walletType = opt.id}
              type="button"
            >
              <span class="material-symbols-outlined text-[20px]">{opt.icon}</span>
              <span class="text-[11px] font-medium">{opt.label}</span>
            </button>
          {/each}
        </div>
      </div>

      <!-- Name -->
      <label class="flex flex-col gap-1.5">
        <span class="text-sm text-on-surface-variant ml-2">Nama Rekening</span>
        <input
          type="text"
          bind:value={name}
          placeholder="cth. BCA Tabungan, GoPay, Dompet Tunai"
          class="px-4 py-3 rounded-[16px] bg-surface-container-highest/50 border border-outline/20 focus:border-primary focus:bg-surface focus:outline-none transition-colors"
        />
      </label>

      <!-- Balance -->
      <label class="flex flex-col gap-1.5">
        <span class="text-sm text-on-surface-variant ml-2">
          {wallet ? 'Sesuaikan Saldo Saat Ini (Rp)' : 'Saldo Awal (Rp)'}
        </span>
        <div class="relative">
          <span class="absolute left-4 top-1/2 -translate-y-1/2 text-on-surface-variant font-bold">Rp</span>
          <input
            type="number"
            bind:value={balance}
            placeholder="0"
            min="0"
            class="w-full pl-10 pr-4 py-3 rounded-[16px] bg-surface-container-highest/50 border border-outline/20 focus:border-primary focus:bg-surface focus:outline-none transition-colors"
          />
        </div>
        {#if wallet}
          <p class="text-xs text-on-surface-variant ml-2">Masukkan saldo terkini untuk menyesuaikan catatan.</p>
        {/if}
      </label>

      <button
        onclick={handleSave}
        disabled={!name.trim()}
        class="w-full mt-2 bg-primary text-background font-bold py-4 rounded-full active:scale-95 transition-all disabled:opacity-50"
        type="button"
      >
        {wallet ? 'Simpan Perubahan' : 'Tambah Rekening'}
      </button>
    </div>
  </div>
{/if}

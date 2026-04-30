<script lang="ts">
  import type { Wallet } from '$lib/api/client';
  import { formatRupiah } from '$lib/utils/currency';
  import { deleteWallet } from '$lib/stores/wallets';
  import { addToast } from '$lib/stores/ui';

  let { wallet, onedit }: {
    wallet: Wallet;
    onedit?: (w: Wallet) => void;
  } = $props();

  let loading = $state(false);

  const typeConfig: Record<string, { label: string; icon: string; colorClass: string; bgClass: string }> = {
    bank:     { label: 'Bank',    icon: 'account_balance',        colorClass: 'text-primary',             bgClass: 'bg-primary/10' },
    e_wallet: { label: 'E-Wallet', icon: 'account_balance_wallet', colorClass: 'text-[#4FC3F7]',           bgClass: 'bg-[#4FC3F7]/10' },
    cash:     { label: 'Tunai',   icon: 'payments',               colorClass: 'text-on-surface-variant',  bgClass: 'bg-surface-container-highest' },
  };

  let config = $derived(typeConfig[wallet.wallet_type] ?? typeConfig.bank);
  let balance = $derived(parseFloat(wallet.balance));

  async function handleDelete() {
    if (!wallet.user_id) {
      addToast('Tidak bisa hapus data demo', 'error');
      return;
    }
    if (confirm(`Hapus rekening "${wallet.name}"?`)) {
      loading = true;
      try {
        await deleteWallet(wallet.id);
        addToast('Rekening dihapus', 'success');
      } catch (e: any) {
        addToast(e.message || 'Gagal menghapus rekening', 'error');
      } finally {
        loading = false;
      }
    }
  }
</script>

<div class="bg-surface-container-high rounded-xl p-4 flex items-center gap-4">
  <!-- Icon -->
  <div class="w-12 h-12 rounded-xl {config.bgClass} flex items-center justify-center shrink-0">
    <span class="material-symbols-outlined {config.colorClass}" style="font-variation-settings: 'FILL' 1;">
      {wallet.icon || config.icon}
    </span>
  </div>

  <!-- Info -->
  <div class="flex-1 min-w-0">
    <p class="font-bold text-on-surface truncate">{wallet.name}</p>
    <p class="text-xs text-on-surface-variant">{config.label}</p>
  </div>

  <!-- Balance + Actions -->
  <div class="flex flex-col items-end gap-1.5 shrink-0">
    <p class="font-bold {config.colorClass}">{formatRupiah(balance)}</p>
    <div class="flex gap-1">
      <button
        class="w-7 h-7 rounded-full bg-surface-container-highest flex items-center justify-center text-on-surface-variant hover:text-primary transition-colors disabled:opacity-50"
        onclick={() => onedit?.(wallet)}
        disabled={loading}
        aria-label="Edit"
      >
        <span class="material-symbols-outlined text-[14px]">edit</span>
      </button>
      <button
        class="w-7 h-7 rounded-full bg-surface-container-highest flex items-center justify-center text-on-surface-variant hover:text-secondary transition-colors disabled:opacity-50"
        onclick={handleDelete}
        disabled={loading}
        aria-label="Hapus"
      >
        <span class="material-symbols-outlined text-[14px]">delete</span>
      </button>
    </div>
  </div>
</div>

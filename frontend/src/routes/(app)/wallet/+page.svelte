<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import WalletCard from '$lib/components/WalletCard.svelte';
  import WalletForm from '$lib/components/WalletForm.svelte';
  import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
  import { wallets, walletsLoading, loadWallets, createWallet, updateWallet } from '$lib/stores/wallets';
  import { addToast } from '$lib/stores/ui';
  import { formatRupiah } from '$lib/utils/currency';
  import type { Wallet, WalletInput } from '$lib/api/client';
  import { onMount } from 'svelte';

  let showForm = $state(false);
  let editWallet = $state<Wallet | null>(null);

  const demoWallets: Wallet[] = [
    { id: 'd1', user_id: '', name: 'BCA Tabungan', wallet_type: 'bank', balance: '8500000', icon: 'account_balance', color: 'primary', created_at: '', updated_at: '' },
    { id: 'd2', user_id: '', name: 'GoPay', wallet_type: 'e_wallet', balance: '350000', icon: 'account_balance_wallet', color: null, created_at: '', updated_at: '' },
    { id: 'd3', user_id: '', name: 'Dompet Tunai', wallet_type: 'cash', balance: '200000', icon: 'payments', color: null, created_at: '', updated_at: '' },
  ];

  let displayWallets = $derived($wallets.length > 0 ? $wallets : demoWallets);
  let totalBalance = $derived(displayWallets.reduce((sum, w) => sum + parseFloat(w.balance), 0));
  let bankTotal = $derived(displayWallets.filter(w => w.wallet_type === 'bank').reduce((sum, w) => sum + parseFloat(w.balance), 0));
  let ewalletTotal = $derived(displayWallets.filter(w => w.wallet_type === 'e_wallet').reduce((sum, w) => sum + parseFloat(w.balance), 0));

  const recommendations = [
    {
      icon: 'savings',
      color: 'text-primary',
      bg: 'bg-primary/10',
      title: 'Dana Darurat',
      desc: 'Simpan minimal 3–6 bulan pengeluaran di rekening bank terpisah sebagai dana darurat.',
    },
    {
      icon: 'pie_chart',
      color: 'text-[#4FC3F7]',
      bg: 'bg-[#4FC3F7]/10',
      title: 'Aturan 50/30/20',
      desc: '50% untuk kebutuhan, 30% untuk keinginan, dan 20% untuk tabungan & investasi setiap bulan.',
    },
    {
      icon: 'account_balance',
      color: 'text-primary',
      bg: 'bg-primary/10',
      title: 'Rekening Bank',
      desc: 'Gunakan rekening bank untuk pembayaran rutin, tagihan, dan transfer antar rekening agar lebih mudah dilacak.',
    },
    {
      icon: 'account_balance_wallet',
      color: 'text-[#4FC3F7]',
      bg: 'bg-[#4FC3F7]/10',
      title: 'E-Wallet',
      desc: 'Isi e-wallet secukupnya untuk pengeluaran harian agar pengeluaran impulsif lebih terkontrol.',
    },
  ];

  onMount(() => {
    loadWallets();
  });

  function handleEdit(w: Wallet) {
    editWallet = w;
    showForm = true;
  }

  function closeForm() {
    showForm = false;
    editWallet = null;
  }

  async function handleSave(data: WalletInput) {
    try {
      if (editWallet) {
        await updateWallet(editWallet.id, data);
        addToast('Rekening diperbarui', 'success');
      } else {
        await createWallet(data);
        addToast('Rekening ditambahkan', 'success');
      }
    } catch (e: any) {
      addToast(e.message || 'Operasi gagal', 'error');
    }
  }
</script>

<svelte:head>
  <title>Dompet & Rekening — PennyWise</title>
</svelte:head>

<Navbar title="Dompet & Rekening" showBack={true} />

<main class="px-6 pt-4 max-w-lg mx-auto pb-32 page-transition">
  {#if $walletsLoading}
    <LoadingSpinner />
  {:else}
    <!-- Total Balance -->
    <section class="bg-surface-container-low rounded-xl p-6 mt-4 mb-6 relative overflow-hidden">
      <div class="absolute -right-8 -top-8 w-36 h-36 rounded-full blur-3xl" style="background: rgba(84,233,138,0.06);"></div>
      <p class="text-on-surface-variant text-sm mb-1">Total Saldo Semua Rekening</p>
      <div class="flex items-baseline gap-1 mb-4">
        <span class="text-primary font-headline text-xl font-bold">Rp</span>
        <h2 class="text-on-surface font-headline text-[2.4rem] leading-none font-extrabold tracking-tighter">
          {totalBalance.toLocaleString('id-ID')}
        </h2>
      </div>
      <div class="flex gap-4 text-xs text-on-surface-variant">
        <span class="flex items-center gap-1">
          <span class="w-2 h-2 rounded-full bg-primary inline-block"></span>
          Bank: {formatRupiah(bankTotal)}
        </span>
        <span class="flex items-center gap-1">
          <span class="w-2 h-2 rounded-full bg-[#4FC3F7] inline-block"></span>
          E-Wallet: {formatRupiah(ewalletTotal)}
        </span>
      </div>
    </section>

    <!-- Wallet List -->
    <section class="mb-8">
      <div class="flex justify-between items-center mb-4">
        <h2 class="font-headline text-lg font-bold text-on-surface">Rekening Saya</h2>
        <button
          class="text-primary text-sm font-semibold hover:opacity-80 transition-opacity"
          onclick={() => { editWallet = null; adjustTarget = null; showForm = true; }}
        >
          + Tambah
        </button>
      </div>

      {#if displayWallets.length > 0}
        <div class="space-y-3">
          {#each displayWallets as wallet (wallet.id)}
            <WalletCard {wallet} onedit={handleEdit} />
          {/each}
        </div>
      {:else}
        <div class="text-center py-10 text-on-surface-variant">
          <span class="material-symbols-outlined text-[48px] mb-2 block">account_balance_wallet</span>
          <p class="font-medium">Belum ada rekening</p>
          <p class="text-sm mt-1">Tambahkan rekening pertama Anda</p>
        </div>
      {/if}
    </section>

    <!-- Recommendations -->
    <section>
      <h2 class="font-headline text-lg font-bold text-on-surface mb-4">Rekomendasi Terbaik</h2>
      <div class="space-y-3">
        {#each recommendations as rec}
          <div class="bg-surface-container-high rounded-xl p-4 flex gap-4 items-start">
            <div class="w-10 h-10 rounded-xl {rec.bg} flex items-center justify-center shrink-0">
              <span class="material-symbols-outlined {rec.color}" style="font-variation-settings: 'FILL' 1;">{rec.icon}</span>
            </div>
            <div>
              <p class="font-bold text-on-surface text-sm">{rec.title}</p>
              <p class="text-xs text-on-surface-variant mt-0.5 leading-relaxed">{rec.desc}</p>
            </div>
          </div>
        {/each}
      </div>
    </section>
  {/if}
</main>

<WalletForm bind:isOpen={showForm} wallet={editWallet} onSave={handleSave} onClose={closeForm} />

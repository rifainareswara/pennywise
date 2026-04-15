<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import SearchBar from '$lib/components/SearchBar.svelte';
  import FilterChips from '$lib/components/FilterChips.svelte';
  import TransactionItem from '$lib/components/TransactionItem.svelte';
  import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import { loadTransactions, groupedTransactions, transactionsLoading, searchQuery, filterType } from '$lib/stores/transactions';
  import { deleteTransaction } from '$lib/stores/transactions';
  import { addToast } from '$lib/stores/ui';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  let localSearch = $state('');

  const filters = [
    { label: 'Semua', value: 'all' },
    { label: 'Bulan Ini', value: 'this_month' },
    { label: 'Pemasukan', value: 'income' },
    { label: 'Pengeluaran', value: 'expense' },
  ];

  let activeFilter = $state('all');

  onMount(() => {
    loadTransactions();
  });

  function handleSearch(e: Event) {
    const target = e.target as HTMLInputElement;
    localSearch = target.value;
    searchQuery.set(target.value);
  }

  function handleFilter(value: string) {
    activeFilter = value;
    if (value === 'income' || value === 'expense') {
      filterType.set(value);
    } else {
      filterType.set('all');
    }
    if (value === 'this_month') {
      const now = new Date();
      loadTransactions({ month: now.getMonth() + 1, year: now.getFullYear() });
    } else if (value === 'income' || value === 'expense') {
      loadTransactions({ type: value });
    } else {
      loadTransactions();
    }
  }

  async function handleDelete(id: string) {
    if (confirm('Hapus transaksi ini?')) {
      try {
        await deleteTransaction(id);
        addToast('Transaksi dihapus', 'success');
      } catch {
        addToast('Failed to delete', 'error');
      }
    }
  }
</script>

<svelte:head>
  <title>Transactions — PennyWise</title>
</svelte:head>

<Navbar title="Transaksi" actions={[{ icon: 'filter_list' }, { icon: 'notifications' }]} />

<main class="px-6 pt-4 space-y-8 page-transition">
  <!-- Search & Filters -->
  <section class="space-y-6">
    <SearchBar value={localSearch} placeholder="Cari transaksi..." oninput={handleSearch} />
    <FilterChips {filters} active={activeFilter} onselect={handleFilter} />
  </section>

  {#if $transactionsLoading}
    <LoadingSpinner />
  {:else}
    {#each Object.entries($groupedTransactions) as [dateLabel, txs] (dateLabel)}
      <section class="space-y-4">
        <div class="flex justify-between items-end px-1">
          <h2 class="font-headline text-lg font-bold text-on-surface">{dateLabel}</h2>
          {#if txs.length > 0}
            <span class="text-label text-xs font-medium text-outline uppercase tracking-widest">
              {new Date(txs[0].date).toLocaleDateString('id-ID', { day: 'numeric', month: 'short', year: 'numeric' })}
            </span>
          {/if}
        </div>
        <div class="space-y-3">
          {#each txs as tx (tx.id)}
            <TransactionItem transaction={tx} onclick={() => goto(`/transactions/${tx.id}/edit`)} />
          {/each}
        </div>
      </section>
    {:else}
      <EmptyState icon="receipt_long" title="Transaksi tidak ditemukan" message="Coba ubah pencarian atau filter, atau tambahkan transaksi baru." />
    {/each}
  {/if}
</main>

<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import TransactionForm from '$lib/components/TransactionForm.svelte';
  import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
  import { page } from '$app/stores';
  import { api, type Transaction } from '$lib/api/client';
  import { deleteTransaction } from '$lib/stores/transactions';
  import { addToast } from '$lib/stores/ui';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  let transaction = $state<Transaction | null>(null);
  let loading = $state(true);

  onMount(async () => {
    const id = $page.params.id;
    try {
      transaction = await api<Transaction>(`/transactions/${id}`);
    } catch {
      addToast('Transaction not found', 'error');
      goto('/transactions');
    } finally {
      loading = false;
    }
  });

  async function handleDelete() {
    if (!transaction) return;
    if (confirm('Apakah Anda yakin ingin menghapus transaksi ini?')) {
      try {
        await deleteTransaction(transaction.id);
        addToast('Transaction deleted', 'success');
        goto('/transactions');
      } catch {
        addToast('Failed to delete', 'error');
      }
    }
  }
</script>

<svelte:head>
  <title>Edit Transaksi — PennyWise</title>
</svelte:head>

<Navbar title="Edit Transaksi" showAvatar={false} actions={[{ icon: 'close', href: '/transactions' }]} />

<main class="px-6 pt-4 max-w-lg mx-auto page-transition">
  {#if loading}
    <LoadingSpinner />
  {:else if transaction}
    <TransactionForm {transaction} mode="edit" />
    <div class="mt-6 mb-8">
      <button
        onclick={handleDelete}
        class="w-full py-4 bg-error-container/20 rounded-full text-error font-bold tracking-tight active:scale-95 duration-200"
      >
        Hapus Transaksi
      </button>
    </div>
  {/if}
</main>

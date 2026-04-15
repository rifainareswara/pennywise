<script lang="ts">
  import { createTransaction, updateTransaction } from '$lib/stores/transactions';
  import { addToast } from '$lib/stores/ui';
  import type { Transaction, TransactionInput } from '$lib/api/client';
  import { goto } from '$app/navigation';

  let { transaction, mode = 'create' }: {
    transaction?: Transaction;
    mode?: 'create' | 'edit';
  } = $props();

  let loading = $state(false);
  let description = $state(transaction?.description || '');
  let amount = $state(transaction ? parseFloat(transaction.amount).toString() : '');
  let category = $state(transaction?.category || '');
  let transactionType = $state<'income' | 'expense'>(transaction?.transaction_type || 'expense');
  let icon = $state(transaction?.icon || 'payments');
  let date = $state(transaction?.date ? new Date(transaction.date).toISOString().split('T')[0] : new Date().toISOString().split('T')[0]);

  const categories = [
    { label: 'Food & Dining', value: 'Food & Dining', icon: 'restaurant' },
    { label: 'Transport', value: 'Transport', icon: 'directions_car' },
    { label: 'Shopping', value: 'Shopping', icon: 'shopping_bag' },
    { label: 'Entertainment', value: 'Entertainment', icon: 'movie' },
    { label: 'Groceries', value: 'Groceries', icon: 'shopping_cart' },
    { label: 'Bills & Utilities', value: 'Bills & Utilities', icon: 'receipt' },
    { label: 'Salary', value: 'Salary', icon: 'payments' },
    { label: 'Freelance', value: 'Freelance', icon: 'account_balance_wallet' },
    { label: 'Other', value: 'Other', icon: 'category' },
  ];

  function selectCategory(cat: typeof categories[0]) {
    category = cat.value;
    icon = cat.icon;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!description || !amount || !category) {
      addToast('Please fill in all required fields', 'error');
      return;
    }

    loading = true;
    try {
      const data: TransactionInput = {
        description,
        amount: parseFloat(amount),
        category,
        transaction_type: transactionType,
        icon,
        date: new Date(date).toISOString(),
      };

      if (mode === 'edit' && transaction) {
        await updateTransaction(transaction.id, data);
        addToast('Transaction updated', 'success');
      } else {
        await createTransaction(data);
        addToast('Transaction added', 'success');
      }
      goto('/transactions');
    } catch (err) {
      addToast(err instanceof Error ? err.message : 'Operation failed', 'error');
    } finally {
      loading = false;
    }
  }
</script>

<form onsubmit={handleSubmit} class="space-y-6">
  <!-- Type Toggle -->
  <div class="flex gap-3">
    <button
      type="button"
      class="flex-1 py-3 rounded-xl text-sm font-semibold transition-all duration-200 {transactionType === 'expense' ? 'bg-secondary/20 text-secondary' : 'bg-surface-container-high text-on-surface-variant'}"
      onclick={() => transactionType = 'expense'}
    >
      Pengeluaran
    </button>
    <button
      type="button"
      class="flex-1 py-3 rounded-xl text-sm font-semibold transition-all duration-200 {transactionType === 'income' ? 'bg-primary/20 text-primary' : 'bg-surface-container-high text-on-surface-variant'}"
      onclick={() => transactionType = 'income'}
    >
      Pemasukan
    </button>
  </div>

  <!-- Amount -->
  <div>
    <label class="text-xs uppercase tracking-widest text-on-surface-variant font-label mb-2 block">Jumlah</label>
    <div class="relative">
      <span class="absolute left-4 top-1/2 -translate-y-1/2 text-on-surface-variant font-headline text-xl font-bold">Rp</span>
      <input
        type="number"
        step="100"
        min="0"
        bind:value={amount}
        placeholder="0"
        class="w-full bg-surface-container-low border-none rounded-xl py-4 pl-12 pr-4 text-on-surface font-headline text-2xl font-bold placeholder:text-outline/40 focus:ring-1 focus:ring-primary/40 transition-all outline-none"
      />
    </div>
  </div>

  <!-- Description -->
  <div>
    <label class="text-xs uppercase tracking-widest text-on-surface-variant font-label mb-2 block">Deskripsi</label>
    <input
      type="text"
      bind:value={description}
      placeholder="cth. Belanja Bulanan"
      class="w-full bg-surface-container-low border-none rounded-xl py-4 px-4 text-on-surface placeholder:text-outline/40 focus:ring-1 focus:ring-primary/40 transition-all outline-none"
    />
  </div>

  <!-- Category -->
  <div>
    <label class="text-xs uppercase tracking-widest text-on-surface-variant font-label mb-3 block">Kategori</label>
    <div class="grid grid-cols-3 gap-2">
      {#each categories as cat}
        <button
          type="button"
          class="flex flex-col items-center gap-2 p-3 rounded-xl transition-all duration-200 {category === cat.value ? 'bg-primary/15 text-primary' : 'bg-surface-container-low text-on-surface-variant'}"
          onclick={() => selectCategory(cat)}
        >
          <span class="material-symbols-outlined" style="font-variation-settings: 'FILL' {category === cat.value ? 1 : 0};">{cat.icon}</span>
          <span class="text-[10px] font-medium truncate w-full text-center">{cat.label}</span>
        </button>
      {/each}
    </div>
  </div>

  <!-- Date -->
  <div>
    <label class="text-xs uppercase tracking-widest text-on-surface-variant font-label mb-2 block">Tanggal</label>
    <input
      type="date"
      bind:value={date}
      class="w-full bg-surface-container-low border-none rounded-xl py-4 px-4 text-on-surface focus:ring-1 focus:ring-primary/40 transition-all outline-none"
    />
  </div>

  <!-- Submit -->
  <button
    type="submit"
    disabled={loading}
    class="w-full py-4 editorial-gradient rounded-full text-on-primary font-bold tracking-tight active:scale-95 duration-200 shadow-xl shadow-primary/10 disabled:opacity-50"
  >
    {#if loading}
      Saving...
    {:else}
      {mode === 'edit' ? 'Perbarui Transaksi' : 'Tambah Transaksi'}
    {/if}
  </button>
</form>

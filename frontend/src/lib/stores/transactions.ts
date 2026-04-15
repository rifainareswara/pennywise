import { writable, derived } from 'svelte/store';
import { transactionsApi, type Transaction, type TransactionInput } from '$lib/api/client';

export const transactions = writable<Transaction[]>([]);
export const transactionsLoading = writable(false);
export const transactionsError = writable<string | null>(null);
export const searchQuery = writable('');
export const filterType = writable<'all' | 'income' | 'expense'>('all');
export const filterPeriod = writable<'this_month' | 'last_month' | 'all'>('this_month');

export const filteredTransactions = derived(
  [transactions, searchQuery, filterType],
  ([$transactions, $search, $type]) => {
    let filtered = $transactions;
    if ($search) {
      const q = $search.toLowerCase();
      filtered = filtered.filter(
        (t) =>
          t.description.toLowerCase().includes(q) ||
          t.category.toLowerCase().includes(q)
      );
    }
    if ($type !== 'all') {
      filtered = filtered.filter((t) => t.transaction_type === $type);
    }
    return filtered;
  }
);

export const groupedTransactions = derived(filteredTransactions, ($filtered) => {
  const groups: Record<string, Transaction[]> = {};
  const today = new Date();
  const yesterday = new Date(today);
  yesterday.setDate(yesterday.getDate() - 1);

  for (const t of $filtered) {
    const d = new Date(t.date);
    let label: string;
    if (d.toDateString() === today.toDateString()) {
      label = 'Hari Ini';
    } else if (d.toDateString() === yesterday.toDateString()) {
      label = 'Kemarin';
    } else {
      label = d.toLocaleDateString('id-ID', { day: 'numeric', month: 'short', year: 'numeric' });
    }
    if (!groups[label]) groups[label] = [];
    groups[label].push(t);
  }
  return groups;
});

export async function loadTransactions(params?: { search?: string; type?: string; month?: number; year?: number }) {
  transactionsLoading.set(true);
  transactionsError.set(null);
  try {
    const data = await transactionsApi.list(params);
    transactions.set(data);
  } catch (err: unknown) {
    transactionsError.set(err instanceof Error ? err.message : 'Failed to load transactions');
  } finally {
    transactionsLoading.set(false);
  }
}

export async function createTransaction(data: TransactionInput) {
  const result = await transactionsApi.create(data);
  transactions.update((list) => [result, ...list]);
  return result;
}

export async function updateTransaction(id: string, data: TransactionInput) {
  const result = await transactionsApi.update(id, data);
  transactions.update((list) => list.map((t) => (t.id === id ? result : t)));
  return result;
}

export async function deleteTransaction(id: string) {
  await transactionsApi.delete(id);
  transactions.update((list) => list.filter((t) => t.id !== id));
}

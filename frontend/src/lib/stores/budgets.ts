import { writable } from 'svelte/store';
import { budgetsApi, type Budget, type BudgetInput } from '$lib/api/client';

export const budgets = writable<Budget[]>([]);
export const budgetsLoading = writable(false);
export const budgetsError = writable<string | null>(null);

export async function loadBudgets(params?: { month?: number; year?: number }) {
  budgetsLoading.set(true);
  budgetsError.set(null);
  try {
    const data = await budgetsApi.list(params);
    budgets.set(data);
  } catch (err: unknown) {
    budgetsError.set(err instanceof Error ? err.message : 'Failed to load budgets');
  } finally {
    budgetsLoading.set(false);
  }
}

export async function createBudget(data: BudgetInput) {
  const result = await budgetsApi.create(data);
  budgets.update((list) => {
    const idx = list.findIndex(
      (b) => b.category === result.category && b.month === result.month && b.year === result.year
    );
    if (idx >= 0) {
      list[idx] = result;
      return [...list];
    }
    return [result, ...list];
  });
  return result;
}

export async function deleteBudget(id: string) {
  await budgetsApi.delete(id);
  budgets.update((list) => list.filter((b) => b.id !== id));
}

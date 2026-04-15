<script lang="ts">
  import type { Budget } from '$lib/api/client';
  import { formatRupiah } from '$lib/utils/currency';
  import { deleteBudget } from '$lib/stores/budgets';
  import { addToast } from '$lib/stores/ui';

  let { budget, onedit }: { budget: Budget, onedit?: (b: Budget) => void } = $props();

  let spent = $derived(parseFloat(budget.spent_amount));
  let limit = $derived(parseFloat(budget.limit_amount));
  let percentage = $derived(limit > 0 ? Math.round((spent / limit) * 100) : 0);
  let remaining = $derived(limit - spent);
  let isOverLimit = $derived(spent > limit);
  let isWarning = $derived(percentage >= 80 && !isOverLimit);

  let loading = $state(false);

  // Map backend categories to Indonesian
  const categoryLabels: Record<string, string> = {
    'Food & Dining': 'Makan & Minum',
    'Transport': 'Transportasi',
    'Shopping': 'Belanja',
    'Entertainment': 'Hiburan',
    'Groceries': 'Kebutuhan Harian',
    'Bills & Utilities': 'Tagihan',
    'Health': 'Kesehatan',
    'Education': 'Edukasi',
    'Other': 'Lainnya',
  };

  let displayCategory = $derived(categoryLabels[budget.category] || budget.category);

  async function handleDelete() {
    if (!budget.user_id) {
      addToast('Cannot delete demo budget', 'error');
      return;
    }
    if (confirm(`Hapus anggaran untuk kategori ${displayCategory}?`)) {
      loading = true;
      try {
        await deleteBudget(budget.id);
        addToast('Anggaran berhasil dihapus', 'success');
      } catch (e: any) {
        addToast(e.message || 'Gagal menghapus anggaran', 'error');
      } finally {
        loading = false;
      }
    }
  }
</script>

<div class="bg-surface-container-high rounded-lg p-6 relative group">
  {#if budget.user_id}
    <div class="absolute top-4 right-4 flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
      <button class="text-on-surface-variant hover:text-primary transition-colors disabled:opacity-50" onclick={() => onedit?.(budget)} disabled={loading}>
        <span class="material-symbols-outlined text-[18px]">edit</span>
      </button>
      <button class="text-on-surface-variant hover:text-secondary transition-colors disabled:opacity-50" onclick={handleDelete} disabled={loading}>
        <span class="material-symbols-outlined text-[18px]">delete</span>
      </button>
    </div>
  {/if}

  <div class="flex justify-between items-start mb-4">
    <div class="flex items-center gap-4">
      <div class="w-12 h-12 rounded-full bg-surface-container-highest flex items-center justify-center">
        <span
          class="material-symbols-outlined"
          class:text-secondary={isOverLimit || isWarning}
          class:text-primary={!isOverLimit && !isWarning}
          style="font-variation-settings: 'FILL' 1;"
        >
          {budget.icon || 'category'}
        </span>
      </div>
      <div>
        <h3 class="font-bold text-on-surface">{displayCategory}</h3>
        {#if isOverLimit}
          <p class="text-xs text-secondary font-medium mt-0.5 flex items-center gap-1">
            <span class="material-symbols-outlined text-sm">error</span> Melebihi batas
          </p>
        {:else if isWarning}
          <p class="text-xs text-secondary font-medium mt-0.5 flex items-center gap-1">
            <span class="material-symbols-outlined text-sm">warning</span> {percentage}% terpakai
          </p>
        {:else}
          <p class="text-xs text-on-surface-variant font-medium mt-0.5">{percentage}% terpakai</p>
        {/if}
      </div>
    </div>
    <div class="text-right">
      <p class="font-bold" class:text-secondary={isOverLimit} class:text-on-surface={!isOverLimit}>
        {formatRupiah(spent)}
      </p>
      <p class="text-xs text-on-surface-variant">dari {formatRupiah(limit)}</p>
    </div>
  </div>
  <div class="w-full bg-surface-container-highest h-1.5 rounded-full overflow-hidden">
    <div
      class="h-full rounded-full transition-all duration-500"
      class:bg-primary={!isOverLimit && !isWarning}
      class:bg-secondary={isOverLimit || isWarning}
      style="width: {Math.min(percentage, 100)}%"
    ></div>
  </div>
  <div class="flex justify-between mt-3 text-[10px] uppercase tracking-widest text-on-surface-variant font-medium">
    <span>Terpakai: {formatRupiah(spent)}</span>
    {#if isOverLimit}
      <span class="text-secondary">Lebih: {formatRupiah(Math.abs(remaining))}</span>
    {:else}
      <span class="text-primary">Sisa: {formatRupiah(remaining)}</span>
    {/if}
  </div>
</div>

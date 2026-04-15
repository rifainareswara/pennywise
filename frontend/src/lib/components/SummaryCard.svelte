<script lang="ts">
  import { formatRupiah } from '$lib/utils/currency';

  let { type, amount, percentage = 0 }: {
    type: 'income' | 'expense';
    amount: string;
    percentage?: number;
  } = $props();

  let isIncome = $derived(type === 'income');
</script>

<div class="bg-surface-container-high p-6 rounded-xl flex items-center justify-between">
  <div class="flex items-center gap-4">
    <div class="w-12 h-12 rounded-full flex items-center justify-center {isIncome ? 'bg-primary/10 text-primary' : 'bg-secondary/10 text-secondary'}">
      <span class="material-symbols-outlined" style="font-variation-settings: 'FILL' 1;">
        {isIncome ? 'arrow_downward' : 'arrow_upward'}
      </span>
    </div>
    <div>
      <p class="text-on-surface-variant text-xs uppercase tracking-widest font-label mb-1">
        {isIncome ? 'Pemasukan' : 'Pengeluaran'}
      </p>
      <p class="text-on-surface font-headline text-xl font-bold">{formatRupiah(amount)}</p>
    </div>
  </div>
  <div class="h-1 w-20 bg-surface-container-highest rounded-full overflow-hidden">
    <div class="h-full rounded-full {isIncome ? 'bg-primary' : 'bg-secondary'}"
      style="width: {Math.min(percentage, 100)}%"
    ></div>
  </div>
</div>

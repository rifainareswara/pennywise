<script lang="ts">
  import type { Transaction } from '$lib/api/client';
  import { formatRupiah } from '$lib/utils/currency';

  let { transaction, onclick }: {
    transaction: Transaction;
    onclick?: () => void;
  } = $props();

  let isIncome = $derived(transaction.transaction_type === 'income');
  let formattedAmount = $derived(() => {
    const n = parseFloat(transaction.amount);
    return isIncome
      ? `+${formatRupiah(n)}`
      : `-${formatRupiah(Math.abs(n))}`;
  });
  let formattedDate = $derived(() => {
    const d = new Date(transaction.date);
    const now = new Date();
    if (d.toDateString() === now.toDateString()) {
      return `Hari ini, ${d.toLocaleTimeString('id-ID', { hour: 'numeric', minute: '2-digit' })}`;
    }
    return d.toLocaleDateString('id-ID', { day: 'numeric', month: 'short', year: 'numeric' });
  });
</script>

<button
  {onclick}
  class="w-full group relative bg-surface-container-low hover:bg-surface-container-high rounded-xl p-4 flex items-center justify-between transition-all duration-300 text-left"
>
  <div class="flex items-center gap-4">
    <div class="w-12 h-12 rounded-xl bg-surface-container-highest flex items-center justify-center text-on-surface-variant">
      <span class="material-symbols-outlined">{transaction.icon || 'payments'}</span>
    </div>
    <div>
      <p class="text-on-surface font-medium leading-none">{transaction.description}</p>
      <p class="text-on-surface-variant text-xs mt-1">{transaction.category} • {formattedDate()}</p>
    </div>
  </div>
  <div class="flex flex-col items-end gap-2">
    <p class="font-headline font-bold" class:text-primary={isIncome} class:text-secondary={!isIncome}>
      {formattedAmount()}
    </p>
    <div class="w-4 h-1 rounded-full bg-outline-variant/30 group-hover:bg-primary/40 transition-colors"></div>
  </div>
</button>

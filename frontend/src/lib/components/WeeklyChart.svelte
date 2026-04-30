<script lang="ts">
  import { formatRupiah } from '$lib/utils/currency';

  let { incomeData = [], expenseData = [], avgIncome = 0, avgExpense = 0 }: {
    incomeData?: number[];
    expenseData?: number[];
    avgIncome?: number;
    avgExpense?: number;
  } = $props();

  const days = ['Sen', 'Sel', 'Rab', 'Kam', 'Jum', 'Sab', 'Min'];

  let maxVal = $derived(Math.max(...incomeData, ...expenseData, 1));

  let bars = $derived(
    days.map((day, i) => ({
      day,
      income: incomeData[i] ?? 0,
      expense: expenseData[i] ?? 0,
      incomeHeight: Math.max(((incomeData[i] ?? 0) / maxVal) * 100, (incomeData[i] ?? 0) > 0 ? 4 : 0),
      expenseHeight: Math.max(((expenseData[i] ?? 0) / maxVal) * 100, (expenseData[i] ?? 0) > 0 ? 4 : 0),
    }))
  );
</script>

<section class="bg-surface-container-low p-6 rounded-xl">
  <div class="flex justify-between items-start mb-6">
    <h3 class="font-headline text-xl font-bold text-on-surface">Aktivitas Mingguan</h3>
    <div class="flex flex-col gap-1.5 items-end">
      <div class="flex items-center gap-1.5">
        <span class="w-2.5 h-2.5 rounded-full bg-primary"></span>
        <span class="text-[11px] text-on-surface-variant">Pemasukan</span>
      </div>
      <div class="flex items-center gap-1.5">
        <span class="w-2.5 h-2.5 rounded-full bg-secondary"></span>
        <span class="text-[11px] text-on-surface-variant">Pengeluaran</span>
      </div>
    </div>
  </div>

  <!-- Averages -->
  <div class="flex gap-6 mb-6">
    <div>
      <p class="text-[10px] uppercase tracking-widest text-on-surface-variant font-label mb-0.5">Rata-rata / hari</p>
      <p class="text-sm font-bold text-primary">{formatRupiah(avgIncome)}</p>
    </div>
    <div class="w-px bg-outline-variant/30"></div>
    <div>
      <p class="text-[10px] uppercase tracking-widest text-on-surface-variant font-label mb-0.5">Rata-rata / hari</p>
      <p class="text-sm font-bold text-secondary">{formatRupiah(avgExpense)}</p>
    </div>
  </div>

  <!-- Bar Chart -->
  <div class="flex items-end justify-between h-36 gap-1">
    {#each bars as bar}
      <div class="flex-1 flex flex-col items-center gap-1.5 h-full">
        <!-- Bars container -->
        <div class="w-full flex items-end justify-center gap-0.5 flex-1">
          <!-- Income bar -->
          <div class="flex-1 relative group">
            <div
              class="w-full rounded-t-md bg-primary/70 hover:bg-primary transition-colors"
              style="height: {bar.incomeHeight}%; min-height: {bar.income > 0 ? '3px' : '0px'};"
            ></div>
            {#if bar.income > 0}
              <span class="absolute -top-5 left-1/2 -translate-x-1/2 text-[9px] text-primary font-bold opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none">
                {formatRupiah(bar.income)}
              </span>
            {/if}
          </div>
          <!-- Expense bar -->
          <div class="flex-1 relative group">
            <div
              class="w-full rounded-t-md bg-secondary/70 hover:bg-secondary transition-colors"
              style="height: {bar.expenseHeight}%; min-height: {bar.expense > 0 ? '3px' : '0px'};"
            ></div>
            {#if bar.expense > 0}
              <span class="absolute -top-5 left-1/2 -translate-x-1/2 text-[9px] text-secondary font-bold opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none">
                {formatRupiah(bar.expense)}
              </span>
            {/if}
          </div>
        </div>
        <!-- Day label -->
        <span class="text-[10px] font-label text-on-surface-variant">{bar.day}</span>
      </div>
    {/each}
  </div>
</section>

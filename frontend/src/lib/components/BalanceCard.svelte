<script lang="ts">
  import { formatRupiah } from '$lib/utils/currency';

  let { balance = '0', changePercent = 0 }: {
    balance?: string;
    changePercent?: number;
  } = $props();

  let isPositive = $derived(changePercent >= 0);
</script>

<section class="mt-4">
  <div class="bg-surface-container-low p-8 rounded-xl relative overflow-hidden group">
    <!-- Background decoration -->
    <div class="absolute -right-12 -top-12 w-48 h-48 rounded-full blur-3xl" style="background: rgba(84,233,138,0.05);"></div>
    <div class="relative z-10">
      <p class="text-on-surface-variant font-medium mb-2 opacity-80">Total Saldo</p>
      <div class="flex items-baseline gap-1">
        <span class="text-primary font-headline text-2xl font-bold">Rp</span>
        <h2 class="text-on-surface font-headline text-[3rem] leading-none font-extrabold tracking-tighter">{parseFloat(balance).toLocaleString('id-ID')}</h2>
      </div>
      <div class="mt-8 flex items-center gap-2">
        <div class="flex items-center text-sm font-semibold px-3 py-1 rounded-full {isPositive ? 'text-primary bg-primary/10' : 'text-secondary bg-secondary/10'}">
          <span class="material-symbols-outlined text-sm mr-1">
            {isPositive ? 'trending_up' : 'trending_down'}
          </span>
          {isPositive ? '+' : ''}{changePercent.toFixed(1)}%
        </div>
        <span class="text-on-surface-variant text-xs font-label">vs bulan lalu</span>
      </div>
    </div>
  </div>
</section>

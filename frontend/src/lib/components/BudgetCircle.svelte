<script lang="ts">
  import { formatRupiah } from '$lib/utils/currency';

  let { percentage = 0, spent = '0', total = '0' }: {
    percentage?: number;
    spent?: string;
    total?: string;
  } = $props();

  // SVG circle math
  const radius = 88;
  const circumference = 2 * Math.PI * radius;
  let offset = $derived(circumference - (Math.min(percentage, 100) / 100) * circumference);
</script>

<section class="mb-10">
  <div class="bg-surface-container-low rounded-xl p-8 relative overflow-hidden">
    <!-- Decorative element -->
    <div class="absolute -top-12 -right-12 w-48 h-48 bg-primary/10 rounded-full blur-[64px]"></div>
    <div class="flex flex-col items-center text-center">
      <div class="relative w-48 h-48 flex items-center justify-center">
        <!-- SVG Circular Progress -->
        <svg class="w-full h-full transform -rotate-90">
          <circle
            class="text-surface-container-high"
            cx="96" cy="96" r={radius}
            fill="transparent" stroke="currentColor" stroke-width="12"
          />
          <circle
            class:text-primary={percentage <= 80}
            class:text-secondary={percentage > 80}
            cx="96" cy="96" r={radius}
            fill="transparent" stroke="currentColor" stroke-width="12"
            stroke-dasharray={circumference}
            stroke-dashoffset={offset}
            stroke-linecap="round"
            style="transition: stroke-dashoffset 1s ease-in-out;"
          />
        </svg>
        <div class="absolute inset-0 flex flex-col items-center justify-center">
          <span class="text-[3.5rem] font-bold text-on-surface font-headline tracking-tighter">{Math.round(percentage)}%</span>
          <span class="text-xs uppercase tracking-[0.2em] text-on-surface-variant font-medium">Terpakai</span>
        </div>
      </div>
      <div class="mt-8 grid grid-cols-2 gap-8 w-full">
        <div class="text-left">
          <p class="text-xs uppercase tracking-widest text-on-surface-variant mb-1">Terpakai</p>
          <p class="text-2xl font-bold font-headline text-on-surface">{formatRupiah(spent)}</p>
        </div>
        <div class="text-right">
          <p class="text-xs uppercase tracking-widest text-on-surface-variant mb-1">Total Limit</p>
          <p class="text-2xl font-bold font-headline text-on-surface">{formatRupiah(total)}</p>
        </div>
      </div>
    </div>
  </div>
</section>

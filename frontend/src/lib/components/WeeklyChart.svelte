<script lang="ts">
  import { formatRupiah } from '$lib/utils/currency';

  let { data = [], average = '0' }: {
    data?: number[];
    average?: string;
  } = $props();

  const days = ['Sen', 'Sel', 'Rab', 'Kam', 'Jum', 'Sab', 'Min'];

  let maxVal = $derived(Math.max(...data, 1));
  let bars = $derived(
    data.map((val, i) => ({
      day: days[i] || `H${i + 1}`,
      height: Math.max((val / maxVal) * 100, 5),
      value: val,
      isHighest: val === maxVal && val > 0,
    }))
  );
</script>

<section class="bg-surface-container-low p-8 rounded-xl">
  <div class="flex justify-between items-end mb-8">
    <div>
      <h3 class="font-headline text-xl font-bold text-on-surface">Aktivitas Mingguan</h3>
      <p class="text-on-surface-variant text-sm mt-1">Rata-rata {formatRupiah(average)} / hari</p>
    </div>
    <div class="flex gap-2">
      <span class="w-2 h-2 rounded-full bg-primary"></span>
      <span class="w-2 h-2 rounded-full bg-surface-container-highest"></span>
    </div>
  </div>
  <!-- Bar Chart -->
  <div class="flex items-end justify-between h-40 gap-3">
    {#each bars as bar}
      <div
        class="flex-1 rounded-t-lg relative group cursor-pointer transition-colors {bar.isHighest ? 'bg-primary' : 'bg-surface-container-high hover:bg-surface-container-highest'}"
        style="height: {bar.height}%;"
      >
        <span
          class="absolute -top-8 left-1/2 -translate-x-1/2 text-[10px] font-label transition-opacity {bar.isHighest ? 'text-on-surface font-bold' : 'text-on-surface-variant opacity-0 group-hover:opacity-100'}"
        >
          {bar.day}
        </span>
      </div>
    {/each}
  </div>
</section>

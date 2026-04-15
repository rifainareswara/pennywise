<script lang="ts">
  import { createBudget } from '$lib/stores/budgets';
  import { addToast } from '$lib/stores/ui';

  import type { Budget } from '$lib/api/client';

  let { onclose, budget }: { onclose?: () => void; budget?: Budget | null } = $props();

  let loading = $state(false);
  let category = $state(budget?.category || '');
  let limitAmount = $state(budget ? parseFloat(budget.limit_amount).toString() : '');
  let icon = $state(budget?.icon || 'category');

  const now = new Date();
  let month = $state(budget ? budget.month : now.getMonth() + 1);
  let year = $state(budget ? budget.year : now.getFullYear());

  const categories = [
    { label: 'Makan & Minum', value: 'Food & Dining', icon: 'restaurant' },
    { label: 'Transportasi', value: 'Transport', icon: 'directions_car' },
    { label: 'Belanja', value: 'Shopping', icon: 'shopping_bag' },
    { label: 'Hiburan', value: 'Entertainment', icon: 'movie' },
    { label: 'Kebutuhan Harian', value: 'Groceries', icon: 'shopping_cart' },
    { label: 'Tagihan', value: 'Bills & Utilities', icon: 'receipt' },
    { label: 'Kesehatan', value: 'Health', icon: 'health_and_safety' },
    { label: 'Edukasi', value: 'Education', icon: 'school' },
    { label: 'Lainnya', value: 'Other', icon: 'category' },
  ];

  function selectCategory(cat: typeof categories[0]) {
    category = cat.value;
    icon = cat.icon;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!category || !limitAmount) {
      addToast('Please fill in all fields', 'error');
      return;
    }

    loading = true;
    try {
      await createBudget({
        category,
        limit_amount: parseFloat(limitAmount),
        icon,
        month,
        year,
      });
      addToast('Budget set successfully', 'success');
      onclose?.();
    } catch (err) {
      addToast(err instanceof Error ? err.message : 'Failed to create budget', 'error');
    } finally {
      loading = false;
    }
  }
</script>

<!-- Backdrop -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="fixed inset-0 bg-black/60 backdrop-blur-sm z-[100] flex items-end justify-center" onclick={(e) => { if (e.target === e.currentTarget) onclose?.(); }}>
  <div
    class="bg-surface-container w-full max-w-lg rounded-t-[2rem] p-6 pb-10 max-h-[85vh] overflow-y-auto"
    style="animation: slideUp 0.3s ease-out;"
  >
    <div class="flex justify-between items-center mb-6">
      <h2 class="font-headline text-xl font-bold text-on-surface">Atur Anggaran</h2>
      <button onclick={onclose} class="text-on-surface-variant hover:text-on-surface transition-colors">
        <span class="material-symbols-outlined">close</span>
      </button>
    </div>

    <form onsubmit={handleSubmit} class="space-y-6">
      <!-- Category Grid -->
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

      <!-- Limit Amount -->
      <div>
        <label class="text-xs uppercase tracking-widest text-on-surface-variant font-label mb-2 block">Batas Anggaran</label>
        <div class="relative">
          <span class="absolute left-4 top-1/2 -translate-y-1/2 text-on-surface-variant font-headline text-xl font-bold">Rp</span>
          <input
            type="number"
            step="100"
            min="0"
            bind:value={limitAmount}
            placeholder="0"
            class="w-full bg-surface-container-low border-none rounded-xl py-4 pl-12 pr-4 text-on-surface font-headline text-2xl font-bold placeholder:text-outline/40 focus:ring-1 focus:ring-primary/40 transition-all outline-none"
          />
        </div>
      </div>

      <!-- Month/Year -->
      <div class="grid grid-cols-2 gap-3">
        <div>
          <label class="text-xs uppercase tracking-widest text-on-surface-variant font-label mb-2 block">Bulan</label>
          <select bind:value={month} class="w-full bg-surface-container-low border-none rounded-xl py-4 px-4 text-on-surface focus:ring-1 focus:ring-primary/40 outline-none">
            {#each Array.from({length: 12}, (_, i) => i + 1) as m}
              <option value={m}>{new Date(2000, m - 1).toLocaleString('id-ID', { month: 'long' })}</option>
            {/each}
          </select>
        </div>
        <div>
          <label class="text-xs uppercase tracking-widest text-on-surface-variant font-label mb-2 block">Tahun</label>
          <select bind:value={year} class="w-full bg-surface-container-low border-none rounded-xl py-4 px-4 text-on-surface focus:ring-1 focus:ring-primary/40 outline-none">
            {#each [2024, 2025, 2026, 2027] as y}
              <option value={y}>{y}</option>
            {/each}
          </select>
        </div>
      </div>

      <!-- Submit -->
      <button
        type="submit"
        disabled={loading}
        class="w-full py-4 editorial-gradient rounded-full text-on-primary font-bold tracking-tight active:scale-95 duration-200 shadow-xl shadow-primary/10 disabled:opacity-50"
      >
        {loading ? 'Menyimpan...' : 'Atur Anggaran'}
      </button>
    </form>
  </div>
</div>

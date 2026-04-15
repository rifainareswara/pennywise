<script lang="ts">
  let { filters = [], active = '', onselect }: {
    filters?: Array<{ label: string; value: string }>;
    active?: string;
    onselect?: (value: string) => void;
  } = $props();
</script>

<div class="flex gap-3 overflow-x-auto no-scrollbar pb-2">
  {#each filters as filter}
    <button
      class="px-5 py-2.5 rounded-full text-sm font-medium whitespace-nowrap active:scale-95 transition-all"
      class:bg-primary={active === filter.value}
      class:text-on-primary={active === filter.value}
      class:font-semibold={active === filter.value}
      class:bg-surface-container-high={active !== filter.value}
      class:text-on-surface={active !== filter.value}
      onclick={() => onselect?.(filter.value)}
    >
      {filter.label}
    </button>
  {/each}
</div>

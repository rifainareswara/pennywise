<script lang="ts">
  let { value = '', placeholder = 'Search...', oninput }: {
    value?: string;
    placeholder?: string;
    oninput?: (e: Event) => void;
  } = $props();
</script>

<div class="relative group">
  <span class="material-symbols-outlined absolute left-4 top-1/2 -translate-y-1/2 text-outline">search</span>
  <input
    type="text"
    class="w-full bg-surface-container-low border-none rounded-xl py-4 pl-12 pr-4 text-on-surface placeholder:text-outline focus:ring-1 focus:ring-primary/40 transition-all outline-none"
    {placeholder}
    {value}
    {oninput}
  />
</div>

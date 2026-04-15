<script lang="ts">
  let { icon = 'inbox', title = 'Nothing here yet', message = 'Get started by adding your first item.' }: {
    icon?: string;
    title?: string;
    message?: string;
  } = $props();
</script>

<div class="flex flex-col items-center justify-center py-16 text-center">
  <div class="w-20 h-20 rounded-full bg-surface-container-high flex items-center justify-center mb-6">
    <span class="material-symbols-outlined text-4xl text-on-surface-variant">{icon}</span>
  </div>
  <h3 class="font-headline text-lg font-bold text-on-surface mb-2">{title}</h3>
  <p class="text-on-surface-variant text-sm max-w-xs">{message}</p>
</div>

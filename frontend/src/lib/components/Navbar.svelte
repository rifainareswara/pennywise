<script lang="ts">
  let { title = 'PennyWise', showAvatar = true, actions = [] }: {
    title?: string;
    showAvatar?: boolean;
    actions?: Array<{ icon: string; onClick?: () => void; href?: string }>;
  } = $props();
</script>

<header class="w-full top-0 sticky z-50 bg-[#131313] flex justify-between items-center px-6 py-4">
  <div class="flex items-center gap-3">
    {#if showAvatar}
      <div class="w-10 h-10 rounded-full overflow-hidden bg-surface-container-high flex items-center justify-center active:scale-95 duration-200">
        <div class="w-full h-full bg-primary/20 flex items-center justify-center">
          <span class="material-symbols-outlined text-primary text-lg" style="font-variation-settings: 'FILL' 1;">person</span>
        </div>
      </div>
    {/if}
    <div>
      {#if showAvatar}
        <span class="block text-on-surface-variant font-label text-[10px] uppercase tracking-widest">{new Date().getHours() < 12 ? 'Selamat Pagi' : new Date().getHours() < 15 ? 'Selamat Siang' : new Date().getHours() < 18 ? 'Selamat Sore' : 'Selamat Malam'}</span>
      {/if}
      <h1 class="font-headline text-lg font-bold tracking-tight text-primary">{title}</h1>
    </div>
  </div>
  <div class="flex items-center gap-4">
    {#each actions as action}
      {#if action.href}
        <a href={action.href} class="text-on-background hover:opacity-80 transition-opacity active:scale-95 duration-200">
          <span class="material-symbols-outlined">{action.icon}</span>
        </a>
      {:else}
        <button onclick={action.onClick} class="text-on-background hover:opacity-80 transition-opacity active:scale-95 duration-200">
          <span class="material-symbols-outlined">{action.icon}</span>
        </button>
      {/if}
    {/each}
  </div>
</header>

<script lang="ts">
  import { page } from '$app/stores';

  const tabs = [
    { icon: 'dashboard', label: 'Beranda', href: '/dashboard', filled: true },
    { icon: 'receipt_long', label: 'Riwayat', href: '/transactions', filled: true },
    { icon: 'add', label: '', href: '/transactions/new', isPlaceholder: true },
    { icon: 'pie_chart', label: 'Anggaran', href: '/budget', filled: false },
    { icon: 'person', label: 'Profil', href: '/profile', filled: false },
  ];

  let currentPath = $derived($page.url.pathname);
</script>

<nav class="fixed bottom-0 left-0 w-full z-50 flex justify-around items-end px-4 pb-6 bg-[#131313]/60 backdrop-blur-xl rounded-t-[24px] shadow-[0_-8px_32px_rgba(46,204,113,0.04)]">
  {#each tabs as tab}
    {#if tab.isPlaceholder}
      <div class="w-12 h-12 invisible"></div>
    {:else}
      <a href={tab.href} class="flex flex-col items-center group">
        {#if currentPath === tab.href || (tab.href !== '/' && currentPath.startsWith(tab.href))}
          <!-- Active tab -->
          <div class="flex items-center justify-center bg-primary text-background rounded-full w-12 h-12 mb-2 active:scale-90 duration-300 ease-out">
            <span class="material-symbols-outlined" style="font-variation-settings: 'FILL' 1;">{tab.icon}</span>
          </div>
          <span class="font-label text-[10px] uppercase tracking-widest text-primary">{tab.label}</span>
        {:else}
          <!-- Inactive tab -->
          <div class="flex items-center justify-center text-on-background/50 w-12 h-12 active:scale-90 duration-300 ease-out hover:text-primary transition-colors">
            <span class="material-symbols-outlined">{tab.icon}</span>
          </div>
          <span class="font-label text-[10px] uppercase tracking-widest text-on-background/50 group-hover:text-primary">{tab.label}</span>
        {/if}
      </a>
    {/if}
  {/each}
</nav>

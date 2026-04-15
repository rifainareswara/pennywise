<script lang="ts">
  import { toasts } from '$lib/stores/ui';
</script>

{#if $toasts.length > 0}
  <div class="fixed top-4 right-4 z-[200] flex flex-col gap-2 max-w-sm">
    {#each $toasts as toast (toast.id)}
      <div
        class="px-5 py-3 rounded-xl shadow-2xl text-sm font-medium flex items-center gap-3"
        class:bg-primary={toast.type === 'success'}
        class:text-on-primary={toast.type === 'success'}
        class:bg-error-container={toast.type === 'error'}
        class:text-on-error-container={toast.type === 'error'}
        class:bg-surface-container-high={toast.type === 'info'}
        class:text-on-surface={toast.type === 'info'}
        style="animation: toastIn 0.3s ease-out;"
      >
        <span class="material-symbols-outlined text-lg">
          {#if toast.type === 'success'}check_circle
          {:else if toast.type === 'error'}error
          {:else}info
          {/if}
        </span>
        {toast.message}
      </div>
    {/each}
  </div>
{/if}

<script lang="ts">
  import '../app.css';
  import Toast from '$lib/components/Toast.svelte';
  import { loadUser, authLoading } from '$lib/stores/auth';
  import { onMount } from 'svelte';

  let { children } = $props();

  onMount(() => {
    loadUser();
  });
</script>

{#if $authLoading}
  <div class="h-screen w-full bg-background flex items-center justify-center">
    <div class="flex flex-col items-center gap-4">
      <div class="w-12 h-12 rounded-full border-2 border-surface-container-highest border-t-primary animate-spin"></div>
      <span class="text-on-surface-variant text-sm">Loading PennyWise...</span>
    </div>
  </div>
{:else}
  {@render children()}
{/if}

<Toast />

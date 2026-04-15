<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import BottomNav from '$lib/components/BottomNav.svelte';
  import { isAuthenticated } from '$lib/stores/auth';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  let { children } = $props();

  onMount(() => {
    if (!$isAuthenticated) {
      goto('/login');
    }
  });
</script>

{#if $isAuthenticated}
  <div class="bg-background min-h-screen pb-32 overflow-x-hidden">
    {@render children()}
    <BottomNav />
  </div>
{/if}

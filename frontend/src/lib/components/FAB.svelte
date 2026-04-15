<script lang="ts">
  let { icon = 'add', href = '/transactions/new' }: {
    icon?: string;
    href?: string;
  } = $props();
</script>

<a
  {href}
  class="fixed right-6 bottom-28 w-14 h-14 bg-gradient-to-br from-primary to-primary-container text-on-primary rounded-full shadow-2xl editorial-shadow flex items-center justify-center z-50 active:scale-90 transition-transform duration-300 group"
>
  <span class="material-symbols-outlined text-3xl group-hover:rotate-90 transition-transform duration-300">{icon}</span>
</a>

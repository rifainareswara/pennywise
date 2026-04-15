<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import TransactionForm from '$lib/components/TransactionForm.svelte';
</script>

<svelte:head>
  <title>Tambah Transaksi — PennyWise</title>
</svelte:head>

<Navbar title="Tambah Transaksi" showAvatar={false} actions={[{ icon: 'close', href: '/transactions' }]} />

<main class="px-6 pt-4 max-w-lg mx-auto page-transition">
  <TransactionForm mode="create" />
</main>

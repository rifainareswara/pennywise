<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import BudgetCircle from '$lib/components/BudgetCircle.svelte';
  import BudgetCategoryCard from '$lib/components/BudgetCategoryCard.svelte';
  import BudgetForm from '$lib/components/BudgetForm.svelte';
  import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import { loadBudgets, budgets, budgetsLoading } from '$lib/stores/budgets';
  import type { Budget } from '$lib/api/client';
  import { onMount } from 'svelte';

  let showForm = $state(false);
  let editBudget = $state<Budget | null>(null);

  // Demo budgets for when backend is unavailable
  const demoBudgets: Budget[] = [
    { id: '1', user_id: '', category: 'Makan & Minum', limit_amount: '1500000', spent_amount: '1380000', icon: 'restaurant', month: new Date().getMonth() + 1, year: new Date().getFullYear() },
    { id: '2', user_id: '', category: 'Transportasi', limit_amount: '800000', spent_amount: '360000', icon: 'directions_car', month: new Date().getMonth() + 1, year: new Date().getFullYear() },
    { id: '3', user_id: '', category: 'Hiburan', limit_amount: '500000', spent_amount: '310000', icon: 'movie', month: new Date().getMonth() + 1, year: new Date().getFullYear() },
    { id: '4', user_id: '', category: 'Belanja', limit_amount: '1000000', spent_amount: '1150000', icon: 'shopping_bag', month: new Date().getMonth() + 1, year: new Date().getFullYear() },
  ];

  let displayBudgets = $derived($budgets.length > 0 ? $budgets : demoBudgets);
  let totalSpent = $derived(displayBudgets.reduce((sum, b) => sum + parseFloat(b.spent_amount), 0));
  let totalLimit = $derived(displayBudgets.reduce((sum, b) => sum + parseFloat(b.limit_amount), 0));
  let overallPercent = $derived(totalLimit > 0 ? (totalSpent / totalLimit) * 100 : 0);

  const now = new Date();
  const monthName = now.toLocaleString('id-ID', { month: 'long' });

  onMount(() => {
    loadBudgets({ month: now.getMonth() + 1, year: now.getFullYear() });
  });

  function handleEdit(budget: Budget) {
    editBudget = budget;
    showForm = true;
  }

  function closeForm() {
    showForm = false;
    editBudget = null;
  }
</script>

<svelte:head>
  <title>Budget Planning — PennyWise</title>
</svelte:head>

<Navbar title="PennyWise" actions={[{ icon: 'notifications' }]} showAvatar={false} />

<main class="px-6 mt-4 max-w-2xl mx-auto page-transition">
  <!-- Screen Title -->
  <section class="mb-8">
    <h1 class="text-[1.75rem] font-bold text-on-surface font-headline leading-tight">Anggaran Bulanan</h1>
    <p class="text-on-surface-variant text-sm mt-1">Perencanaan {monthName} {now.getFullYear()}</p>
  </section>

  {#if $budgetsLoading}
    <LoadingSpinner />
  {:else}
    <!-- Overall Budget Usage -->
    <BudgetCircle
      percentage={overallPercent}
      spent={totalSpent.toString()}
      total={totalLimit.toString()}
    />

    <!-- Category Header -->
    <section class="mb-6 flex justify-between items-end">
      <h2 class="text-xl font-bold text-on-surface font-headline">Kategori</h2>
      <button
        class="text-primary text-sm font-semibold hover:opacity-80 transition-opacity"
        onclick={() => showForm = true}
      >
        Atur Anggaran Baru
      </button>
    </section>

    <!-- Category List -->
    <section class="space-y-4">
      {#if displayBudgets.length > 0}
        {#each displayBudgets as budget (budget.id)}
          <BudgetCategoryCard {budget} onedit={handleEdit} />
        {/each}
      {:else}
        <EmptyState icon="pie_chart" title="Belum ada anggaran" message="Atur anggaran pertama Anda untuk mulai melacak pengeluaran." />
      {/if}
    </section>

    <!-- Adjust Budget Button -->
    <div class="mt-12 mb-8">
      <button
        class="w-full py-4 editorial-gradient rounded-full text-on-primary font-bold tracking-tight active:scale-95 duration-200 shadow-xl shadow-primary/10"
        onclick={() => showForm = true}
      >
        Sesuaikan Batas Anggaran
      </button>
    </div>
  {/if}
</main>

{#if showForm}
  <BudgetForm onclose={closeForm} budget={editBudget} />
{/if}

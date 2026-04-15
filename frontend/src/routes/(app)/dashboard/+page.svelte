<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import BalanceCard from '$lib/components/BalanceCard.svelte';
  import SummaryCard from '$lib/components/SummaryCard.svelte';
  import WeeklyChart from '$lib/components/WeeklyChart.svelte';
  import TransactionItem from '$lib/components/TransactionItem.svelte';
  import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import { dashboardApi, type DashboardSummary, type Transaction } from '$lib/api/client';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  let loading = $state(true);
  let summary = $state<DashboardSummary | null>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      summary = await dashboardApi.summary();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to load dashboard';
      // Seed with demo data if backend not available
      summary = {
        total_balance: '42850000',
        total_income: '12500000',
        total_expenses: '4750000',
        balance_change_percent: 12.5,
        weekly_activity: [320000, 520000, 720000, 240000, 440000, 600000, 360000],
        recent_transactions: [
          { id: '1', user_id: '', amount: '185000', category: 'Groceries', description: 'Belanja Supermarket', transaction_type: 'expense', icon: 'shopping_cart', date: new Date().toISOString(), created_at: '' },
          { id: '2', user_id: '', amount: '54000', category: 'Entertainment', description: 'Netflix Premium', transaction_type: 'expense', icon: 'subscriptions', date: new Date(Date.now() - 86400000).toISOString(), created_at: '' },
          { id: '3', user_id: '', amount: '8500000', category: 'Salary', description: 'Gaji Bulanan', transaction_type: 'income', icon: 'payments', date: new Date(Date.now() - 172800000).toISOString(), created_at: '' },
          { id: '4', user_id: '', amount: '45000', category: 'Food & Dining', description: 'Kopi Starbucks', transaction_type: 'expense', icon: 'local_cafe', date: new Date(Date.now() - 259200000).toISOString(), created_at: '' },
        ],
      };
    } finally {
      loading = false;
    }
  });
</script>

<svelte:head>
  <title>Dashboard — PennyWise</title>
</svelte:head>

<Navbar title="PennyWise" actions={[{ icon: 'notifications' }]} />

<main class="px-6 space-y-8 max-w-5xl mx-auto page-transition">
  {#if loading}
    <LoadingSpinner />
  {:else if summary}
    <!-- Total Balance Card -->
    <BalanceCard balance={summary.total_balance} changePercent={summary.balance_change_percent} />

    <!-- Summary Grid -->
    <section class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <SummaryCard type="income" amount={summary.total_income} percentage={85} />
      <SummaryCard type="expense" amount={summary.total_expenses} percentage={42} />
    </section>

    <!-- Weekly Activity Chart -->
    <WeeklyChart data={summary.weekly_activity} average="457000" />

    <!-- Recent Transactions -->
    <section class="space-y-4">
      <div class="flex justify-between items-center">
        <h3 class="font-headline text-xl font-bold text-on-surface">Aktivitas Terbaru</h3>
        <a href="/transactions" class="text-primary text-sm font-semibold hover:underline">Lihat Semua</a>
      </div>
      <div class="space-y-3">
        {#if summary.recent_transactions.length > 0}
          {#each summary.recent_transactions as tx (tx.id)}
            <TransactionItem transaction={tx} onclick={() => goto(`/transactions/${tx.id}/edit`)} />
          {/each}
        {:else}
          <EmptyState icon="receipt_long" title="No transactions yet" message="Add your first transaction to see your activity here." />
        {/if}
      </div>
    </section>
  {/if}
</main>

<script lang="ts">
  import { onMount } from 'svelte';
  import { slide } from 'svelte/transition';
  import { fetchTasks, createTask, updateTask, deleteTask } from '$lib/services/tasks';
  import type { Task } from '$lib/services/tasks';
  import TaskCard from '$lib/components/TaskCard.svelte';
  import TaskDrawer from '$lib/components/TaskDrawer.svelte';
  import { addToast } from '$lib/stores/ui';
  
  let showCompleted = $state(false);

  let tasks = $state<Task[]>([]);
  let isLoading = $state(true);

  // Drawer state
  let isDrawerOpen = $state(false);
  let editingTask = $state<Task | null>(null);

  // Filter state
  let currentFilter = $state('semua');
  
  let dynamicFilters = $derived.by(() => {
    const base = [
      { id: 'semua', label: 'Semua', icon: 'filter_list' },
      { id: 'penting', label: 'Penting', icon: 'local_fire_department' },
    ];

    // Collect distinct task types
    if (tasks.some(t => t.task_type === 'shopping')) base.push({ id: 'type:shopping', label: 'Belanja', icon: 'shopping_cart' });
    if (tasks.some(t => t.task_type === 'billing')) base.push({ id: 'type:billing', label: 'Tagihan', icon: 'receipt_long' });
    if (tasks.some(t => t.task_type === 'debt')) base.push({ id: 'type:debt', label: 'Hutang', icon: 'sync_alt' });

    // Collect distinct custom categories
    const categories = new Set(tasks.map(t => (t.category || '').trim()).filter(c => c !== '' && c.toLowerCase() !== 'general'));
    for (const cat of categories) {
      base.push({ id: `cat:${cat.toLowerCase()}`, label: cat, icon: 'label' });
    }

    return base;
  });

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      isLoading = true;
      tasks = await fetchTasks();
    } catch (error) {
      addToast('Gagal memuat tugas', 'error');
    } finally {
      isLoading = false;
    }
  }

  let filteredTasks = $derived(tasks.filter(t => {
    if (currentFilter === 'semua') return true;
    if (currentFilter === 'penting') return t.priority === 'urgent' || t.priority === 'high';
    
    if (currentFilter.startsWith('type:')) {
      return t.task_type === currentFilter.split(':')[1];
    }
    
    if (currentFilter.startsWith('cat:')) {
      const targetCat = currentFilter.split(':')[1];
      return (t.category || '').trim().toLowerCase() === targetCat;
    }

    return true;
  }));

  let pendingTasks = $derived(filteredTasks.filter(t => t.status !== 'completed'));
  let completedTasks = $derived(filteredTasks.filter(t => t.status === 'completed'));

  async function handleSaveTask(data: Partial<Task>) {
    try {
      if (data.id) {
        await updateTask(data.id, data);
        addToast('Tugas diperbarui', 'success');
      } else {
        await createTask(data);
        addToast('Tugas ditambahkan', 'success');
      }
      await loadData();
    } catch (e) {
      addToast('Gagal menyimpan tugas', 'error');
    }
  }

  async function toggleComplete(task: Task) {
    try {
      const newStatus = task.status === 'completed' ? 'pending' : 'completed';
      await updateTask(task.id, { status: newStatus });
      await loadData();
    } catch (e) {
      addToast('Gagal mengupdate status', 'error');
    }
  }

  function openNewTask() {
    editingTask = null;
    isDrawerOpen = true;
  }

  function openEditTask(task: Task) {
    editingTask = task;
    isDrawerOpen = true;
  }
</script>

<svelte:head>
  <title>Tugas Pintar - PennyWise</title>
</svelte:head>

<div class="px-6 pt-12 pb-32 min-h-screen">
  <div class="flex justify-between items-center mb-8">
    <div>
      <h1 class="text-3xl font-title text-on-background leading-tight">Daftar Tugas</h1>
      <p class="text-sm text-on-surface-variant font-label mt-1">Selesaikan pekerjaan dan tagihan tepat waktu</p>
    </div>
  </div>

  <!-- Filter Pills -->
  <div class="flex gap-2 mb-6 overflow-x-auto no-scrollbar pb-2">
    {#each dynamicFilters as filter}
      <button 
        class="flex items-center gap-1.5 px-4 py-2 rounded-full border text-sm font-medium transition-all whitespace-nowrap {currentFilter === filter.id ? 'bg-primary border-primary text-on-primary' : 'border-outline text-on-surface-variant hover:border-primary/50 hover:text-primary'}"
        onclick={() => currentFilter = filter.id}
      >
        <span class="material-symbols-outlined text-[18px]">{filter.icon}</span>
        {filter.label}
      </button>
    {/each}
  </div>

  {#if isLoading}
    <div class="flex justify-center items-center py-20">
      <div class="w-8 h-8 rounded-full border-2 border-outline border-t-primary animate-spin"></div>
    </div>
  {:else}
    <div class="flex flex-col gap-3">
      {#if pendingTasks.length === 0 && completedTasks.length === 0}
        <div class="text-center py-20 flex flex-col items-center gap-3">
          <span class="material-symbols-outlined text-[64px] text-outline opacity-50">task</span>
          <p class="text-on-surface-variant font-medium">Belum ada tugas.</p>
        </div>
      {/if}

      <!-- Pending Tasks -->
      {#each pendingTasks as task}
        <TaskCard 
          {task} 
          onclick={() => openEditTask(task)} 
          onComplete={() => toggleComplete(task)} 
        />
      {/each}

      <!-- Completed tasks collapsible logic or simply append them below -->
      {#if completedTasks.length > 0}
        <div class="mt-6">
          <button 
            class="w-full text-sm font-label text-on-surface-variant uppercase tracking-widest mb-3 flex items-center gap-2 outline-none group active:scale-95 transition-transform"
            onclick={() => showCompleted = !showCompleted}
          >
            <span class="w-full h-[1px] bg-outline flex-1 transition-colors group-hover:bg-primary/50"></span>
            <span class="flex items-center gap-1 group-hover:text-primary transition-colors">
              Selesai ({completedTasks.length})
              <span class="material-symbols-outlined text-[16px] transition-transform duration-300 {showCompleted ? 'rotate-180' : ''}">expand_more</span>
            </span>
            <span class="w-full h-[1px] bg-outline flex-1 transition-colors group-hover:bg-primary/50"></span>
          </button>
          
          {#if showCompleted}
            <div class="flex flex-col gap-3 opacity-60" transition:slide={{ duration: 300 }}>
              {#each completedTasks as task}
                <TaskCard 
                  {task} 
                  onclick={() => openEditTask(task)} 
                  onComplete={() => toggleComplete(task)} 
                />
              {/each}
            </div>
          {/if}
        </div>
      {/if}
    </div>
  {/if}
</div>

<!-- Floating Action Button for New Task -->
<button 
  class="fixed right-6 bottom-[100px] w-14 h-14 bg-gradient-to-br from-primary to-primary-container text-on-primary rounded-[20px] shadow-xl shadow-primary/30 flex items-center justify-center active:scale-95 transition-all z-40"
  onclick={openNewTask}
>
  <span class="material-symbols-outlined text-[28px]">add_task</span>
</button>

<TaskDrawer 
  bind:isOpen={isDrawerOpen} 
  task={editingTask} 
  onSave={handleSaveTask} 
  onClose={() => isDrawerOpen = false} 
/>

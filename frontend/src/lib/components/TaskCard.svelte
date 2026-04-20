<script lang="ts">
  import type { Task } from '$lib/services/tasks';
  import { formatRupiah } from '$lib/utils/currency';

  let { task, onclick, onComplete } = $props<{ task: Task; onclick: () => void; onComplete: () => void }>();

  // Helper flags for special styling
  let isImportant = $derived(task.priority === 'urgent' || task.priority === 'high');
  let priorityLabel = $derived(
    task.priority === 'urgent' ? 'Sangat Penting' :
    task.priority === 'high' ? 'Penting' :
    task.priority === 'low' ? 'Rendah' : 'Sedang'
  );

  // Dynamic styling based on category
  let categoryColorGroup = $derived(
    task.category.toLowerCase().includes('programming') ? 'bg-purple-500/10 text-purple-400 border-purple-500/20' :
    task.category.toLowerCase().includes('kuliah') ? 'bg-blue-500/10 text-blue-400 border-blue-500/20' :
    'bg-surface-container-highest text-on-surface-variant border-transparent'
  );
</script>

<button 
  class="relative w-full text-left bg-surface rounded-[20px] p-4 border transition-all duration-300 {categoryColorGroup} hover:border-primary/50 group active:scale-[0.98] cursor-pointer"
  onclick={onclick}
  type="button"
>
  <div class="flex justify-between items-start mb-3">
    <div class="flex flex-col gap-1">
      <h3 class="font-title text-[16px] text-on-surface leading-tight" class:line-through={task.status === 'completed'}>
        {task.title}
      </h3>
      <div class="flex items-center gap-2 mt-1">
        <!-- Category Badge -->
        <span class="inline-flex items-center px-2 py-0.5 rounded text-[10px] uppercase font-bold tracking-wider {categoryColorGroup}">
          {task.category}
        </span>
        <!-- Priority Badge (if Important) -->
        {#if isImportant}
          <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-[10px] uppercase font-bold tracking-wider bg-error/10 text-error">
            <span class="material-symbols-outlined text-[12px]">local_fire_department</span>
            {priorityLabel}
          </span>
        {/if}
      </div>
    </div>

    <!-- Quick Complete Button -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div 
      class="w-8 h-8 rounded-full border-2 flex items-center justify-center transition-colors {task.status === 'completed' ? 'bg-primary border-primary text-on-primary' : 'border-outline text-transparent group-hover:border-primary/50 hover:bg-primary/20'}"
      onclick={(e) => { e.stopPropagation(); onComplete(); }}
    >
      <span class="material-symbols-outlined text-[18px]">check</span>
    </div>
  </div>

  <!-- Description snippet -->
  {#if task.description && task.status !== 'completed'}
    <p class="text-[12px] text-on-surface-variant line-clamp-1 mb-3">
      {task.description}
    </p>
  {/if}

  <!-- Smart Metadata Info -->
  {#if task.task_type !== 'general' && task.status !== 'completed'}
    <div class="mt-2 border-t border-outline-variant/30 pt-3 flex items-center gap-4 text-xs">
      
      {#if task.task_type === 'shopping' && task.metadata?.items}
        <div class="flex items-center gap-1.5 text-primary">
          <span class="material-symbols-outlined text-[16px]">shopping_cart</span>
          <span class="font-medium">{task.metadata.items.length} items</span>
        </div>
      {/if}

      {#if task.task_type === 'billing' && task.metadata?.nominal}
        <div class="flex items-center gap-1.5 text-error">
          <span class="material-symbols-outlined text-[16px]">receipt_long</span>
          <span class="font-medium">{formatRupiah(task.metadata.nominal)}</span>
        </div>
      {/if}

      {#if task.task_type === 'debt' && task.metadata?.nominal}
        <div class="flex items-center gap-1.5 {task.metadata.debt_type === 'receivable' ? 'text-primary' : 'text-error'}">
          <span class="material-symbols-outlined text-[16px]">
            {task.metadata.debt_type === 'receivable' ? 'arrow_downward' : 'arrow_upward'}
          </span>
          <span class="font-medium">{formatRupiah(task.metadata.nominal)} ({task.metadata.contact_name})</span>
        </div>
      {/if}

      {#if task.due_date}
        <div class="flex items-center gap-1.5 text-on-surface-variant ml-auto">
          <span class="material-symbols-outlined text-[16px]">schedule</span>
          <span>{new Date(task.due_date).toLocaleDateString('id-ID', { day: 'numeric', month: 'short' })}</span>
        </div>
      {/if}
    </div>
  {/if}
</button>

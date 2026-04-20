<script lang="ts">
  import { slide, fade } from 'svelte/transition';
  import type { Task } from '$lib/services/tasks';

  let { isOpen = $bindable(false), task = null, onSave, onClose } = $props<{
    isOpen: boolean;
    task?: Partial<Task> | null;
    onSave: (data: Partial<Task>) => void;
    onClose: () => void;
  }>();

  // Form states
  let taskType = $state<'general'|'shopping'|'billing'|'debt'>('general');
  let title = $state('');
  let description = $state('');
  let category = $state('General');
  let priority = $state<'low'|'medium'|'high'|'urgent'>('medium');
  let dueDate = $state('');
  
  // Dynamic fields
  let billingNominal = $state('');
  let debtContact = $state('');
  let debtType = $state<'debt'|'receivable'>('debt');
  let debtNominal = $state('');

  $effect(() => {
    if (isOpen) {
      if (task) {
        taskType = task.task_type || 'general';
        title = task.title || '';
        description = task.description || '';
        category = task.category || 'General';
        priority = task.priority || 'medium';
        dueDate = task.due_date ? new Date(task.due_date).toISOString().slice(0,10) : '';

        // Load metadata if available
        if (taskType === 'billing' && task.metadata?.nominal) {
          billingNominal = task.metadata.nominal.toString();
        }
        if (taskType === 'debt' && task.metadata?.nominal) {
          debtNominal = task.metadata.nominal.toString();
          debtContact = task.metadata.contact_name || '';
          debtType = task.metadata.debt_type || 'debt';
        }
      } else {
        // Reset form
        taskType = 'general';
        title = '';
        description = '';
        category = 'General';
        priority = 'medium';
        dueDate = '';
        billingNominal = '';
        debtContact = '';
        debtNominal = '';
      }
    }
  });

  function handleSave() {
    if (!title.trim()) return;

    let metadata: any = {};
    if (taskType === 'billing') {
      metadata = { nominal: Number(billingNominal) || 0 };
    } else if (taskType === 'debt') {
      metadata = { nominal: Number(debtNominal) || 0, contact_name: debtContact, debt_type: debtType };
    } else if (taskType === 'shopping') {
      metadata = { items: [] }; // simplfied for demo
    }

    onSave({
      id: task?.id,
      title,
      description,
      task_type: taskType,
      category,
      priority,
      due_date: dueDate ? new Date(dueDate).toISOString() : null,
      metadata
    });
    isOpen = false;
  }
</script>

{#if isOpen}
  <!-- Backdrop -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 bg-background/80 backdrop-blur-sm z-[100]" transition:fade={{ duration: 200 }} onclick={onClose}></div>

  <!-- Drawer -->
  <div 
    class="fixed inset-x-0 bottom-0 bg-surface rounded-t-[32px] z-[101] shadow-2xl safe-area-bottom pb-20 pt-6 px-6 max-h-[90vh] overflow-y-auto"
    transition:slide={{ duration: 300, axis: 'y' }}
  >
    <!-- Handle -->
    <div class="w-12 h-1.5 bg-outline rounded-full mx-auto mb-6"></div>

    <h2 class="font-title text-2xl text-on-surface mb-6">
      {task ? 'Edit Tugas' : 'Tambah Tugas Baru'}
    </h2>

    <div class="flex flex-col gap-5">
      <!-- Task Type Chips -->
      <div class="flex gap-2 overflow-x-auto no-scrollbar pb-2">
        {#each [
          {id: 'general', label: 'Umum', icon: 'task'},
          {id: 'shopping', label: 'Belanja', icon: 'shopping_cart'},
          {id: 'billing', label: 'Tagihan', icon: 'receipt_long'},
          {id: 'debt', label: 'Hutang/Piutang', icon: 'sync_alt'}
        ] as type}
          <button 
            class="flex items-center gap-1 min-w-max px-4 py-2 rounded-full border text-sm transition-all {taskType === type.id ? 'bg-primary border-primary text-on-primary' : 'border-outline text-on-surface-variant hover:border-primary/50'}"
            onclick={() => taskType = type.id as any}
          >
            <span class="material-symbols-outlined text-[18px]">{type.icon}</span>
            {type.label}
          </button>
        {/each}
      </div>

      <!-- General Info -->
      <label class="flex flex-col gap-1.5">
        <span class="text-sm text-on-surface-variant ml-2">Judul Tugas</span>
        <input type="text" bind:value={title} placeholder="Apa yang ingin diselesaikan?" class="px-4 py-3 rounded-[16px] bg-surface-container-highest/50 border border-outline/20 focus:border-primary focus:bg-surface focus:outline-none transition-colors" />
      </label>

      <label class="flex flex-col gap-1.5">
        <span class="text-sm text-on-surface-variant ml-2">Catatan Detail</span>
        <textarea bind:value={description} placeholder="Tambahkan detail, catatan, atau rincian tambahan jika perlu..." rows="3" class="px-4 py-3 rounded-[16px] bg-surface-container-highest/50 border border-outline/20 focus:border-primary focus:bg-surface focus:outline-none transition-colors resize-none"></textarea>
      </label>

      <div class="grid grid-cols-2 gap-4">
        <label class="flex flex-col gap-1.5">
          <span class="text-sm text-on-surface-variant ml-2">Kategori</span>
          <input type="text" bind:value={category} placeholder="ex: Kuliah, Programming" class="px-4 py-3 rounded-[16px] bg-surface-container-highest/50 border border-outline/20 focus:border-primary focus:bg-surface focus:outline-none transition-colors" />
        </label>
        <label class="flex flex-col gap-1.5">
          <span class="text-sm text-on-surface-variant ml-2">Jatuh Tempo</span>
          <input type="date" bind:value={dueDate} class="px-4 py-3 rounded-[16px] bg-surface-container-highest/50 border border-outline/20 focus:border-primary focus:bg-surface focus:outline-none transition-colors" />
        </label>
      </div>

      <label class="flex flex-col gap-1.5">
        <span class="text-sm text-on-surface-variant ml-2">Prioritas</span>
        <select bind:value={priority} class="px-4 py-3 rounded-[16px] bg-surface-container-highest/50 border border-outline/20 focus:border-primary focus:outline-none transition-colors appearance-none">
          <option value="low">Rendah (Low)</option>
          <option value="medium">Sedang (Medium)</option>
          <option value="high">Tinggi (High)</option>
          <option value="urgent">Sangat Mendesak (Urgent)</option>
        </select>
      </label>

      <!-- Dynamic Sections -->
      {#if taskType === 'billing'}
        <div class="p-4 rounded-[20px] border border-outline/30 bg-primary/5 flex flex-col gap-4" transition:slide>
          <h4 class="font-medium text-sm text-primary flex items-center gap-2"><span class="material-symbols-outlined text-[18px]">receipt_long</span> Detail Tagihan</h4>
          <label class="flex flex-col gap-1.5">
            <span class="text-xs text-on-surface-variant">Nominal Tagihan (Rp)</span>
            <input type="number" bind:value={billingNominal} placeholder="150000" class="px-4 py-3 rounded-[12px] bg-surface border border-outline/20" />
          </label>
        </div>
      {/if}

      {#if taskType === 'debt'}
        <div class="p-4 rounded-[20px] border border-outline/30 bg-error/5 flex flex-col gap-4" transition:slide>
          <h4 class="font-medium text-sm text-error flex items-center gap-2"><span class="material-symbols-outlined text-[18px]">sync_alt</span> Detail Hutang/Piutang</h4>
          <div class="flex gap-2">
            <button class="flex-1 py-2 rounded-[12px] text-sm border transition-all {debtType === 'debt' ? 'bg-error text-onError border-error' : 'border-outline'}" onclick={() => debtType='debt'}>Hutang (Saya Berhutang)</button>
            <button class="flex-1 py-2 rounded-[12px] text-sm border transition-all {debtType === 'receivable' ? 'bg-primary text-onPrimary border-primary' : 'border-outline'}" onclick={() => debtType='receivable'}>Piutang (Orang Berhutang)</button>
          </div>
          <label class="flex flex-col gap-1.5">
            <span class="text-xs text-on-surface-variant">Nama Kontak</span>
            <input type="text" bind:value={debtContact} placeholder="Budi" class="px-4 py-3 rounded-[12px] bg-surface border border-outline/20" />
          </label>
          <label class="flex flex-col gap-1.5">
            <span class="text-xs text-on-surface-variant">Nominal (Rp)</span>
            <input type="number" bind:value={debtNominal} placeholder="50000" class="px-4 py-3 rounded-[12px] bg-surface border border-outline/20" />
          </label>
        </div>
      {/if}

      <button 
        onclick={handleSave}
        disabled={!title.trim()}
        class="w-full mt-4 bg-primary text-background font-bold py-4 rounded-full active:scale-95 transition-all disabled:opacity-50"
      >
        Simpan Tugas
      </button>
    </div>
  </div>
{/if}

import { writable } from 'svelte/store';

// Toast notifications
export interface Toast {
  id: string;
  message: string;
  type: 'success' | 'error' | 'info';
}

export const toasts = writable<Toast[]>([]);

export function addToast(message: string, type: Toast['type'] = 'info') {
  const id = crypto.randomUUID();
  toasts.update((t) => [...t, { id, message, type }]);
  setTimeout(() => {
    toasts.update((t) => t.filter((toast) => toast.id !== id));
  }, 4000);
}

// Modal state
export const modalOpen = writable(false);
export const modalComponent = writable<string | null>(null);
export const modalProps = writable<Record<string, unknown>>({});

export function openModal(component: string, props: Record<string, unknown> = {}) {
  modalComponent.set(component);
  modalProps.set(props);
  modalOpen.set(true);
}

export function closeModal() {
  modalOpen.set(false);
  modalComponent.set(null);
  modalProps.set({});
}

import { api } from '$lib/api/client';

export interface Task {
    id: string;
    user_id: string;
    task_type: 'general' | 'shopping' | 'billing' | 'debt';
    title: string;
    description: string | null;
    category: string;
    priority: 'low' | 'medium' | 'high' | 'urgent';
    due_date: string | null;
    status: 'pending' | 'in_progress' | 'completed';
    metadata: any;
    created_at: string;
    updated_at: string;
}

export const fetchTasks = async (): Promise<Task[]> => {
    return await api<Task[]>('/tasks');
};

export const fetchTaskById = async (id: string): Promise<Task> => {
    return await api<Task>(`/tasks/${id}`);
};

export const createTask = async (taskData: Partial<Task>): Promise<Task> => {
    return await api<Task>('/tasks', { method: 'POST', body: taskData });
};

export const updateTask = async (id: string, taskData: Partial<Task>): Promise<Task> => {
    return await api<Task>(`/tasks/${id}`, { method: 'PUT', body: taskData });
};

export const deleteTask = async (id: string): Promise<void> => {
    await api(`/tasks/${id}`, { method: 'DELETE' });
};

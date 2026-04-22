// API Client — centralized fetch wrapper with JWT handling
const API_BASE = import.meta.env.VITE_API_URL || '/api';

interface ApiOptions {
  method?: string;
  body?: unknown;
  headers?: Record<string, string>;
}

class ApiError extends Error {
  status: number;
  data: unknown;
  constructor(status: number, message: string, data?: unknown) {
    super(message);
    this.status = status;
    this.data = data;
  }
}

function getToken(): string | null {
  if (typeof window === 'undefined') return null;
  return localStorage.getItem('pennywise_token');
}

export function setToken(token: string) {
  localStorage.setItem('pennywise_token', token);
}

export function clearToken() {
  localStorage.removeItem('pennywise_token');
}

export async function api<T = unknown>(endpoint: string, options: ApiOptions = {}): Promise<T> {
  const { method = 'GET', body, headers = {} } = options;
  const token = getToken();

  const requestHeaders: Record<string, string> = {
    'Content-Type': 'application/json',
    ...headers,
  };

  if (token) {
    requestHeaders['Authorization'] = `Bearer ${token}`;
  }

  const config: RequestInit = {
    method,
    headers: requestHeaders,
  };

  if (body && method !== 'GET') {
    config.body = JSON.stringify(body);
  }

  const response = await fetch(`${API_BASE}${endpoint}`, config);

  if (!response.ok) {
    const errorData = await response.json().catch(() => ({}));
    throw new ApiError(
      response.status,
      errorData.message || `API Error: ${response.status}`,
      errorData
    );
  }

  if (response.status === 204) return undefined as T;
  return response.json();
}

// Auth API
export const authApi = {
  register: (data: { email: string; name: string; password: string }) =>
    api<{ token: string; user: User }>('/auth/register', { method: 'POST', body: data }),
  login: (data: { email: string; password: string }) =>
    api<{ token: string; user: User }>('/auth/login', { method: 'POST', body: data }),
  profile: () => api<User>('/auth/profile'),
  updateProfile: (data: { name: string }) => 
    api<User>('/auth/profile', { method: 'PUT', body: data }),
  updatePassword: (data: { old_password: string; new_password: string }) =>
    api<{ message: string }>('/auth/password', { method: 'PUT', body: data }),
};

// Transactions API
export const transactionsApi = {
  list: (params?: { search?: string; type?: string; month?: number; year?: number }) => {
    const searchParams = new URLSearchParams();
    if (params?.search) searchParams.set('search', params.search);
    if (params?.type) searchParams.set('type', params.type);
    if (params?.month) searchParams.set('month', String(params.month));
    if (params?.year) searchParams.set('year', String(params.year));
    const qs = searchParams.toString();
    return api<Transaction[]>(`/transactions${qs ? '?' + qs : ''}`);
  },
  create: (data: TransactionInput) =>
    api<Transaction>('/transactions', { method: 'POST', body: data }),
  update: (id: string, data: TransactionInput) =>
    api<Transaction>(`/transactions/${id}`, { method: 'PUT', body: data }),
  delete: (id: string) =>
    api(`/transactions/${id}`, { method: 'DELETE' }),
};

// Budgets API
export const budgetsApi = {
  list: (params?: { month?: number; year?: number }) => {
    const searchParams = new URLSearchParams();
    if (params?.month) searchParams.set('month', String(params.month));
    if (params?.year) searchParams.set('year', String(params.year));
    const qs = searchParams.toString();
    return api<Budget[]>(`/budgets${qs ? '?' + qs : ''}`);
  },
  create: (data: BudgetInput) =>
    api<Budget>('/budgets', { method: 'POST', body: data }),
  delete: (id: string) =>
    api(`/budgets/${id}`, { method: 'DELETE' }),
};

// Dashboard API
export const dashboardApi = {
  summary: () => api<DashboardSummary>('/dashboard/summary'),
};

// Types
export interface User {
  id: string;
  email: string;
  name: string;
  avatar_url?: string;
  created_at: string;
}

export interface Transaction {
  id: string;
  user_id: string;
  amount: string;
  category: string;
  description: string;
  transaction_type: 'income' | 'expense';
  icon: string;
  date: string;
  created_at: string;
}

export interface TransactionInput {
  amount: number;
  category: string;
  description: string;
  transaction_type: 'income' | 'expense';
  icon?: string;
  date?: string;
}

export interface Budget {
  id: string;
  user_id: string;
  category: string;
  limit_amount: string;
  spent_amount: string;
  icon: string;
  month: number;
  year: number;
}

export interface BudgetInput {
  category: string;
  limit_amount: number;
  icon?: string;
  month: number;
  year: number;
}

export interface DashboardSummary {
  total_balance: string;
  total_income: string;
  total_expenses: string;
  balance_change_percent: number;
  weekly_activity: number[];
  recent_transactions: Transaction[];
}

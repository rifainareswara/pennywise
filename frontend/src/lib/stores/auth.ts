import { writable, derived } from 'svelte/store';
import { authApi, setToken, clearToken, type User } from '$lib/api/client';

export const user = writable<User | null>(null);
export const token = writable<string | null>(null);
export const authLoading = writable(true);
export const isAuthenticated = derived(user, ($user) => $user !== null);

export async function login(email: string, password: string) {
  const res = await authApi.login({ email, password });
  setToken(res.token);
  token.set(res.token);
  user.set(res.user);
  return res;
}

export async function register(email: string, name: string, password: string) {
  const res = await authApi.register({ email, name, password });
  setToken(res.token);
  token.set(res.token);
  user.set(res.user);
  return res;
}

export function logout() {
  clearToken();
  token.set(null);
  user.set(null);
}

export async function loadUser() {
  authLoading.set(true);
  try {
    const storedToken = typeof window !== 'undefined' ? localStorage.getItem('pennywise_token') : null;
    if (!storedToken) {
      authLoading.set(false);
      return;
    }
    token.set(storedToken);
    const profile = await authApi.profile();
    user.set(profile);
  } catch {
    clearToken();
    token.set(null);
    user.set(null);
  } finally {
    authLoading.set(false);
  }
}

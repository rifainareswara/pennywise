import { writable } from 'svelte/store';
import { walletsApi, type Wallet, type WalletInput } from '$lib/api/client';

export const wallets = writable<Wallet[]>([]);
export const walletsLoading = writable(false);

export async function loadWallets() {
  walletsLoading.set(true);
  try {
    const data = await walletsApi.list();
    wallets.set(data);
  } catch {
    wallets.set([]);
  } finally {
    walletsLoading.set(false);
  }
}

export async function createWallet(data: WalletInput) {
  const result = await walletsApi.create(data);
  wallets.update(list => [result, ...list]);
  return result;
}

export async function updateWallet(id: string, data: WalletInput) {
  const result = await walletsApi.update(id, data);
  wallets.update(list => list.map(w => w.id === id ? result : w));
  return result;
}

export async function adjustWalletBalance(id: string, balance: number) {
  const result = await walletsApi.adjust(id, balance);
  wallets.update(list => list.map(w => w.id === id ? result : w));
  return result;
}

export async function deleteWallet(id: string) {
  await walletsApi.delete(id);
  wallets.update(list => list.filter(w => w.id !== id));
}

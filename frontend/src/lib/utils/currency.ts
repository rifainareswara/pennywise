/**
 * Format a number as Indonesian Rupiah (IDR) currency.
 * Uses 'id-ID' locale for proper thousand separators (dots).
 */
export function formatRupiah(value: number | string, showSign = false): string {
  const n = typeof value === 'string' ? parseFloat(value) : value;
  if (isNaN(n)) return 'Rp0';

  const abs = Math.abs(n);
  const formatted = abs.toLocaleString('id-ID', {
    minimumFractionDigits: 0,
    maximumFractionDigits: 0,
  });

  if (showSign) {
    return n >= 0 ? `+Rp${formatted}` : `-Rp${formatted}`;
  }

  return `Rp${formatted}`;
}

/**
 * Format a compact Rupiah string (e.g., Rp3,2Jt for millions).
 */
export function formatRupiahCompact(value: number | string): string {
  const n = typeof value === 'string' ? parseFloat(value) : value;
  if (isNaN(n)) return 'Rp0';

  if (n >= 1_000_000_000) {
    return `Rp${(n / 1_000_000_000).toFixed(1).replace('.', ',')}M`;
  }
  if (n >= 1_000_000) {
    return `Rp${(n / 1_000_000).toFixed(1).replace('.', ',')}Jt`;
  }
  if (n >= 1_000) {
    return `Rp${(n / 1_000).toFixed(1).replace('.', ',')}Rb`;
  }
  return `Rp${n.toLocaleString('id-ID')}`;
}

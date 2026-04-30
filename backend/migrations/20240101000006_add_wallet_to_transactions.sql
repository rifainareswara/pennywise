ALTER TABLE transactions ADD COLUMN wallet_id UUID REFERENCES wallets(id) ON DELETE SET NULL;
CREATE INDEX idx_transactions_wallet ON transactions(wallet_id);

CREATE TABLE wallets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    wallet_type VARCHAR(50) NOT NULL DEFAULT 'bank',
    balance DECIMAL(15,2) NOT NULL DEFAULT 0,
    icon VARCHAR(100) DEFAULT 'account_balance',
    color VARCHAR(50) DEFAULT 'primary',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_wallets_user ON wallets(user_id);

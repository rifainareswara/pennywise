# 🪙 PennyWise — Personal Finance & Budgeting App

A fullstack personal finance application with an editorial-grade dark UI, built with **SvelteKit + Tailwind CSS** (frontend) and **Rust Axum + PostgreSQL** (backend). Fully localized for the Indonesian market with IDR (Rp) currency formatting and translated categories.

## 🏗 Architecture

```
pennywise/
├── frontend/          # SvelteKit + Tailwind CSS v4
│   ├── src/
│   │   ├── lib/
│   │   │   ├── api/          # API client (fetch wrapper + types)
│   │   │   ├── components/   # Reusable Svelte components
│   │   │   └── stores/       # Svelte stores (auth, transactions, budgets, ui)
│   │   └── routes/
│   │       ├── (app)/        # Authenticated pages (dashboard, transactions, budget, profile)
│   │       ├── login/
│   │       └── register/
│   └── ...
├── backend/           # Rust Axum
│   ├── src/
│   │   ├── handlers/     # Route handlers
│   │   ├── services/     # Business logic (auth/JWT/argon2)
│   │   ├── repositories/ # SQLx database queries
│   │   ├── models/       # Data structures & DTOs
│   │   └── middleware/   # JWT auth middleware
│   └── migrations/       # PostgreSQL migrations
├── docker-compose.yml
└── .env.example
```

## 🚀 Quick Start

### Prerequisites
- **Node.js** 20+
- **Rust** 1.77+ (with `cargo`)
- **PostgreSQL** 16 (or use Docker)

### 1. Clone & configure
```bash
git clone https://github.com/your-username/pennywise.git
cd pennywise
cp .env.example .env
```

### 2. Start PostgreSQL
```bash
# Option A: Docker
docker compose up db -d

# Option B: Local PostgreSQL
createdb pennywise
```

### 3. Run Backend
```bash
cd backend
cargo run
# Server starts on http://localhost:3000
```

### 4. Run Frontend
```bash
cd frontend
npm install
npm run dev
# App starts on http://localhost:5173
```

### 5. Docker (Full Stack)
```bash
docker compose up --build -d
```

## 📱 Pages

| Page | Route | Description |
|------|-------|-------------|
| Landing | `/` | Onboarding with Get Started / Log In |
| Login | `/login` | Email & password sign in |
| Register | `/register` | Create account |
| Dashboard | `/dashboard` | Balance, income/expense, weekly chart, recent transactions |
| Transactions | `/transactions` | Search, filter, grouped list |
| Add Transaction | `/transactions/new` | Form with category grid |
| Edit Transaction | `/transactions/:id/edit` | Edit + delete |
| Budget | `/budget` | Circular progress, category breakdown (add, edit, delete) |
| Profile | `/profile` | User info, settings, logout |

## 🔌 API Endpoints

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/api/auth/register` | No | Register |
| POST | `/api/auth/login` | No | Login → JWT |
| GET | `/api/auth/profile` | Yes | Current user |
| GET | `/api/transactions` | Yes | List (search, filter, pagination) |
| POST | `/api/transactions` | Yes | Create |
| GET | `/api/transactions/:id` | Yes | Get one |
| PUT | `/api/transactions/:id` | Yes | Update |
| DELETE | `/api/transactions/:id` | Yes | Delete |
| GET | `/api/budgets` | Yes | List by month/year |
| POST | `/api/budgets` | Yes | Create/upsert |
| DELETE | `/api/budgets/:id` | Yes | Delete budget |
| GET | `/api/dashboard/summary` | Yes | Aggregated dashboard data |

## 🎨 Design System

Based on Material Design 3 dark theme with an editorial aesthetic:
- **Primary**: `#54E98A` (growth green)
- **Secondary**: `#FFB4A9` (soft coral for alerts)
- **Fonts**: Manrope (headlines) + Inter (body)
- **No borders** — surfaces are defined by tonal layering
- **Glass & gradient** CTAs with pill-shaped buttons

## 🔐 Security

- **Argon2** password hashing
- **JWT** Bearer token authentication (24h expiry)
- **CORS** configured for frontend origin
- **Input validation** on all endpoints

## 🛠 Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | SvelteKit 2, Svelte 5, Tailwind CSS 4 |
| Backend | Rust, Axum 0.8, Tokio |
| Database | PostgreSQL 16, SQLx |
| Auth | Argon2, JWT |
| DevOps | Docker, Docker Compose |

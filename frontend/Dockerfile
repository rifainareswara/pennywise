# Build stage
FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

# Runtime stage
FROM node:20-alpine
WORKDIR /app

# Ambil hasil build (adapter-node menghasilkan folder 'build')
COPY --from=builder /app/build ./build
COPY --from=builder /app/package*.json ./

# Install hanya dependensi produksi
RUN npm ci --omit=dev

# SvelteKit butuh ENV PORT agar tahu jalan di port mana
ENV PORT=5173
EXPOSE 5173

# Jalankan server SvelteKit
CMD ["node", "build/index.js"]
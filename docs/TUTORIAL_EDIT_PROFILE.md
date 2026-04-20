# Tutorial Belajar Coding: Fitur Edit Profil (PennyWise)

Dokumen ini adalah materi pembelajaran yang mencatat langkah demi langkah cara membangun fitur **Edit Profil** pada aplikasi Fullstack SvelteKit + Rust. Fitur yang sudah berjalan dengan baik dapat dipelajari di sini.

## Konsep Dasar Fullstack
Aplikasi kita dibagi dua, yaitu **Frontend (SvelteKit)** yang menangani tampilan visual di layar pengguna, dan **Backend (Rust Axum)** yang mengatur penyimpanan data ke tabel *database* (PostgreSQL).

Ketika Anda mengirimkan nama profil baru, SvelteKit mengirim instruksi JSON lewat internet yang kemudian ditangkap dan dimasukkan ke database oleh Rust Axum.

---

## Tahap 1: Backend (Sisi Server & Database)
Sudah dikerjakan! Berikut adalah 4 komponen vital yang kita modifikasi untuk menopang fitur Edit Profil:

### 1. Model (Struktur Data) - `backend/src/models/user.rs`
Server butuh cetakan bentuk data untuk tahu apa yang akan diterimanya. Kita membuat *Struct* bernama `UpdateProfileInput`:
```rust
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileInput {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}
```
*Penjelasan:* Ini berarti setiap permintaan Edit Profil *wajib* membawa `name` berformat tulisan (String) dan panjangnya minimal 1 huruf.

### 2. Repository (Query Database) - `backend/src/repositories/user.rs`
Server tidak bisa mengedit data dengan sihir, dia butuh instruksi SQL yang jelas untuk ngobrol ke PostgreSQL:
```rust
pub async fn update_name(pool: &PgPool, id: Uuid, name: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "UPDATE users SET name = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
    )
    .bind(name)
    .bind(id)
    .fetch_one(pool)
    .await
}
```
*Penjelasan:* Kita mengeksekusi fungsi SQL `UPDATE` pada tabel `users`. `$1` akan diisi nama baru, `$2` dijumpai UUID Anda. Ini jauh dari jangkauan *hacker* (SQL Injection).

### 3. Handler (Logika Endpoint API) - `backend/src/handlers/auth.rs`
Ini adalah satpam dan pemroses logika utama pendaftaran rute.
```rust
pub async fn update_profile(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>, // Memastikan harus Login
    Json(input): Json<UpdateProfileInput>, // Menerima JSON nama
) -> Result<Json<UserResponse>, AppError> {
    
    // 1. Validasi teks input (tidak boleh kosong)
    input.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

    // 2. Eksekusi fungsi update database dari langkah 2
    let updated_user = user_repo::update_name(&pool, auth.user_id, &input.name)
        .await
        .map_err(|_| AppError::Internal("Database error".into()))?;

    // 3. Kembalikan data sukses ke pengguna (Front-end)
    Ok(Json(UserResponse::from(updated_user)))
}
```

### 4. Router (Registrasi URL) - `backend/src/main.rs`
Kita harus mendaftarkan URL agar aplikasi depan tahu kemana paket data harus dilempar:
```rust
let protected_routes = Router::new()
    .route("/api/auth/profile", get(handlers::auth::profile))
    .route("/api/auth/profile", put(handlers::auth::update_profile)) // BARU DITAMBAHKAN
//...
```
*Penjelasan:* Rute profil lama menggunakan `GET` (ambil data), sementara Edit Profil mendaftar dengan *method* `PUT` (simpan ulang / ubah data). Keduanya ada di dalam kelompok rahasia (`protected_routes`) sehingga butuh JWT Token!

---

## Tahap 2: Frontend (Membangun Antarmuka Svelte)

Pada tahap ini kita merajut sistem API agar tombol-tombol di layar HP benar-benar hidup.

### 1. API Client - `frontend/src/lib/api/client.ts`
Kita butuh jembatan penghubung yang bisa mengirimkan `JSON` yang dibuat oleh Backend di Tahap 1.
```typescript
  updateProfile: (data: { name: string }) => 
    api<User>('/auth/profile', { method: 'PUT', body: data }),
```

### 2. Svelte View - `frontend/src/routes/(app)/profile/edit/+page.svelte`
Buatlah formulir dengan `<input bind:value={name}>`. Svelte 5 menggunakan sistem `$state` yang sangat reaktif (cepat merespon ketikan Anda).
```svelte
<script lang="ts">
  import { user } from '$lib/stores/auth';
  import { authApi } from '$lib/api/client';
  
  let name = $state($user?.name || '');

  async function handleSave(e: Event) {
    e.preventDefault();
    const updatedUser = await authApi.updateProfile({ name });
    
    // Perbarui profil di atas layar secara magis
    user.set(updatedUser);
  }
</script>

<form onsubmit={handleSave}>
   <input bind:value={name} type="text" required />
   <button type="submit">Simpan</button>
</form>
```
*Penjelasan:* Begitu fungsi `authApi.updateProfile` memanggil `PUT`, ia mencetak profil *terupdate*. Kita memanggil `user.set()` milik penyimpanan (Store) Svelte, dan _Voila_, semua huruf nama Anda di dalam aplikasi PennyWise otomatis terganti secara instan (Reaktivitas Data)!

---

## Tahap 3: Fitur Ganti Sandi (Tingkat Lanjut Keamanan/Kriptografi)

Banyak yang bertanya, "Lalu bagaimana jika saya ingin melakukan pergantian *Password*?" Prosesnya cukup identik, namun ada elemen validasi Argon2 (Library Kriptografi untuk Password).

1. **(Backend Model)** Server **wajib** meminta 2 data: `old_password` dan `new_password`.
2. **(Backend Handler)** Anda TIDAK AKAN PERNAH menggunakan instruksi update secara normal! Server diwajibkan memeriksa *hash* `old_password` dari database melawan hasil kriptografi *password* yang baru dicetak dari keyboard sang pengguna via fungsi `verify_password()`.
3. **(Penyimpanan Rahasia)** Jika cocok, hash lama dibuang dan hash baru di simpan melalui Argon2 yang dienkripsi berlapis (Salting + Hashing).
```rust
    // Verify old password (Argon2)
    let is_valid = verify_password(&input.old_password, &user.password_hash)?;
    if !is_valid {
        return Err(AppError::BadRequest("Password lama tidak sesuai".into()));
    }
```
4. **(Frontend UI)** Formulir ini dapat ditemukan di `frontend/src/routes/(app)/profile/security/+page.svelte` yang memantau `<input type="password">` dan mencocokannya dengan isian 'Konfirmasi Sandi'. Tidak ada pergantian *state* global Svelte (`user.set`) karena kata sandi bukan data yang diamankan / dikirimkan oleh API ke client Svelte untuk mencegah kebocoran *Plain-text*.

Begitulah panduan Arsitektur Autentikasi dan Profil yang kokoh ini. Selamat mencoba modifikasinya!

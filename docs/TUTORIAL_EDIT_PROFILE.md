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
Tahap ini belum kita kerjakan (menunggu arahan selanjutnya).

Secara garis besar, nanti kita akan membuat hal berikut:
1. `EditProfile.svelte`: Halaman visual dengan kolom input teks bertuliskan nama.
2. `authApi.updateProfile()`: Menyolder kabel khusus yang berisi fungsi Fetch API ke titik rute `PUT /api/auth/profile`.
3. Mengganti isi penyimpanan frontend (`auth Store`) agar nama di seluruh aplikasi PennyWise seketika berubah mengikuti nama yang baru diedit!

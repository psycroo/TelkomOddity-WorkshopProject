# 📡 TelkomOddity: The On-Chain Chronicle
> **"Social media posts can be deleted. Admins can be pressured. But the Blockchain? The Blockchain remembers every glitch in the Telkom matrix."**

[![Smart Contract](https://img.shields.io/badge/Contract-Soroban-ff3cac?style=for-the-badge&logo=stellar)](https://stellar.org/soroban)
[![Built With](https://img.shields.io/badge/Built_With-Rust-00f5d4?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Target](https://img.shields.io/badge/Target-Telkom_University-e02b20?style=for-the-badge)](https://telkomuniversity.ac.id/)

---

## 📜 01 — Background: Kenapa Ini Ada?

Kampus kita, **Telkom University**, bukan sekadar tempat kuliah. Ini adalah ekosistem yang penuh dengan kejadian "ajaib"—mulai dari kucing yang ikut ujian di GKU, fenomena parkiran yang penuh secara misterius, hingga momen-momen absurd di depan gerbang asrama.

Seringkali, momen ini hanya lewat di *menfess* atau *story* Instagram yang hilang dalam 24 jam. Kami percaya bahwa **keanehan adalah identitas**. 

**TelkomOddity** hadir untuk memastikan setiap tawa, kebingungan, dan keunikan di kampus tercatat selamanya. Menggunakan teknologi **Stellar Blockchain**, laporanmu tidak disimpan di database kampus yang bisa mati—ia hidup di ribuan node di seluruh dunia. 

**No Admins. No Censorship. Just Pure Campus History.**

---

## ✨ 02 — Fitur Utama

| Fitur | Vibe | Penjelasan |
| :--- | :--- | :--- |
| **🚀 Submit Oddity** | *Immutable* | Kirim laporan kejadian (NIM, Nama, Lokasi, Deskripsi). |
| **👁️ The Eye** | *Transparent* | Baca semua rekaman kejadian aneh dari masa ke masa tanpa filter. |
| **📈 Oddity Counter** | *Stats* | Pantau seberapa "aneh" kampus kita hari ini melalui statistik on-chain. |
| **🛡️ Ownership** | *Control* | Hanya pemilik laporan yang bisa menghapus (burn) data mereka sendiri. |

---

## 🛠️ 03 — Tech Stack

Projek ini dibangun dengan efisiensi tingkat tinggi dan keamanan maksimal:

* **Language:** [Rust](https://www.rust-lang.org/) (Type-safe & Blazing Fast).
* **Smart Contract Framework:** [Soroban](https://soroban.stellar.org/) (Next-gen smart contracts on Stellar).
* **Storage:** Soroban Instance Storage (Data persisten di ledger).
* **Network:** Stellar Futurenet.

---

## 💻 04 — API & Struktur Data

Jika kamu seorang developer yang ingin berinteraksi dengan contract ini, berikut adalah "jeroannya":

<details>
<summary>📂 Lihat Detail Struktur Data (Laporan)</summary>

Setiap laporan disimpan dalam struct `Laporan`:
- `id`: Unique ID (Auto-generated).
- `nim`: NIM mahasiswa pelapor.
- `nama`: Nama pelapor.
- `lokasi`: TKP (GKU, OpenLib, Student Center, dll).
- `kejadian`: Deskripsi peristiwa.
- `kategori`: `ANEH`, `UNIK`, atau `LUCU`.

</details>

<details>
<summary>🔌 Lihat Fungsi Smart Contract</summary>

- `buat_laporan`: Menulis data baru ke blockchain (Membutuhkan gas fee XLM).
- `get_all_laporan`: Mengambil seluruh list laporan (Gratis/Read-only).
- `hitung_laporan`: Mengembalikan total laporan yang tersimpan.
- `hapus_laporan(id)`: Menghapus laporan berdasarkan ID unik.

</details>

---

## 🚀 05 — Cara Menjalankan

### Persiapan
1. Install **Rust** & **Soroban CLI**.
2. Konfigurasi identitas Stellar kamu.

### Deployment
```bash
# Build project ke WASM
soroban contract build

# Deploy ke jaringan (contoh: Futurenet)
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/telkom_oddity.wasm --source my_account --network futurenet

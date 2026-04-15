#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Laporan {
    pub id       : u64,
    pub nim      : String,
    pub nama     : String,
    pub lokasi   : String,
    pub kejadian : String,
    pub kategori : String, // "ANEH" | "UNIK" | "LUCU"
}

const LAPORAN_DATA: Symbol = symbol_short!("LAP_DATA");

#[contract]
pub struct TelkomReportContract;

#[contractimpl]
impl TelkomReportContract {

    // Ambil semua laporan
    pub fn get_all_laporan(env: Env) -> Vec<Laporan> {
        env.storage().instance().get(&LAPORAN_DATA).unwrap_or(Vec::new(&env))
    }

    // Buat laporan baru
    pub fn buat_laporan(
        env      : Env,
        nim      : String,
        nama     : String,
        lokasi   : String,
        kejadian : String,
        kategori : String,
    ) -> String {
        let mut list: Vec<Laporan> = env.storage().instance().get(&LAPORAN_DATA).unwrap_or(Vec::new(&env));

        list.push_back(Laporan {
            id: env.prng().gen::<u64>(),
            nim,
            nama,
            lokasi,
            kejadian,
            kategori,
        });

        env.storage().instance().set(&LAPORAN_DATA, &list);
        String::from_str(&env, "Laporan berhasil disimpan!")
    }

    // Hapus laporan berdasarkan ID
    pub fn hapus_laporan(env: Env, id: u64) -> String {
        let mut list: Vec<Laporan> = env.storage().instance().get(&LAPORAN_DATA).unwrap_or(Vec::new(&env));

        for i in 0..list.len() {
            if list.get(i).unwrap().id == id {
                list.remove(i);
                env.storage().instance().set(&LAPORAN_DATA, &list);
                return String::from_str(&env, "Laporan berhasil dihapus!");
            }
        }

        String::from_str(&env, "ID tidak ditemukan.")
    }

    // Hitung total laporan
    pub fn hitung_laporan(env: Env) -> u32 {
        let list: Vec<Laporan> = env.storage().instance().get(&LAPORAN_DATA).unwrap_or(Vec::new(&env));
        list.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_contract() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TelkomReportContract);
        let client = TelkomReportContractClient::new(&env, &contract_id);

        // Awalnya kosong
        assert_eq!(client.hitung_laporan(), 0);

        // Tambah laporan
        client.buat_laporan(
            &String::from_str(&env, "NIM_KAMU"),
            &String::from_str(&env, "NAMA_KAMU"),
            &String::from_str(&env, "LOKASI"),
            &String::from_str(&env, "DESKRIPSI_KEJADIAN"),
            &String::from_str(&env, "LUCU"),
        );

        assert_eq!(client.hitung_laporan(), 1);

        // Hapus laporan
        let id = client.get_all_laporan().get(0).unwrap().id;
        client.hapus_laporan(&id);
        assert_eq!(client.hitung_laporan(), 0);
    }
}
mod test;


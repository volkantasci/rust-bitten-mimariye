use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Ajanın getirdiği "çalışan" kod: derlenir, testi geçer, makul bir çıktı verir
pub fn rapor(kayitlar: Vec<String>, onbellek: Arc<Mutex<HashMap<String, usize>>>) -> String {
    let mut toplam: u64 = 0;
    for k in kayitlar.clone() {
        let uzunluk = k.len() as u8; // 255'ten uzun kayıtlar sessizce kırpılır
        toplam += uzunluk as u64;
        onbellek.lock().unwrap().insert(k.clone(), uzunluk as usize);
    }
    let adlar: Vec<String> = kayitlar.iter().map(|k| k.to_uppercase()).collect();
    let ilk = adlar.iter().next().unwrap();
    format!("{} kayıt, toplam {} bayt, ilk: {}", adlar.len(), toplam, ilk)
}

fn main() {
    let o = Arc::new(Mutex::new(HashMap::new()));
    println!("{}", rapor(vec!["elma".into(), "x".repeat(300)], o));
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn kisa_kayitlar() {
        let o = Arc::new(Mutex::new(HashMap::new()));
        assert_eq!(rapor(vec!["elma".into(), "armut".into()], o), "2 kayıt, toplam 9 bayt, ilk: ELMA");
    }
}

mod banka {
    pub struct Hesap {
        pub sahip: String,
        bakiye: u64, // gizli: değişmez kural "bakiye eksiye düşmez" burada korunur
    }

    impl Hesap {
        pub fn ac(sahip: &str) -> Self {
            Hesap { sahip: sahip.to_string(), bakiye: 0 }
        }
        pub fn yatir(&mut self, tutar: u64) {
            self.bakiye += tutar;
        }
        pub fn cek(&mut self, tutar: u64) -> Result<(), String> {
            if tutar > self.bakiye {
                return Err(format!("yetersiz bakiye: {} < {tutar}", self.bakiye));
            }
            self.bakiye -= tutar;
            Ok(())
        }
        pub fn bakiye(&self) -> u64 {
            self.bakiye
        }
    }
}

fn main() {
    let mut h = banka::Hesap::ac("Ayşe");
    h.yatir(50);
    let sonuc = h.cek(80);
    println!("{} çekmek istiyor: {:?}", h.sahip, sonuc);
    h.bakiye = 1_000_000; // kuralı dışarıdan delmeye çalış
    println!("bakiye: {}", h.bakiye());
}

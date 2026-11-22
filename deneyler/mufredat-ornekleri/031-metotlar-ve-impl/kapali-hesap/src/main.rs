struct Hesap {
    sahip: String,
    bakiye: i64,
}

impl Hesap {
    fn yeni(sahip: &str) -> Self {
        Hesap { sahip: sahip.to_string(), bakiye: 0 }
    }
    fn bakiye(&self) -> i64 { // &self: yalnızca okur
        self.bakiye
    }
    fn yatir(&mut self, tutar: i64) { // &mut self: değiştirir
        self.bakiye += tutar;
    }
    fn kapat(self) -> i64 { // self: hesabı tüketir
        println!("{} hesabı kapandı", self.sahip);
        self.bakiye
    }
}

fn main() {
    let mut h = Hesap::yeni("Ayşe");
    h.yatir(100);
    println!("bakiye: {}", h.bakiye());
    let son = h.kapat();
    println!("son bakiye: {son}");
    h.yatir(5); // kapanmış hesaba para yatırmak
}

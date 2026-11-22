use std::marker::PhantomData;

// Durumlar birer tip: değerleri yok, yalnızca derleyiciye bilgi taşırlar
struct Taslak;
struct Onayli;
struct Gonderildi;

struct Siparis<Durum> {
    kalemler: Vec<(String, u32)>,
    _durum: PhantomData<Durum>,
}

impl Siparis<Taslak> {
    fn yeni() -> Self {
        Siparis { kalemler: Vec::new(), _durum: PhantomData }
    }
    fn ekle(mut self, urun: &str, adet: u32) -> Self {
        self.kalemler.push((urun.to_string(), adet));
        self
    }
    fn onayla(self) -> Result<Siparis<Onayli>, String> {
        if self.kalemler.is_empty() {
            return Err("boş sipariş onaylanamaz".into());
        }
        Ok(Siparis { kalemler: self.kalemler, _durum: PhantomData })
    }
}

impl Siparis<Onayli> {
    fn gonder(self) -> Siparis<Gonderildi> {
        Siparis { kalemler: self.kalemler, _durum: PhantomData }
    }
}

impl Siparis<Gonderildi> {
    fn ozet(&self) -> String {
        format!("{} kalem gönderildi", self.kalemler.len())
    }
}

// "Ayrıştır, doğrulama": geçerli e-posta yalnızca bu tiple temsil edilir
struct Eposta(String);
impl Eposta {
    fn ayristir(s: &str) -> Result<Eposta, String> {
        match s.split_once('@') {
            Some((ad, alan)) if !ad.is_empty() && alan.contains('.') => Ok(Eposta(s.to_string())),
            _ => Err(format!("geçersiz e-posta: {s:?}")),
        }
    }
}

fn bildirim_gonder(kime: &Eposta, mesaj: &str) {
    println!("{} adresine: {mesaj}", kime.0); // burada tekrar doğrulamaya gerek yok
}

fn main() {
    let s = Siparis::yeni().ekle("defter", 2).ekle("kalem", 5).onayla().unwrap().gonder();
    println!("{}", s.ozet());
    println!("{:?}", Siparis::yeni().onayla().err());
    for adres in ["ayse@ornek.com", "ayse.ornek.com"] {
        match Eposta::ayristir(adres) {
            Ok(e) => bildirim_gonder(&e, &s.ozet()),
            Err(h) => println!("{h}"),
        }
    }
}

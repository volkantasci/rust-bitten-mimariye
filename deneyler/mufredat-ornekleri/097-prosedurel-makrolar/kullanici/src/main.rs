use tanit_turet::Tanit;

#[derive(Tanit)]
#[allow(dead_code)]
struct Siparis {
    numara: u64,
    musteri: String,
    tutar_kurus: i64,
}

fn main() {
    println!("{} alanları: {:?}", Siparis::tip_adi(), Siparis::alan_adlari());
}

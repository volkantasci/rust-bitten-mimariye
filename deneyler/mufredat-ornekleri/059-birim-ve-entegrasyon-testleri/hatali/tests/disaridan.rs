// Entegrasyon testi: crate'i bir kullanıcı gibi, yalnızca açık (pub) arayüzünden kullanır
#[test]
fn ortalama_ve_medyan_birlikte() {
    let mut notlar = [70.0, 90.0, 80.0];
    assert_eq!(istatistik::ortalama(&notlar), Some(80.0));
    assert_eq!(istatistik::medyan(&mut notlar), Some(80.0));
}

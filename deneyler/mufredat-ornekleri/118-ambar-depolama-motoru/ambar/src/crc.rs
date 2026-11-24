//! CRC-32 (CRC kataloğundaki adıyla CRC-32/ISO-HDLC; üreteç polinomu 0x04C11DB7):
//! bozulmuş ya da yarım yazılmış kayıtları yakalamak için sağlama değeri. Veri, GF(2)
//! üzerinde bir polinom gibi üreteç polinomuna bölünür; kalan CRC'dir.
//! Tablo derleme anında (const fn) hesaplanır.
const fn tablo_olustur() -> [u32; 256] {
    let mut tablo = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut c = i as u32;
        let mut k = 0;
        while k < 8 {
            // Bir bitlik polinom bölmesi: GF(2)'de çıkarma = XOR.
            // 0xEDB88320, üreteç polinomunun bit sırası ters çevrilmiş yazımıdır.
            c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 };
            k += 1;
        }
        tablo[i] = c;
        i += 1;
    }
    tablo
}

static TABLO: [u32; 256] = tablo_olustur();

pub fn crc32(veri: &[u8]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for &b in veri {
        c = TABLO[((c ^ b as u32) & 0xFF) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFF_FFFF
}

#[cfg(test)]
mod testler {
    use super::crc32;

    #[test]
    fn yayimlanmis_denetim_degeri() {
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926); // CRC kataloğundaki "check" (denetim) değeri
    }

    #[test]
    fn her_tek_bit_degisimi_crcyi_degistirir() {
        let mesaj = crate::gunluk::kodla(&crate::gunluk::Kayit {
            anahtar: "anahtar00500".into(),
            deger: Some("değer-500".into()),
        });
        let asil = crc32(&mesaj);
        let mut denenen = 0;
        for bit in 0..mesaj.len() * 8 {
            let mut b = mesaj.clone();
            b[bit / 8] ^= 1 << (bit % 8); // tek bir biti çevir
            assert_ne!(crc32(&b), asil, "{bit}. bit değişimi yakalanmadı");
            denenen += 1;
        }
        assert_eq!(denenen, 38 * 8); // 38 baytlık kayıt: 304 farklı tek bit bozulması
    }
}

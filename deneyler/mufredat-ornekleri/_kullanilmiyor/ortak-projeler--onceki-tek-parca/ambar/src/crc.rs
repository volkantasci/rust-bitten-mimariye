//! CRC-32 (IEEE 802.3): bozulmuş ya da yarım yazılmış kayıtları yakalamak için sağlama.
//! Tablo derleme anında (const fn) hesaplanır.
const fn tablo_olustur() -> [u32; 256] {
    let mut tablo = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut c = i as u32;
        let mut k = 0;
        while k < 8 {
            c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 }; // polinom bölmesi (GF(2))
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
    #[test]
    fn standart_kontrol_degeri() {
        assert_eq!(super::crc32(b"123456789"), 0xCBF4_3926); // CRC-32 için yayımlanmış kontrol değeri
    }
    #[test]
    fn tek_bit_degisimi_yakalanir() {
        assert_ne!(super::crc32(b"ambar"), super::crc32(b"ambas"));
    }
}

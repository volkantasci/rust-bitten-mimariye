use std::ffi::{CStr, CString, c_char, c_int};

// C tarafının imzalarını Rust tarafında yeniden yazıyoruz. rustc C kaynağını görmez:
// bu satırlar iki tarafın anlaşmasıdır (sözleşme), doğruluğunu biz sağlarız.
unsafe extern "C" {
    fn topla(a: c_int, b: c_int) -> c_int;
    fn selamla(ad: *const c_char);
    fn karesi_arti_bir(x: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize; // libc
}

// C'nin çağırabileceği bir Rust fonksiyonu: C çağırma kuralı, karıştırılmamış ad
#[unsafe(no_mangle)]
pub extern "C" fn rust_kare(x: c_int) -> c_int {
    x * x
}

fn main() {
    let ad = CString::new("Atölye").unwrap(); // UTF-8 baytlar + sonda NUL (0) baytı
    // SAFETY: imzalar c/hesap.c ve libc ile uyumlu; ad.as_ptr() bu satır boyunca geçerli.
    unsafe {
        println!("topla(2, 3) = {}", topla(2, 3));
        selamla(ad.as_ptr());
        println!("strlen(\"Atölye\") = {} bayt, Rust'ta {} karakter", strlen(ad.as_ptr()), "Atölye".chars().count());
        println!("karesi_arti_bir(7) = {} (C, Rust'taki rust_kare'yi çağırdı)", karesi_arti_bir(7));
    }
    let geri: &CStr = ad.as_c_str();
    println!("C dizesinin baytları: {:?}", geri.to_bytes_with_nul());
}

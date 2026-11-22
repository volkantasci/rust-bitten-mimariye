/// Aynı hilenin bariz hâli: & referansı doğrudan *mut'a çevirip yazmak
pub fn say_ve_artir(sayac: &u32) -> u32 {
    let p = sayac as *const u32 as *mut u32;
    unsafe {
        *p += 1;
        *p
    }
}

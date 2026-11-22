// Derlemeden önce çalışır: C kaynağını derleyip statik kütüphane olarak bağlar
fn main() {
    cc::Build::new().file("c/hesap.c").compile("hesap");
    println!("cargo::rerun-if-changed=c/hesap.c");
}

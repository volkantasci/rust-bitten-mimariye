#[no_mangle]
pub extern "C" fn disari_acik() -> i32 {
    1
}

extern "C" {
    fn abs(x: i32) -> i32;
}

fn main() {
    println!("{}", unsafe { abs(-3) } + disari_acik());
}

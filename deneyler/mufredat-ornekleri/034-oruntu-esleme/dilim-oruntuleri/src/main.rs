fn siniflandir(v: &[i32]) -> String {
    match v {
        [] => "boş".to_string(),
        [tek] => format!("tek eleman: {tek}"),
        [ilk, .., son] if ilk == son => format!("baş ve son aynı: {ilk}"),
        [ilk, orta @ .., son] => format!("{ilk} … {son} (arada {} eleman)", orta.len()),
    }
}

fn main() {
    for v in [&[][..], &[7], &[3, 1, 3], &[1, 2, 3, 4, 5]] {
        println!("{:?} → {}", v, siniflandir(v));
    }
}

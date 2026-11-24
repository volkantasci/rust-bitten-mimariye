// Her belirteç ağacını (token tree) ayrı bir satıra yazan küçük bir makro
macro_rules! belirtecler {
    ($($t:tt)*) => {
        $( println!("{}", stringify!($t)); )*
    };
}

fn main() {
    belirtecler!(let toplam_2 = 0x2A + 1_000u32 * (3 - x); a >>= b; "bir dize");
    println!("---");
    belirtecler!(a> >=b); // arada boşluk: bitişik olmayan noktalama birleşmez
}

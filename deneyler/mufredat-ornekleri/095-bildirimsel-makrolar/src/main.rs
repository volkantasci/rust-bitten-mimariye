// Örüntü eşleme belirteçler (token) üzerinde: $anahtar ve $deger birer ifade (expr) parçası
macro_rules! harita {
    ( $( $anahtar:expr => $deger:expr ),* $(,)? ) => {{
        let mut h = std::collections::BTreeMap::new();
        $( h.insert($anahtar, $deger); )* // her eşleşme için bir kez tekrarla
        h
    }};
}

// Özyinelemeli makro: en az bir argüman
macro_rules! en_buyuk {
    ($x:expr) => { $x };
    ($x:expr, $($geri:expr),+) => {{
        let a = $x;
        let b = en_buyuk!($($geri),+);
        if a > b { a } else { b }
    }};
}

// Hijyen: makronun içindeki x, çağıranın x'iyle karışmaz
macro_rules! iki_kati {
    ($e:expr) => {{
        let x = 2;
        x * $e
    }};
}

fn main() {
    let h = harita! { "elma" => 3, "armut" => 5, "ayva" => 1, };
    println!("{h:?}");
    println!("en_buyuk!(3, 9, 4, 7) = {}", en_buyuk!(3, 9, 4, 7));
    let x = 10;
    println!("iki_kati!(x) = {} (makronun x'i 2, bizim x'imiz 10)", iki_kati!(x));
}

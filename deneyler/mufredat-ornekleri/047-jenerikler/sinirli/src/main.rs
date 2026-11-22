fn en_buyuk<T: PartialOrd + Copy>(liste: &[T]) -> T {
    let mut eb = liste[0];
    for &x in liste {
        if x > eb {
            eb = x;
        }
    }
    eb
}

fn main() {
    println!("{}", en_buyuk(&[3, 9, 2]));
    println!("{}", en_buyuk(&[1.5, -2.0, 0.25]));
    println!("{}", en_buyuk(&['ç', 'a', 'z']));
}

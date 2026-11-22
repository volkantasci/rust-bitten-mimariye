fn en_buyuk<T>(liste: &[T]) -> &T {
    let mut eb = &liste[0];
    for x in liste {
        if x > eb {
            eb = x;
        }
    }
    eb
}

fn main() {
    println!("{}", en_buyuk(&[3, 9, 2]));
}

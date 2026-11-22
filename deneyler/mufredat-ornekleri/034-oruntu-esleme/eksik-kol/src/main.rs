enum Yon {
    Kuzey,
    Guney,
    Dogu,
    Bati,
}

fn adim(y: Yon) -> (i32, i32) {
    match y {
        Yon::Kuzey => (0, 1),
        Yon::Guney => (0, -1),
        Yon::Dogu => (1, 0),
    }
}

fn main() {
    println!("{:?}", adim(Yon::Bati));
}

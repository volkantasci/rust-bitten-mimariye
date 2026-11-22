use std::any::type_name_of_val;

fn main() {
    let x = 5;
    let y = {
        let kare = x * x;
        kare + 1 // noktalı virgül yok: bloğun değeri
    };
    let tur = if y % 2 == 0 { "çift" } else { "tek" };
    let mut sayac = 0;
    let sonuc = loop {
        sayac += 1;
        if sayac == 10 {
            break sayac * 2; // döngü de değer döndürebilir
        }
    };
    println!("y = {y} ({tur}), sonuc = {sonuc}");

    let tamsayi = 7; // varsayılan tamsayı tipi
    let kesirli = 2.5; // varsayılan kesirli tip
    let mut liste = Vec::new(); // tipi aşağıdaki kullanımdan çıkarılır
    liste.push(3_u8);
    println!("{} {} {}", type_name_of_val(&tamsayi), type_name_of_val(&kesirli), type_name_of_val(&liste));
}

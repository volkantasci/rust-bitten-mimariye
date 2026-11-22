fn main() {
    let s1 = String::from("tapu");
    println!("s1: yığındaki yapı {:p}, öbekteki veri {:p}", &s1, s1.as_ptr());
    let s2 = s1; // sahiplik s2'ye geçti (taşıma)
    println!("s2: yığındaki yapı {:p}, öbekteki veri {:p}", &s2, s2.as_ptr());
}

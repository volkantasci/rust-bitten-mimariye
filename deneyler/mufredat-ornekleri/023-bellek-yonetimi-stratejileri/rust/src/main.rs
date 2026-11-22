struct Kaynak(&'static str);

impl Drop for Kaynak {
    fn drop(&mut self) {
        println!("  {} bırakıldı", self.0);
    }
}

fn main() {
    let _a = Kaynak("A");
    {
        let _b = Kaynak("B");
        let _c = Kaynak("C");
        println!("iç kapsam bitiyor");
    } // C ve B burada, ters sırayla bırakılır
    let d = Kaynak("D");
    drop(d); // erken bırakma
    println!("main bitiyor");
} // A burada bırakılır

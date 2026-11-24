#@ video: 097-prosedurel-makrolar
#@ degisken: derleme dizinindeki özet (hash) klasör adları
k cargo run -q -p kullanici
# Prosedürel makro, derleyicinin yüklediği bir paylaşılan kütüphane (.so) olarak derlenir
ks 'file $(find target/debug -name "libtanit_turet-*.so") | sed -E "s#/[0-9a-f]{16}/#/…/#; s#-[0-9a-f]{16}\.so#.so#; s#, BuildID[^,]*##"'
# türetme makrosunun ürettiği kod (genişletilmiş hâl, yalnızca nightly)
ks 'cargo +nightly rustc -q -p kullanici --target-dir target/nightly -- -Zunpretty=expanded 2>&1 | sed -n "/^impl Siparis/,/^}/p"'
k cargo build -q -p hatali-kullanim
ks 'cargo tree -p tanit-turet -e normal --depth 1'
# Derleme anında makronun .so dosyasını açan süreç (strace -Y süreç adını gösterir: <rustc>); özetler … ile kısaltıldı
ks 'touch kullanici/src/main.rs && strace -f -Y -e trace=openat cargo build -q -p kullanici 2>&1 | grep -E "libtanit_turet-[0-9a-f]+\.so\", O_RDONLY" | head -n 1 | sed -E "s#^\[pid +[0-9]+(<[^>]+>)\] #\1 #; s#/[0-9a-f]{16}/#/…/#; s#-[0-9a-f]{16}\.so#-….so#; s#= [0-9]+\$#= (fd)#"'
# Kendin dene: adsız alanlı struct için de hata üretmek (makronun bir kopyasında); hata Nokta satırını gösterir
ks 'rm -rf /tmp/tanit-kopya && mkdir /tmp/tanit-kopya && cp -r Cargo.toml Cargo.lock tanit-turet kullanici hatali-kullanim /tmp/tanit-kopya/ && cd /tmp/tanit-kopya && sed -i "s/            _ => Vec::new(),/            _ => return syn::Error::new_spanned(ad, \"Tanit adlı alanları olan struct ister\").to_compile_error().into(),/" tanit-turet/src/lib.rs && printf "\n#[derive(Tanit)]\nstruct Nokta(i32, i32);\n" >> kullanici/src/main.rs && cargo build -q -p kullanici 2>&1 | grep -A4 "^error: Tanit adlı"'

#@ video: 049-trait-nesneleri-ve-vtable
#@ degisken: adresler (ASLR)
cd vtable && k cargo run -q; cd ..
cd dyn-uyumsuz && k cargo build -q; cd ..

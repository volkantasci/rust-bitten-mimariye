#@ video: 093-ffi-c-ile-konusmak
k cargo run -q
# İkilide iki dünyanın sembolleri yan yana
ks 'nm target/debug/ffi | grep -E " T (topla|selamla|karesi_arti_bir|rust_kare)$"'

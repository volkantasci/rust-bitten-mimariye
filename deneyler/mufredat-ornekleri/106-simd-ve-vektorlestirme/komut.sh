#@ video: 106-simd-ve-vektorlestirme
#@ degisken: süreler
k cargo build -q --release
k ./target/release/simd
# Hangi toplama komutları? addss: tek (skaler) f32, addps: 4'lü paket (SSE)
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/simd | awk "/<simd::toplam_basit>:/,/^$/" | grep -oE "\b(addss|addps|vaddps)\b" | sort | uniq -c'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/simd | awk "/<simd::toplam_8_serit>:/,/^$/" | grep -oE "\b(addss|addps|vaddps)\b" | sort | uniq -c'
# Bu işlemciye özel derleme: AVX2 ile 8'li paketler (vaddps ymm)
ks 'RUSTFLAGS="-C target-cpu=native" cargo build -q --release --target-dir target/native && ./target/native/release/simd'
ks 'objdump -d -M intel --no-show-raw-insn -C target/native/release/simd | awk "/<simd::toplam_8_serit>:/,/^$/" | grep -E "vaddps +ymm" | head -n 2'

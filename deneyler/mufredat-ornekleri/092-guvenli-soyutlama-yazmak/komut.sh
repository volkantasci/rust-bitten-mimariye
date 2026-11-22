#@ video: 092-guvenli-soyutlama-yazmak
#@ goster: Undefined Behavior|error: |test result|---- |hatali_surum|dogru_surum
# Normal testler: ikisi de "geçer" — hatalı sürümün tanımsız davranışı görünmez
k cargo test -q
# miri altında: doğru sürüm temiz, hatalı sürüm yakalanır
ks 'cargo +nightly miri test -q dogru_surum 2>&1 | grep -E "test result|running"'
ks 'cargo +nightly miri test -q hatali_surum 2>&1 | grep -E "Undefined Behavior|error:|-->" | head -n 6'

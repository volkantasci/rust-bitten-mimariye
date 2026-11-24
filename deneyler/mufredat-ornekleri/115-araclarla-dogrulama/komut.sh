#@ video: 115-araclarla-dogrulama
#@ degisken: Miri'nin etiket ve ayırma numaraları; süreler. proptest tohumu sabit (PROPTEST_RNG_SEED=1): karşı örnek herkeste aynı çıkar
rm -rf proptest-regressions
export PROPTEST_DISABLE_FAILURE_PERSISTENCE=1
# 0) Bariz hâli: derleyicinin varsayılan "deny" denetimi yakalar
cd bariz && k cargo build -q; cd ..
# 1) Dolaylı hâli derlenir; örnek testler geçer, farklılık (diferansiyel) testi patlar
ks 'PROPTEST_RNG_SEED=1 cargo test -q 2>&1 | grep -E -A4 "minimal failing input" | grep -vE "^(--|thread|note:|\s*$)"'
ks 'PROPTEST_RNG_SEED=1 cargo test -q 2>&1 | grep "test result"'
# Kendin dene: başka bir tohumla küçültme başka (daha uzun) bir karşı örnekte durabilir
ks 'PROPTEST_RNG_SEED=2 cargo test -q medyan_basvuruyla 2>&1 | grep -E -A5 "minimal failing input" | grep -vE "^(--|thread|note:|\s*$)"'
# Küçültme adımları (PROPTEST_VERBOSE=2, aynı tohum): her denenen girdi ve sonucu, sırayla
ks 'PROPTEST_RNG_SEED=1 PROPTEST_VERBOSE=2 cargo test -q medyan_basvuruyla 2>&1 | awk "/Next test input/ {g=\$0; sub(/.*v = /, \"\", g)} /Test case passed/ {print g \" → geçti\"} /Test case failed/ {print g \" → başarısız\"}"'
# Kendin dene: medyan_hizli'yi düzeltmek (girdi &[i32], select_nth_unstable &mut ister: önce kopya); özellik testi geçer
ks 'rm -rf /tmp/medyan-kopya && mkdir /tmp/medyan-kopya && cp -r Cargo.toml Cargo.lock src /tmp/medyan-kopya/ && cd /tmp/medyan-kopya && sed -i "s/    if v.is_empty() { None } else { Some(v\[v.len() \/ 2\]) }/    if v.is_empty() { None } else { let mut k = v.to_vec(); let orta = k.len() \/ 2; Some(*k.select_nth_unstable(orta).1) }/" src/lib.rs && grep -n "select_nth_unstable" src/lib.rs && PROPTEST_RNG_SEED=1 cargo test -q 2>&1 | grep "test result" | head -n 1'
# 2) Miri: "geçen" sayaç testindeki tanımsız davranışı yakalar
ks 'cargo +nightly miri test -q sayac_artiyor 2>&1 | grep -E "Undefined Behavior|-->|test result" | head -n 4'
rm -rf proptest-regressions

#@ video: 116-dogru-soruyu-sormak
#@ degisken: iş parçacığı kimlikleri, test süreleri
# Önce sözleşme: imza + belge + testler; gövde todo!()
cd 1-sozlesme && ks 'cargo test -q 2>&1 | grep -E "not yet implemented|test result" | sort | uniq -c'; cd ..
# Sonra gerçekleştirim (ajan ya da insan): aynı testler
cd 2-uygulama && ks 'cargo test -q 2>&1 | grep "test result" | head -n 1'; cd ..
# Kendin dene: sözleşmeye altıncı madde (genişlik 0 ise boş liste); bugünkü gerçekleştirim bu testi geçemez
ks 'rm -rf /tmp/sarmala-kopya && mkdir /tmp/sarmala-kopya && cp -r 2-uygulama/Cargo.toml 2-uygulama/Cargo.lock 2-uygulama/src /tmp/sarmala-kopya/ && cd /tmp/sarmala-kopya && sed -i "\$ d" src/lib.rs && printf "    #[test]\n    fn genislik_sifir_bos_liste() {\n        assert_eq!(sarmala(\"bir iki\", 0), Vec::<String>::new());\n    }\n}\n" >> src/lib.rs && cargo test -q 2>&1 | grep -E "^test result|left:|right:"'

#@ video: 046-traitler-davranis-sozlesmesi
cd sozlesme && k cargo run -q; cd ..
# Sözleşmenin bir maddesi (alan) yazılmazsa derleyici kabul etmez
cd eksik && k cargo build -q; cd ..

#@ video: 089-async-tuzaklari
#@ degisken: milisaniye değerleri
cd bloklama && k cargo run -q; cd ..
cd kilit-await && k cargo build -q; cd ..
# Kendin dene: kilidi await'ten önce bırakmak
cd kilit-await-duzeltilmis && k cargo run -q; cd ..

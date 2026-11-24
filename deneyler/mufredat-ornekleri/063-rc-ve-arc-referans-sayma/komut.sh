#@ video: 063-rc-ve-arc-referans-sayma
cd sayac && k cargo run -q; cd ..
# Rc'yi başka bir iş parçacığına göndermek derlenmez (Rc: !Send); Arc'ın atomik sayacı bunun için var
cd rc-is-parcacigi && k cargo build -q; cd ..

#@ video: 077-ag-katmanlari-ve-soketler
#@ degisken: port numaraları ve dosya tanımlayıcı numaraları
k cargo build -q
k ./target/debug/soketler
# Aynı programın ağ sistem çağrıları
ks 'strace -f -e trace=socket,bind,listen,accept4,connect,write,sendto,recvfrom,read,shutdown,close ./target/debug/soketler 2>&1 | grep -E "socket\(|bind\(|listen\(|accept4\(|connect\(|sendto\(|recvfrom\(" '

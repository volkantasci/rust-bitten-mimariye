#include <stdio.h>

/* Rust tarafında tanımlı, C'den çağrılacak fonksiyon: C tarafının beyanı */
extern int rust_kare(int x);

int topla(int a, int b) { return a + b; }

void selamla(const char *ad) { printf("C diyor ki: merhaba %s\n", ad); fflush(stdout); }

int karesi_arti_bir(int x) { return rust_kare(x) + 1; }   /* C → Rust */

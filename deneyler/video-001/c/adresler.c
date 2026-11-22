#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* kullan_sonra_birak2.c'nin aynısı; ek olarak iki bloğun adresini yazdırır.
   Adresi iade etmeden ÖNCE bir tamsayıya kopyalıyoruz, çünkü iade edilmiş bir
   işaretçinin değerini kullanmak (karşılaştırmak bile) C'de tanımsız davranıştır. */
void temizle(char *p) { free(p); }

int main(void) {
    char *isim = malloc(16);
    strcpy(isim, "Ferris");
    uintptr_t isim_adresi = (uintptr_t)isim;
    temizle(isim);

    char *sifre = malloc(16);
    strcpy(sifre, "gizli-123");
    uintptr_t sifre_adresi = (uintptr_t)sifre;

    printf("isim  bloğu: %#jx\n", (uintmax_t)isim_adresi);
    printf("sifre bloğu: %#jx\n", (uintmax_t)sifre_adresi);
    printf("aynı blok mu? %s\n", isim_adresi == sifre_adresi ? "evet" : "hayır");
    printf("isim = %s\n", isim);
    free(sifre);
    return 0;
}

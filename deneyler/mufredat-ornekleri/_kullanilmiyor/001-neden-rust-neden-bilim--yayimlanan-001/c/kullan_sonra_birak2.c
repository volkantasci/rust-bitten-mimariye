#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Aynı hata, ama iade işlemi başka bir fonksiyonda */
void temizle(char *p) { free(p); }

int main(void) {
    char *isim = malloc(16);
    strcpy(isim, "Ferris");
    temizle(isim);

    char *sifre = malloc(16);
    strcpy(sifre, "gizli-123");

    printf("isim = %s\n", isim);
    free(sifre);
    return 0;
}

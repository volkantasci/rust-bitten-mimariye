#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    char *isim = malloc(16);
    strcpy(isim, "Ferris");
    free(isim);                      /* bellek iade edildi */

    char *sifre = malloc(16);        /* allocator aynı yeri yeniden verebilir */
    strcpy(sifre, "gizli-123");

    printf("isim = %s\n", isim);     /* iade edilmiş belleği okuyoruz */
    free(sifre);
    return 0;
}

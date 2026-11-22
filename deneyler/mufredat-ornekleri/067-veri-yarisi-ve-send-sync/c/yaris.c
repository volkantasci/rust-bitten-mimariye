#include <pthread.h>
#include <stdio.h>

volatile long sayac = 0;            /* paylaşılan, korumasız */

void *artir(void *arg) {
    for (int i = 0; i < 10000000; i++)
        sayac++;                    /* oku - artır - yaz: üç ayrı adım */
    return NULL;
}

int main(void) {
    pthread_t a, b;
    pthread_create(&a, NULL, artir, NULL);
    pthread_create(&b, NULL, artir, NULL);
    pthread_join(a, NULL);
    pthread_join(b, NULL);
    printf("beklenen 20000000, bulunan %ld\n", sayac);
    return 0;
}

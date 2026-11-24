#include <stdlib.h>

int main(void) {
    char *p = malloc(32);
    free(p);
    free(p);          /* aynı bellek iki kez iade ediliyor */
    return 0;
}

#include <stdio.h>
#include <string.h>

int main(int argc, char **argv) {
    char tampon[8];
    strcpy(tampon, argc > 1 ? argv[1] : "kisa");   /* uzunluk denetimi yok */
    printf("tampon = %s\n", tampon);
    return 0;
}

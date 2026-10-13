#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[]) {
    char* str = (char*)malloc(sizeof(char) * 10);
    strncpy(str, "123456789", 10);
    printf("String: %s\n", str);
    printf("Sizeof: %zu\n", strlen(str));

    free(str);
    puts(""); return 0;
}

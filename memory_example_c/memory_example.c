#include <stdio.h>
#include <stdlib.h>

int main() {

    char *str = malloc(6); //allocates 6 bytes of memory to the variable str
    str[0] = 'h';
    str[1] = 'e';
    str[2] = 'l';
    str[3] = 'l';
    str[4] = 'o';
    str[5] = '\0'; //indicates end of a string
    //str now has a value, "hello"
    printf(str);
    free(str); //frees up the memory used by str
    
    return 0;
}
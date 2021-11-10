#include <stdio.h>

struct S1 {
    u_int8_t a;
    u_int16_t b;
    u_int8_t c;
};

struct S2 {
    u_int8_t a;
    u_int8_t b;
    u_int16_t c;
};

int main() {
 
 printf("size of S1: %d\n size of S2: %d\n",sizeof(struct S1),sizeof(struct S2));
 
 return 0;
 
}
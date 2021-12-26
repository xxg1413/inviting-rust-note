#include <stdio.h>

int hello() 
{
    printf("Hello World!\n");

    return 0;
}

int main()
{

    int (*p)() = &hello;
    (*p)();

    int *p1 = (int *)p;
    p1[1]=0xdeadbeef;

}

/*output:
Hello World!
Segmentation fault
*/
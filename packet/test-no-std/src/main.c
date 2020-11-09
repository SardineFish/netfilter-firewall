#include <memory.h>
#include <stdlib.h>
#include <stdio.h>

unsigned char* extern_alloc(size_t size)
{
    printf("alloc for %ld bytes.\n", size);
    return malloc(size);
}

unsigned char* extern_dealloc(unsigned char * ptr)
{
    printf("dealloc memory at 0x%llx.\n", ptr);
    free(ptr);
}

extern void rust_main();

int main()
{
    rust_main();
    return 0;
}
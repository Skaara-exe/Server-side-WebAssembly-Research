#include "../hello_world.h"
#include <cstdio>

int32_t exports_hello_world_add(int32_t a, int32_t b) {
    return a + b;
}

void exports_hello_world_greet(void) {
    printf("Hello, world!\n");
}
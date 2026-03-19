#include <cstdio>

int main()
{
    const char* msg[] = {"Hello", "World", "!"};

    for (const char* word : msg)
    {
        printf("%s ", word);
    }
    printf("\n");
}
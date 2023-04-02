#include "rust_api.h"
#include <stdio.h>

int main()
{
    uint32_t array[]= {5,2,1,7,5};
    uint32_t sum = array_sum(array, sizeof(array)/ sizeof(array[0]));

    printf("sum %u\n", sum);

    array_sort(array, sizeof(array)/ sizeof(array[0]));

    printf("after sord : ");
    for (size_t i = 0; i < 5; i++)
    {
        printf("%u, ", array[i]);
    }
    

    getchar();    
}

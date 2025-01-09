#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

int main(void) {
    char buffer[32];
    if (!fgets(buffer, sizeof(buffer), stdin)) {
        return 1;
    }

    int value = atoi(buffer);

    // Attempt to multiply by 1000; if 'value' is large enough, it can overflow
    int result = value * 1000;
    printf("Result of value * 1000: %d\n", result);

    // Force a situation where an extreme value might cause unexpected branching
    if (result < 0) {
        // Possibly do something invalid
        char *ptr = NULL;
        ptr[0] = 'B'; // Crash if negative triggers unexpected logic
    }

    return 0;
}


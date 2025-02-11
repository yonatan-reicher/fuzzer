#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    char *ptr = NULL;
    char input[32];

    if (!fgets(input, sizeof(input), stdin)) {
        return 0;
    }
    // Trim newline if present
    input[strcspn(input, "\n")] = '\0';

    /*
     * Condition: If the input length exceeds 20, we attempt to use ptr.
     * This is purely length-based, so a typical fuzzer that tries random lengths
     * will eventually trigger the vulnerability.
     */
    if (strlen(input) > 20) {
        // Dereference a NULL pointer
        ptr[0] = 'A'; // Crash
    }

    printf("Done.\n");
    return 0;
}


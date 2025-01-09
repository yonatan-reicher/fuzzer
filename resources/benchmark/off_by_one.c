#include <stdio.h>
#include <string.h>

int main(void) {
    char buffer[8];
    int len;

    if (!fgets(buffer, sizeof(buffer), stdin)) {
        return 1;
    }

    len = strlen(buffer);
    // Intentionally writing a null terminator one past the buffer's last valid index
    buffer[len] = '\0';  // Potential off-by-one if the buffer is full

    printf("Processed string: %s\n", buffer);
    return 0;
}


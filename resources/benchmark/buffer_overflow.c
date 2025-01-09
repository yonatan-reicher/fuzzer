#include <stdio.h>
#include <string.h>

void vulnerable_function(char *input) {
    char buffer[10];
    strcpy(buffer, input);  // No bounds checking
    printf("Input: %s\n", buffer);
}

int main() {
    char input[100];
    printf("Enter input: ");
    scanf("%99s", input);
    vulnerable_function(input);
    return 0;
}


#include "url.h"
#include <stdio.h>
#include <string.h>

int main(void)
{
    char url_input[8000];
    memset(url_input, 0, sizeof(url_input));
    fgets(url_input, sizeof(url_input), stdin);
    // remove newline because it crashes the url lib
    size_t len = strlen(url_input);
    if (len > 0 && url_input[len - 1] == '\n')
    {
        url_input[len - 1] = '\0'; // Replace newline with null terminator
    }
    // url_data_t *parsed = url_parse("git://git@github.com:jwerle/url.h.git");
    // URL BUG
    // library has a buffer overflow on too large schemes (> 32 chars)
    printf("scheme: %s\n", url_get_scheme(url_input));
    printf("input: %s\n", url_input);
    url_data_t *parsed = url_parse(url_input);

    if(parsed) {
        url_data_inspect(parsed);

        url_free(parsed);
    }

   return 0; 
}
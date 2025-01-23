#include <uriparser/Uri.h>
#include <stdio.h>

int main(void) {
    UriUriA uri;
    const char *uriString = "http://username:password@www.example.com:8080/path?query=string#fragment";
    const char *errorPos;

    char url_input[8000];
    memset(url_input, 0, sizeof(url_input));
    fgets(url_input, sizeof(url_input), stdin);
    // remove newline because it crashes the url lib
    size_t len = strlen(url_input);
    if (len > 0 && url_input[len - 1] == '\n')
    {
        url_input[len - 1] = '\0'; // Replace newline with null terminator
    }
    printf("input: %s\n", url_input);

    if (uriParseSingleUriA(&uri, url_input, &errorPos) != URI_SUCCESS) {
        fprintf(stderr, "Error parsing URI at position: %ld\n", errorPos - url_input);
        return 0;
    }

    // Scheme
    if (uri.scheme.first) {
        printf("Scheme: %.*s\n", (int)(uri.scheme.afterLast - uri.scheme.first), uri.scheme.first);
    }

    // User Info
    if (uri.userInfo.first) {
        printf("User Info: %.*s\n", (int)(uri.userInfo.afterLast - uri.userInfo.first), uri.userInfo.first);
    }

    // Host
    if (uri.hostText.first) {
        printf("Host: %.*s\n", (int)(uri.hostText.afterLast - uri.hostText.first), uri.hostText.first);
    }

    // Port
    if (uri.portText.first) {
        printf("Port: %.*s\n", (int)(uri.portText.afterLast - uri.portText.first), uri.portText.first);
    }

    // Path
    if (uri.pathHead) {
        printf("Path: ");
        UriPathSegmentA *pathSegment = uri.pathHead;
        while (pathSegment) {
            printf("/%.*s", (int)(pathSegment->text.afterLast - pathSegment->text.first), pathSegment->text.first);
            pathSegment = pathSegment->next;

        }
        printf("\n");
    }

    // Query
    if (uri.query.first) {
        printf("Query: %.*s\n", (int)(uri.query.afterLast - uri.query.first), uri.query.first);
    }

    // Fragment
    if (uri.fragment.first) {
        printf("Fragment: %.*s\n", (int)(uri.fragment.afterLast - uri.fragment.first), uri.fragment.first);
    }

    uriFreeUriMembersA(&uri);
    return 0;
}
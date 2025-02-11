#!/usr/bin/env python3

import sys


if __name__ == '__main__':
    a = sys.stdin.read()
    print(a)
    if 'a' in a:
        exit(1)
    else:
        while True: pass

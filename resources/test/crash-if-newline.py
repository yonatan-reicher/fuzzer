#!/usr/bin/env python3

import sys


if __name__ == "__main__":
    while True:
        if '\n' in sys.stdin.read():
            print('A newline????!!')
            exit(1)

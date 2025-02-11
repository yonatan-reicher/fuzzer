#!/usr/bin/env python3


if __name__ == '__main__':
    try:
        while True:
            if 'a' in input():
                exit(1)
    except EOFError:
        pass

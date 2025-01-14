import os
import subprocess


THIS_DIR = os.path.realpath(os.path.dirname(__file__))
WORDS_PATH = THIS_DIR + "/words.txt"
GENERATE_WORDS_CPP_PATH = THIS_DIR + "/generate_words.cpp"
# Use EXE so it works both on Windows and Linux
GENERATE_WORDS_EXE_PATH = THIS_DIR + "/generate_words.exe"


def compile_word_generator():
    subprocess.run(
        [
            "g++",
            "-O2",
            "-o",
            GENERATE_WORDS_EXE_PATH,
            GENERATE_WORDS_CPP_PATH
        ],
        check=True,
    )


def run_word_generator():
    subprocess.run(
        [
            "cargo",
            "run",
            "--release",
            "--",
            "--strings",
            GENERATE_WORDS_EXE_PATH,
        ],
        check=True,
    )


def rm_words():
    subprocess.run(
        [
            "rm",
            WORDS_PATH,
        ],
    )


def rm_word_generator():
    subprocess.run(
        [
            "rm",
            GENERATE_WORDS_EXE_PATH,
        ],
        check=True,
    )


def generate_words():
    compile_word_generator()
    rm_words()
    run_word_generator()
    rm_word_generator()



def read_words(file_path = WORDS_PATH) -> dict[bytes, int]:
    ret = {}
    count = 0
    with open(file_path, "rb") as f:
        try:
            while True:
                len_bytes = f.read(8)
                if len_bytes == b'': break
                len = int.from_bytes(len_bytes, signed=False, byteorder='little')
                word = f.read(len)
                if word not in ret: ret[word] = 0
                ret[word] += 1
                count += 1
        except:
            print(f"Error reading word number {count}")
    return ret


def main():
    generate_words()
    words = read_words()
    sorted_by_values = sorted(words.items(), key=lambda item: item[1])
    for word, count in sorted_by_values:
        print(f"{count}: {word[:50]}")

if __name__ == "__main__":
    main()

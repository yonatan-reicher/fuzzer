import os
import subprocess


THIS_DIR = os.path.realpath(os.path.dirname(__file__))
WORDS_PATH = THIS_DIR + "/words.txt"
GENERATE_WORDS_CPP_PATH = THIS_DIR + "/generate_words.cpp"
# Use EXE so it works both on Windows and Linux
GENERATE_WORDS_EXE_PATH = THIS_DIR + "/generate_words.exe"
BIG_LIST_OF_NAUGTHY_STRINGS_PATH = "./resources/big-list-of-naughty-strings.txt"


def get_big_list_of_naugthy_strings() -> list[bytes]:
    with open(BIG_LIST_OF_NAUGTHY_STRINGS_PATH, "rb") as f:
        return f.read().splitlines()


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


def read_words(file_path=WORDS_PATH) -> dict[bytes, int]:
    ret = {}
    count = 0
    with open(file_path, "rb") as f:
        try:
            while True:
                len_bytes = f.read(8)
                if len_bytes == b'':
                    break
                len = int.from_bytes(
                    len_bytes, signed=False, byteorder='little')
                word = f.read(len)
                if word not in ret:
                    ret[word] = 0
                ret[word] += 1
                count += 1
        except:
            print(f"Error reading word number {count}")
    return ret


def prettify_word(word: bytes, max_chars: int | None = 150) -> str:
    if max_chars is None:
        word_str = None
        try:
            # a space to align with b'...'
            word_str = ' "' + word.decode("utf-8") + '"'
        except:
            pass

        return (
            word_str
            if word_str and word_str != '' and word_str.isprintable() else
            str(word)
        )

    p = prettify_word(word, None)
    if len(p) <= max_chars:
        return p
    return p[:max_chars - 3] + "..."


def main():
    generate_words()
    words = read_words()
    naughty_strings = set(get_big_list_of_naugthy_strings())
    sorted_by_values = sorted(words.items(), key=lambda item: item[1])

    print("Naughty strings:")
    for word, count in sorted_by_values:
        if word not in naughty_strings:
            continue
        print(f"{count}: {prettify_word(word)}")

    print("Our generated strings:")
    for word, count in sorted_by_values:
        if word in naughty_strings:
            continue
        print(f"{count}: {prettify_word(word)}")


if __name__ == "__main__":
    main()

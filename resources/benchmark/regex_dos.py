import re

def vulnerable_function(input_str):
    try:
        pattern = re.compile(r"^([a-zA-Z]+)+$")
        if pattern.fullmatch(input_str):
            print("Matched!")
    except re.error:
        print("Regex error!")

if __name__ == "__main__":
    user_input = input("Enter input: ")
    vulnerable_function(user_input)


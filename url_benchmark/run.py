#!/usr/bin/env python3

import os
import subprocess
import argparse
import sys

# ANSI escape codes for colors
class Colors:
    RED = '\033[91m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    CYAN = '\033[96m'
    RESET = '\033[0m'

def colorize(text, color):
    return f"{color}{text}{Colors.RESET}"

def run_command(command, cwd=None, shell=False):
    try:
        result = subprocess.run(command, cwd=cwd, check=True, stderr=sys.stderr, stdout=sys.stdout, text=True, shell=shell)
        if result.stdout:
            print(colorize(result.stdout.strip(), Colors.GREEN))
        return result
    except subprocess.CalledProcessError as e:
        print("caught an error")
        if e.stdout:
            print(colorize(e.stdout.strip(), Colors.YELLOW))
        if e.stderr:
            print(colorize(e.stderr.strip(), Colors.RED))
        raise
        
def immediate_subfolders(root):
    return [os.path.join(root, d) for d in os.listdir(root) if os.path.isdir(os.path.join(root, d))]

def process_testcases(root_folder, custom_cmake_command):
    all_targets_built = True

    for subdir in immediate_subfolders(root_folder):
        has_cmake = False
        files = os.listdir(subdir)
        if 'Makefile' in files:
            if 'CMakeLists.txt' in files:
                print(colorize(f"Running custom cmake command in {subdir}", Colors.CYAN))
                has_cmake = True
                try:
                    run_command(custom_cmake_command, cwd=subdir)
                except subprocess.CalledProcessError:
                    all_targets_built = False

            print(colorize(f"Running make in {subdir}", Colors.CYAN))
            try:
                run_command(['make'], cwd=subdir)
            except subprocess.CalledProcessError:
                all_targets_built = False

            main_c_file = os.path.join(subdir, 'main.c')
            main_executable = os.path.join(subdir, 'main')

            if has_cmake:
                print(colorize(f"Compiling {main_c_file} with gcc to create main executable", Colors.CYAN))
                try:
                    run_command(['gcc', 'main.c', '-o', 'main', f'-I./include', 
                                 f'-L./', '-luriparser', '--static'], cwd=subdir)
                except subprocess.CalledProcessError:
                    print(colorize(f"Failed to compile {main_c_file} in {subdir}", Colors.RED))
                    all_targets_built = False

            if os.path.isfile(main_executable):
                print(colorize(f"Successfully built 'main' in {subdir}", Colors.GREEN))
            else:
                print(colorize(f"'main' executable not found after gcc in {subdir}", Colors.RED))
                all_targets_built = False

    return all_targets_built

def run_fuzzer_tests(root_folder, fuzzer_path):
    for subdir in immediate_subfolders(root_folder):
        main_executable = os.path.join(subdir, 'main')
        if os.path.isfile(main_executable):
            print(colorize(f"Running fuzzer on {main_executable}", Colors.CYAN))
            try:
                run_command([fuzzer_path, '--urls', main_executable])
            except subprocess.CalledProcessError as e:
                if e.stderr:
                    print(colorize(f"Error running fuzzer on {main_executable}: {e.stderr.strip()}", Colors.RED))
                if e.stdout:
                    print(colorize(f"Error running fuzzer on {main_executable}: {e.stderr.strip()}", Colors.YELLOW))


def clean_make_cmake(root_folder):
    for subdir in immediate_subfolders(root_folder):
        run_command("make clean", cwd=subdir, shell=True)
    run_command('find . \( -name "*.so" -o -name "*.a" -o -name "main" -o -name "CMakeCache.txt"  -o -name "CMakeFiles" \) -exec rm -rf {} +', shell=True)
    


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Build and process test cases.")
    parser.add_argument("--fuzzer-path", type=str, help="Path to the fuzzer to run.")
    parser.add_argument("--testcases-folder", type=str, default="./testcases", help="Path to the testcases folder.")
    parser.add_argument("--clean", action='store_true', help="Clean compilation and generate files")
    args = parser.parse_args()

    testcases_folder = args.testcases_folder
    fuzzer_path = args.fuzzer_path
    custom_cmake_command = ['cmake', '-DCMAKE_BUILD_TYPE=Release', '-DURIPARSER_BUILD_TESTS=OFF', 
                            '-DURIPARSER_BUILD_DOCS=OFF', '-DURIPARSER_SHARED_LIBS=OFF'] 
    
    if args.clean:
        clean_make_cmake(testcases_folder)
        sys.exit(0)

    if not os.path.exists(testcases_folder):
        print(colorize(f"Error: Folder '{testcases_folder}' does not exist.", Colors.RED))
    else:
        all_built = process_testcases(testcases_folder, custom_cmake_command)
        print(colorize("All targets successfully built.", Colors.GREEN))
        if all_built:
            if fuzzer_path:
                print(colorize("Running fuzzer on all 'main' executables.", Colors.GREEN))
                run_fuzzer_tests(testcases_folder, fuzzer_path)
        else:
            print(colorize("Not all targets were built successfully.", Colors.RED))
            if fuzzer_path:
                print(colorize("Skipping fuzzer execution.", Colors.RED))

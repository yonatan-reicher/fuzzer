import os
import subprocess
import time


def is_windows() -> bool:
    return os.name == "nt"



BENCHMARK_DIR = "resources/benchmark"
COMPILED_DIR = "resources/benchmark/bin"

C_COMPILER = "gcc"
CPP_COMPILER = "g++"
C_FLAGS = ["-g", "-O2"]          # or e.g., ["-fsanitize=address", "-g", "-O1"]
CPP_FLAGS = ["-g", "-O2"]        # similarly, can add sanitizers or coverage

FUZZER_BUILD_CMD = [
    "cargo",
    "build",
    "--release",
]

FUZZER_CMD = [
    "./target/release/fuzzer" if not is_windows() else "./target/release/fuzzer.exe",
    "--strings",
]


def compile_source(source_path: str, output_path: str):
    _, ext = os.path.splitext(source_path)

    if ext == ".c":
        cmd = [C_COMPILER, *C_FLAGS, source_path, "-o", output_path]
    elif ext == ".cpp":
        cmd = [CPP_COMPILER, *CPP_FLAGS, source_path, "-o", output_path]
    else:
        print(f"[WARN] Unsupported extension {ext}, skipping {source_path}")
        return False

    print(f"[INFO] Compiling {source_path} -> {output_path}")
    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if result.returncode != 0:
        print(f"[ERROR] Failed to compile {source_path}:\n{result.stderr}")
        return False
    return True


def build_fuzzer():
    print("[INFO] Building fuzzer...", end="")
    completed = subprocess.run(
        FUZZER_BUILD_CMD,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=True
    )
    if completed.returncode != 0:
        print("failed!")
        raise RuntimeError(f"Failed to build fuzzer: {completed.stderr}")
    print("done!")


def run_fuzzer_on_file(executable_path: str):
    """
    Invokes the Rust fuzzer with --strings <executable_path>.
    Returns (return_code, stdout, stderr, duration).
    """
    cmd = FUZZER_CMD + [executable_path]
    print(f"running: {cmd}")
    start_time = time.perf_counter()
    completed = subprocess.run(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    end_time = time.perf_counter()

    duration = end_time - start_time
    return (completed.returncode, completed.stdout, completed.stderr, duration)


def main():
    # 1) Create the compiled_binaries directory if it does not exist
    os.makedirs(COMPILED_DIR, exist_ok=True)

    # 2) Collect all files in resources/benchmark
    all_files = os.listdir(BENCHMARK_DIR)
    source_files = [f for f in all_files if os.path.isfile(os.path.join(BENCHMARK_DIR, f))
                    and (f.endswith(".c") or f.endswith(".cpp"))]

    # 3) Compile each .c / .cpp
    compiled_paths = []
    for filename in source_files:
        base, ext = os.path.splitext(filename)
        # Construct full paths
        src_path = os.path.join(BENCHMARK_DIR, filename)
        out_path = os.path.join(COMPILED_DIR, base)  # no extension for the executable
        success = compile_source(src_path, out_path)
        if success:
            compiled_paths.append(out_path)
        else:
            print(f"[WARN] Skipping fuzzer run for {filename} due to compile error.")

    if not compiled_paths:
        print("[INFO] No successfully compiled binaries. Exiting.")
        return

    # 4) Run the fuzzer against each compiled binary
    build_fuzzer()
    for compiled_binary in compiled_paths:
        print(f"\n=== Running fuzzer on '{compiled_binary}' ===")
        rc, out, err, duration = run_fuzzer_on_file(
                compiled_binary if not is_windows() else f"{compiled_binary}.exe"
            )
        print(f"Return code     : {rc}")
        print(f"Execution time  : {duration:.2f} seconds")
        print("Fuzzer stdout   :")
        print(out.strip())
        if err.strip():
            print("Fuzzer stderr   :")
            print(err.strip())

        # Optional: Save logs
        log_file_name = f"fuzzer_{os.path.basename(compiled_binary)}.log"
        with open(log_file_name, "w") as log_file:
            log_file.write(out)
            log_file.write("\n--- STDERR ---\n")
            log_file.write(err)

if __name__ == "__main__":
    main()

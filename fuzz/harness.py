#!/usr/bin/env python3
import sys
import os
import time
import subprocess
import random
import string
import datetime

# Ensure original python library can be imported
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "original")))

try:
    import inflection as py_inflection
except ImportError:
    print("Error: Could not import Python 'inflection' library from original/ directory.")
    sys.exit(1)

METHODS = [
    "pluralize",
    "singularize",
    "camelize",
    "underscore",
    "dasherize",
    "humanize",
    "titleize",
    "parameterize",
]

CLI_BINARY = os.path.abspath(
    os.path.join(os.path.dirname(__file__), "..", "target", "release", "inflection")
)

LOG_FILE = os.path.abspath(os.path.join(os.path.dirname(__file__), "log.txt"))


def generate_random_string():
    length = random.randint(1, 30)
    chars = string.ascii_letters + string.digits + " _-äöüÄÖÜ"
    return "".join(random.choice(chars) for _ in range(length))


def run_rust_cli(method, input_str):
    cmd = [CLI_BINARY, method, input_str]
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        return None
    return result.stdout.strip()


def run_python_inflection(method, input_str):
    func = getattr(py_inflection, method)
    return str(func(input_str))


def main():
    if not os.path.exists(CLI_BINARY):
        print(f"Error: Rust binary not found at {CLI_BINARY}. Run 'cargo build --release' first.")
        sys.exit(1)

    duration = 65  # Target minimum duration in seconds
    start_time = time.time()
    end_time = start_time + duration

    iterations = 0
    divergence_count = 0
    divergences = []

    print(f"Starting differential fuzzing against Rust binary for {duration} seconds...")

    while time.time() < end_time:
        method = random.choice(METHODS)
        test_input = generate_random_string()

        try:
            py_res = run_python_inflection(method, test_input)
            rust_res = run_rust_cli(method, test_input)

            if rust_res is None:
                divergence_count += 1
                divergences.append(f"CRASH/ERROR on {method}('{test_input}')")
            elif py_res != rust_res:
                divergence_count += 1
                divergences.append(
                    f"DIVERGENCE on {method}('{test_input}'): Python='{py_res}' vs Rust='{rust_res}'"
                )

        except Exception as e:
            divergence_count += 1
            divergences.append(f"EXCEPTIONAL FAIL on {method}('{test_input}'): {str(e)}")

        iterations += 1

    elapsed = round(time.time() - start_time, 2)

    # Format log report
    report = [
        "==================================================",
        "PORT MORTEM DIFFERENTIAL FUZZING REPORT",
        "==================================================",
        f"Timestamp    : {datetime.datetime.now(datetime.timezone.utc).isoformat()}",
        f"Duration     : {elapsed} seconds",
        f"Total Tests  : {iterations:,}",
        f"Divergences  : {divergence_count}",
        f"Status       : {'PASSED' if divergence_count == 0 else 'FAILED'}",
        "==================================================",
    ]

    if divergences:
        report.append("\nDivergence Details:")
        for d in divergences[:50]:  # Cap to top 50 in log
            report.append(f" - {d}")

    report_content = "\n".join(report) + "\n"

    os.makedirs(os.path.dirname(LOG_FILE), exist_ok=True)
    with open(LOG_FILE, "w") as f:
        f.write(report_content)

    print(f"Fuzzing completed in {elapsed}s across {iterations:,} iterations.")
    print(f"Report written to {LOG_FILE}")

    if divergence_count > 0:
        print(f"FAILED: Found {divergence_count} divergences between Python and Rust implementations.")
        sys.exit(1)
    else:
        print("SUCCESS: Zero divergences found!")
        sys.exit(0)


if __name__ == "__main__":
    main()

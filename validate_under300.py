#!/usr/bin/env python3
"""Validate all Python solutions for problems < 300."""
import subprocess
import sys
import os
import time

def load_answers():
    answers = {}
    for path in ['../Solutions.txt', 'solutions.txt']:
        if not os.path.exists(path):
            continue
        with open(path) as f:
            for line in f:
                line = line.strip()
                if '. ' in line and path.endswith('Solutions.txt'):
                    parts = line.split('. ', 1)
                    try:
                        num = int(parts[0])
                        answers[num] = parts[1]
                    except:
                        pass
                elif line.startswith('Problem ') and ': ' in line:
                    parts = line.split(': ', 1)
                    try:
                        num = int(parts[0].replace('Problem ', ''))
                        if num not in answers:
                            answers[num] = parts[1]
                    except:
                        pass
    return answers

def main():
    answers = load_answers()
    timeout = 60
    results = []

    for num in range(1, 300):
        script = f'python/{num:03d}.py' if num < 100 else f'python/{num}.py'
        if not os.path.exists(script):
            # Try without leading zeros
            script = f'python/{num}.py'
            if not os.path.exists(script):
                results.append((num, 'MISSING', None, answers.get(num)))
                continue

        expected = answers.get(num)
        start = time.time()
        try:
            proc = subprocess.run(
                [sys.executable, script],
                capture_output=True, text=True, timeout=timeout
            )
            elapsed = time.time() - start
            stdout = proc.stdout.strip()

            # Get first line of output
            first_line = stdout.split('\n')[0] if stdout else ''

            if not stdout:
                status = 'NO_OUTPUT'
            elif expected and first_line == expected:
                status = 'CORRECT'
            elif expected and first_line != expected:
                # Check if output is verbose (contains the answer somewhere)
                status = 'WRONG'
            else:
                status = 'UNKNOWN'

            print(f'{num}|{status}|{first_line}|{expected}|{elapsed:.1f}s', flush=True)
            results.append((num, status, first_line, expected, elapsed))
        except subprocess.TimeoutExpired:
            elapsed = time.time() - start
            print(f'{num}|TIMEOUT|None|{expected}|{elapsed:.1f}s', flush=True)
            results.append((num, 'TIMEOUT', None, expected, elapsed))
        except Exception as e:
            elapsed = time.time() - start
            print(f'{num}|ERROR|{e}|{expected}|{elapsed:.1f}s', flush=True)
            results.append((num, 'ERROR', str(e), expected, elapsed))

if __name__ == '__main__':
    main()

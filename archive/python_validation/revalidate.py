#!/usr/bin/env python3
"""Re-validate problematic README entries and output updated results."""
import subprocess
import time
import sys
import os
import concurrent.futures

# Problems to re-validate (all with None, verbose output, or suspicious answers)
PROBLEMS = [
    302, 305, 309, 311, 312, 319, 320, 322, 323, 324, 325, 326, 327, 328, 331,
    334, 335, 337, 338, 339, 340, 341, 342, 343, 344, 349, 350, 352, 353, 354,
    355, 356, 360, 361, 362, 364, 365, 366, 368, 369, 370, 373, 374, 375, 376,
    378, 379, 380, 383, 384, 385, 386, 393, 394, 395, 397, 398, 399, 400, 404,
    405, 406, 407, 408, 411, 413, 414, 415, 416, 417, 421, 424, 426, 427, 428,
]

EXPECTED = {
    302: "1170060", 305: "18174995535140", 309: "210139",
    311: "2466018557", 312: "324681947", 319: "268457129",
    320: "278157919195482643", 322: "999998760323313995",
    323: "6.3551758451", 324: "96972774", 325: "54672965",
    326: "1966666166408794329", 327: "34315549139516",
    328: "260511850222", 331: "467178235146843549",
    334: "15032002126169083", 335: "5032316", 337: "85068035",
    338: "15614292", 339: "19823.542204", 340: "291504964",
    341: "56098610614277014", 342: "5943040885644",
    343: "269533451410884183", 344: "65579304332",
    349: "115384615384614952", 350: "84664213",
    352: "378563.260589", 353: "1.2759860331", 354: "58065134",
    355: "1726545007", 356: "28010159", 360: "878825614395267072",
    361: "178476944", 362: "457895958010", 364: "44855254",
    365: "162619462356610313", 366: "88351299",
    368: "253.6135092068", 369: "862400558448",
    370: "41791929448408", 373: "727227472448913",
    374: "334420941", 375: "7435327983715286168",
    376: "973059630185670", 378: "147534623725724718",
    379: "132314136838185", 380: "6.3202e25093",
    383: "22173624649806", 384: "3354706415856332783",
    385: "3776957309612153700", 386: "528755790",
    393: "112398351350823112", 394: "3.2370342194",
    395: "28.2453753155", 397: "141630459461893728",
    398: "2010.59096", 399: "1508395636674243,6.5e27330467",
    400: "438505383468410633", 404: "1199215615081353",
    405: "237696125", 406: "36813.12757207",
    407: "39782849136421", 408: "299742733", 411: "9936352",
    413: "3079418648040719", 414: "552506775824935461",
    415: "55859742", 416: "898082747", 417: "446572970925740",
    421: "2304215802083466198", 424: "1059760019628",
    426: "31591886008", 427: "97138867", 428: "747215561862",
}


def validate_problem(prob):
    """Run a single problem and return (prob, answer, time_s, correct)."""
    # Try subdirectory first, then flat file
    script = f"python/{prob}/{prob}.py"
    if not os.path.exists(script):
        script = f"python/{prob}.py"
    if not os.path.exists(script):
        return (prob, None, 0, False, "no_file")

    start = time.time()
    try:
        result = subprocess.run(
            ["python", script],
            capture_output=True, text=True, timeout=300,
            cwd="/home/v/01_projects/euler_project/euler"
        )
        elapsed = time.time() - start
        stdout = result.stdout.strip()
        if not stdout:
            return (prob, None, elapsed, False, "no_output")

        # Get the last non-empty line as the answer
        lines = [l.strip() for l in stdout.split('\n') if l.strip()]
        answer = lines[-1] if lines else ""

        # Check against expected
        expected = EXPECTED.get(prob, "")
        # Normalize comparison
        correct = answer == expected

        return (prob, answer, elapsed, correct, "ok")
    except subprocess.TimeoutExpired:
        elapsed = time.time() - start
        return (prob, None, elapsed, False, "timeout")
    except Exception as e:
        elapsed = time.time() - start
        return (prob, None, elapsed, False, str(e))


def format_time(seconds):
    if seconds >= 1:
        return f"{seconds:.3f}s"
    else:
        return f"{int(seconds * 1000)}ms"


def main():
    print(f"Re-validating {len(PROBLEMS)} problems with 300s timeout...")
    print(f"Running up to 12 in parallel...\n")

    results = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=12) as executor:
        futures = {executor.submit(validate_problem, p): p for p in PROBLEMS}
        for future in concurrent.futures.as_completed(futures):
            prob = futures[future]
            result = future.result()
            prob, answer, elapsed, correct, status = result
            mark = "PASS" if correct else "FAIL"
            ans_display = (answer[:60] + "...") if answer and len(answer) > 60 else answer
            print(f"  {prob:>3}: {mark} | {format_time(elapsed):>10} | {status:>10} | {ans_display}")
            results.append(result)

    results.sort(key=lambda x: x[0])

    print("\n" + "=" * 80)
    print("SUMMARY")
    print("=" * 80)

    passed = [r for r in results if r[3]]
    failed = [r for r in results if not r[3]]

    print(f"\nPASSED: {len(passed)}")
    for prob, answer, elapsed, correct, status in passed:
        print(f"  | {prob} | `{answer}` | {format_time(elapsed)} |")

    print(f"\nFAILED: {len(failed)}")
    for prob, answer, elapsed, correct, status in failed:
        exp = EXPECTED.get(prob, "?")
        ans_display = (answer[:50] + "...") if answer and len(answer) > 50 else answer
        print(f"  {prob}: got={ans_display}, expected={exp}, status={status}, time={format_time(elapsed)}")


if __name__ == "__main__":
    main()

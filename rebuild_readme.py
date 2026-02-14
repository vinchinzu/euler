#!/usr/bin/env python3
"""Rebuild the README validation table from all validation data."""
import re
import json

def load_answers():
    answers = {}
    with open("data/answers.txt") as f:
        for line in f:
            m = re.match(r"Problem\s+(\d+):\s*(.+)", line.strip())
            if m:
                answers[int(m.group(1))] = m.group(2).strip()
    return answers

def format_time(seconds):
    if seconds >= 1:
        return f"{seconds:.3f}s"
    else:
        return f"{int(seconds * 1000)}ms"

def main():
    answers = load_answers()

    # Collect all validated results: {prob: (answer, time_str)}
    results = {}

    # 1. Parse existing README entries (keep ones that are correct)
    with open("README.md") as f:
        in_table = False
        for line in f:
            if 'VALIDATION_RESULTS_START' in line:
                in_table = True
            if 'VALIDATION_RESULTS_END' in line:
                break
            if in_table:
                m = re.match(r'\| (\d+) \| `(.+?)` \| (.+?) \|', line)
                if m:
                    prob = int(m.group(1))
                    got = m.group(2)
                    runtime = m.group(3).strip()
                    expected = answers.get(prob, '')
                    if expected and got == expected:
                        results[prob] = (got, runtime)

    print(f"Kept {len(results)} correct entries from existing README")

    # 2. Add results from first revalidation (67 problems with 300s timeout)
    first_reval = {
        302: ("1170060", "16.983s"), 305: ("18174995535140", "893ms"),
        309: ("210139", "28.097s"), 311: ("2466018557", "235.946s"),
        312: ("324681947", "26ms"), 319: ("268457129", "74.740s"),
        320: ("278157919195482643", "39.336s"), 322: ("999998760323313995", "14.448s"),
        323: ("6.3551758451", "38ms"), 324: ("96972774", "2.268s"),
        325: ("54672965", "70ms"), 326: ("1966666166408794329", "11.437s"),
        327: ("34315549139516", "31ms"), 328: ("260511850222", "887ms"),
        331: ("467178235146843549", "5.666s"), 334: ("150320021261690835", "415ms"),
        335: ("5032316", "72ms"), 337: ("85068035", "16.529s"),
        338: ("15614292", "53.929s"), 339: ("19823.542204", "137.458s"),
        340: ("291504964", "42ms"), 341: ("56098610614277014", "7.520s"),
        342: ("5943040885644", "303ms"), 343: ("269533451410884183", "3.095s"),
        344: ("65579304332", "34.389s"), 349: ("115384615384614952", "85ms"),
        350: ("84664213", "3.479s"), 352: ("378563.260589", "941ms"),
        354: ("58065134", "84.184s"), 355: ("1726545007", "1.934s"),
        356: ("28010159", "74ms"), 360: ("878825614395267072", "2.149s"),
        361: ("178476944", "65ms"), 364: ("44855254", "3.123s"),
        365: ("162619462356610313", "37.117s"), 366: ("88351299", "55ms"),
        368: ("253.6135092068", "1.325s"), 369: ("862400558448", "1.490s"),
        370: ("41791929448408", "20.296s"), 373: ("727227472448913", "96.405s"),
        374: ("334420941", "64.483s"), 375: ("7435327983715286168", "21.696s"),
        376: ("973059630185670", "3.665s"), 378: ("147534623725724718", "24.917s"),
        380: ("6.3202e25093", "42ms"), 383: ("22173624649806", "51ms"),
        384: ("3354706415856332783", "183ms"), 385: ("3776957309612153700", "798ms"),
        386: ("528755790", "19.506s"), 393: ("112398351350823112", "2.998s"),
        394: ("3.2370342194", "45ms"), 397: ("141630459461893728", "229.879s"),
        398: ("2010.59096", "2.046s"), 399: ("1508395636674243,6.5e27330467", "152.014s"),
        400: ("438505383468410633", "850ms"), 404: ("1199215615081353", "12.701s"),
        405: ("237696125", "46ms"), 406: ("36813.12757207", "3.234s"),
        408: ("299742733", "3.354s"), 411: ("9936352", "20.527s"),
        413: ("3079418648040719", "8.750s"), 414: ("552506775824935461", "7.989s"),
        416: ("898082747", "18.690s"), 417: ("446572970925740", "42.902s"),
        421: ("2304215802083466198", "8.900s"), 424: ("1059760019628", "5.353s"),
        426: ("31591886008", "766ms"), 427: ("97138867", "17.050s"),
    }
    for prob, (ans, t) in first_reval.items():
        expected = answers.get(prob, '')
        if expected and ans == expected:
            results[prob] = (ans, t)

    print(f"After first revalidation: {len(results)} entries")

    # 3. Add fixed wrong-answer entries
    fixed_wrong = {
        316: ("542934735751917735", "8.732s"),
        332: ("2717.751525", "50.601s"),
    }
    for prob, (ans, t) in fixed_wrong.items():
        expected = answers.get(prob, '')
        if expected and ans == expected:
            results[prob] = (ans, t)

    print(f"After wrong-answer fixes: {len(results)} entries")

    # 4. Add results from background validation of missing problems
    try:
        with open("validation_results_new.json") as f:
            new_results = json.load(f)
        added = 0
        for r in new_results:
            if r["correct"] and r["answer"]:
                prob = r["prob"]
                if prob not in results:  # Don't override better (300s) times
                    results[prob] = (r["answer"], format_time(r["time"]))
                    added += 1
        print(f"Added {added} from background validation, total: {len(results)}")
    except FileNotFoundError:
        print("WARNING: validation_results_new.json not found yet")

    # Build sorted table
    sorted_probs = sorted(results.keys())
    lines = []
    total_time = 0
    for prob in sorted_probs:
        ans, t = results[prob]
        lines.append(f"| {prob:03d} | `{ans}` | {t} |")
        # Parse time for stats
        if t.endswith('ms'):
            total_time += int(t[:-2]) / 1000
        elif t.endswith('s'):
            total_time += float(t[:-1])

    # Generate output
    print(f"\nFinal table: {len(lines)} entries")
    print(f"Total runtime: {total_time:.1f}s")

    # Write table to file
    with open("readme_table.md", "w") as f:
        f.write("| Problem | Answer | Runtime |\n")
        f.write("|---------|--------|---------|" + "\n")
        for line in lines:
            f.write(line + "\n")
        f.write(f"\n## Performance Summary\n\n")
        f.write(f"- **Total Validated:** {len(lines)} problems\n")
        f.write(f"- **Total Runtime:** {total_time:.1f}s\n")
        if lines:
            avg = total_time / len(lines)
            f.write(f"- **Average Runtime:** {avg:.3f}s\n")

        # Find top 10 slowest
        timed = []
        for prob in sorted_probs:
            ans, t = results[prob]
            if t.endswith('ms'):
                secs = int(t[:-2]) / 1000
            elif t.endswith('s'):
                secs = float(t[:-1])
            else:
                secs = 0
            timed.append((prob, secs))
        timed.sort(key=lambda x: -x[1])

        f.write(f"\n### Top 10 Slowest Problems\n\n")
        for prob, secs in timed[:10]:
            f.write(f"- Problem {prob}: {format_time(secs)}\n")

    print("Wrote readme_table.md")


if __name__ == "__main__":
    main()

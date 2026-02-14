#!/usr/bin/env python3
"""Merge timeout re-run results into README.md. Safe to re-run."""
import re, json

def format_time(s):
    return f"{s:.3f}s" if s >= 1 else f"{int(s*1000)}ms"

def main():
    # Load timeout results
    with open("timeout_progress.json") as f:
        progress = json.load(f)

    passed = {int(k): v for k, v in progress.items() if v["correct"]}
    print(f"Timeout re-run: {len(progress)} done, {len(passed)} passed")

    # Find which are already in README
    with open("README.md") as f:
        content = f.read()

    existing = set()
    for m in re.finditer(r'^\| (\d+) \|', content, re.M):
        existing.add(int(m.group(1)))

    new_entries = {p: v for p, v in passed.items() if p not in existing}
    if not new_entries:
        print("No new entries to add.")
        return

    print(f"Adding {len(new_entries)} new entries: {sorted(new_entries)}")

    # Insert into table
    lines = content.split('\n')
    output = []
    inserted = set()
    in_table = False

    for line in lines:
        if '| Problem | Answer | Runtime |' in line:
            in_table = True
            output.append(line)
            continue
        if in_table and line.startswith('|'):
            m = re.match(r'\| (\d+) \|', line)
            if m:
                prob = int(m.group(1))
                for new_prob in sorted(new_entries):
                    if new_prob not in inserted and new_prob < prob:
                        v = new_entries[new_prob]
                        t = format_time(v["time"])
                        output.append(f"| {new_prob:03d} | `{v['answer']}` | {t} |")
                        inserted.add(new_prob)
        elif in_table and not line.startswith('|'):
            for new_prob in sorted(new_entries):
                if new_prob not in inserted:
                    v = new_entries[new_prob]
                    t = format_time(v["time"])
                    output.append(f"| {new_prob:03d} | `{v['answer']}` | {t} |")
                    inserted.add(new_prob)
            in_table = False
        output.append(line)

    new_total = len(existing) + len(inserted)
    result = '\n'.join(output)
    result = re.sub(r'\*\*Total Passed:\*\* \d+ problems', f'**Total Passed:** {new_total} problems', result)
    result = re.sub(r'\*\*Total Validated:\*\* \d+ problems', f'**Total Validated:** {new_total} problems', result)

    with open("README.md", "w") as f:
        f.write(result)

    print(f"Updated README: {new_total} total entries (+{len(inserted)} new)")

if __name__ == "__main__":
    main()


import re

def parse_readme():
    with open('README.md', 'r') as f:
        content = f.read()
    
    # Extract the table
    # Table starts after | Problem | Answer | Runtime |
    # and ends at the Performance Summary or end of file
    
    table_match = re.search(r'\| Problem \| Answer \| Runtime \|\n\|[-|]+\|\n(.*?)\n\n', content, re.DOTALL)
    if not table_match:
        print("Could not find table")
        return

    table_content = table_match.group(1)
    lines = table_content.strip().split('\n')
    
    present_ids = set()
    none_ids = set()
    
    for line in lines:
        parts = [p.strip() for p in line.split('|')]
        if len(parts) >= 3:
            try:
                prob_id = int(parts[1])
                present_ids.add(prob_id)
                
                answer = parts[2].strip('`')
                if answer == 'None' or answer == '':
                    none_ids.add(prob_id)
            except ValueError:
                continue

    # 1. Missing < 300
    missing_lt_300 = []
    for i in range(1, 300):
        if i not in present_ids:
            missing_lt_300.append(i)
            
    # 2. Have NONE (any ID)
    # The user said "missing <300 or that have NONE". 
    # This implies "Have NONE" applies to all IDs, or maybe just <300? 
    # Usually lists like this want all broken things. I'll include all NONEs.
    # But I should probably sort them.
    
    all_todo = set(missing_lt_300) | none_ids
    sorted_todo = sorted(list(all_todo))
    
    with open('TODO.md', 'w') as f:
        f.write("# Project Euler TODO List\n\n")
        f.write("Problems that are either missing (for ID < 300) or have no solution (NONE).\n\n")
        
        f.write("| Problem | Status | Note |\n")
        f.write("|---------|--------|------|\n")
        
        for pid in sorted_todo:
            if pid in missing_lt_300:
                f.write(f"| {pid:03d} | Missing | Not in README |\n")
            elif pid in none_ids:
                f.write(f"| {pid:03d} | Failed | Answer is None |\n")
                
    print(f"Generated TODO.md with {len(sorted_todo)} problems.")

if __name__ == "__main__":
    parse_readme()

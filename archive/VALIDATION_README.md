# Project Euler Validation Script

## Overview
The improved `validate.py` script validates all Python solutions with robust error handling, progress tracking, and automatic resume capabilities.

## Key Features

### 1. Two-Pass Validation
- **First Pass**: 60-second timeout for each problem
- **Second Pass**: 120-second timeout for problems that timed out in first pass
- Efficiently handles both fast and slow solutions

### 2. Fault Tolerance
- **Saves after every problem** - results written to JSON immediately
- **Interruptible** - can be stopped at any time (Ctrl+C)
- **Auto-resume** - when restarted, skips already validated correct answers
- **Atomic writes** - uses temporary file + rename for safe JSON updates

### 3. Answer Sources
- Primary: `solutions_b.txt` (971 answers)
- Fallback: `Solutions.txt`
- Automatically skips header lines

### 4. Progress Tracking
```
[42/937] Problem 42... ✅ PASSED (0.035s)
[43/937] Problem 43... ⏰ TIMEOUT (60.0s)
[44/937] Problem 44... ❌ FAILED (0.788s)
    Expected: 5482660
    Got:      5482659
[45/937] Problem 45... 🔥 ERROR (0.034s)
    Return code 1: ModuleNotFoundError...
```

### 5. Result Storage
**File**: `validation_results.json`

**Structure**:
```json
{
  "last_updated": "2026-01-13T18:24:52.040363",
  "total_problems": 937,
  "results": {
    "1": {
      "status": "passed",
      "expected": "233168",
      "actual": "233168",
      "error": null,
      "timeout": false,
      "runtime": 0.020,
      "pass": "First Pass"
    },
    ...
  }
}
```

**Status Values**:
- `passed` - Correct answer
- `failed` - Wrong answer
- `error` - Runtime error
- `timeout` - Exceeded time limit
- `unknown` - No expected answer available

### 6. Summary Report
At completion, displays:
- Total solutions tested
- Pass/fail/error/timeout counts
- Performance statistics
- Top 10 slowest problems
- List of all problems with issues

## Usage

### Basic Usage
```bash
python validate.py
```

### Help
```bash
python validate.py --help
```

### Monitor Progress
```bash
# In another terminal
tail -f validation_run.log

# Or check JSON directly
python3 -c "import json; d=json.load(open('validation_results.json')); print(f'Progress: {d[\"total_problems\"]}')"
```

### Interrupt and Resume
```bash
# Start validation
python validate.py

# Press Ctrl+C to interrupt

# Check what was saved
python3 -c "import json; d=json.load(open('validation_results.json')); print(json.dumps(d, indent=2))"

# Resume - will skip already validated problems
python validate.py
```

### View Results
```python
import json

data = json.load(open('validation_results.json'))
results = data['results']

# Find all failed problems
failed = [p for p, r in results.items() if r['status'] == 'failed']
print(f"Failed problems: {failed}")

# Find slowest problems
slowest = sorted(results.items(), key=lambda x: x[1].get('runtime', 0), reverse=True)[:10]
for prob, res in slowest:
    print(f"Problem {prob}: {res['runtime']:.3f}s")

# Find all timeouts
timeouts = [p for p, r in results.items() if r['status'] == 'timeout']
print(f"Timeout problems: {timeouts}")
```

## Example Run

```
$ python validate.py
Loading answers from solutions_b.txt...
Loaded 971 expected answers
Found 937 Python solutions
Already validated: 0 passed problems

======================================================================
First Pass: Testing 937 problems (timeout: 60s)
======================================================================

[1/937] Problem 1... ✅ PASSED (0.020s)
[2/937] Problem 2... ✅ PASSED (0.018s)
[3/937] Problem 3... ✅ PASSED (0.017s)
...
[937/937] Problem 967... ✅ PASSED (0.125s)

======================================================================
Second Pass: Testing 12 problems (timeout: 120s)
======================================================================

[1/12] Problem 187... ✅ PASSED (45.2s)
[2/12] Problem 493... ⏰ TIMEOUT (120.0s)
...

✅ Results saved to validation_results.json

======================================================================
VALIDATION SUMMARY
======================================================================
Total solutions tested: 937
✅ Passed: 925 (98.7%)
❌ Failed: 8
🔥 Errors: 2
⏰ Timeouts: 2
❓ Unknown: 0

⏱️  Performance Stats (for 925 passed solutions):
   Total runtime: 1234.56s
   Average runtime: 1.335s
   Max runtime: 45.234s

   Slowest 10 passed problems:
     Problem 187: 45.234s
     Problem 145: 38.123s
     ...

⚠️  FAILED/ERROR/TIMEOUT PROBLEMS:

   ❌ Failed (8): 205, 347, 512, 678, 745, 823, 891, 923
   🔥 Errors (2): 456, 789
   ⏰ Timeouts (2): 493, 867
======================================================================
```

## Tips

1. **Run in background**: 
   ```bash
   nohup python validate.py > validation.log 2>&1 &
   ```

2. **Monitor live**:
   ```bash
   tail -f validation.log
   ```

3. **Check progress anytime**:
   ```bash
   python3 -c "import json; d=json.load(open('validation_results.json')); print(f'{d[\"total_problems\"]} problems validated')"
   ```

4. **Clean start**:
   ```bash
   rm validation_results.json
   python validate.py
   ```

5. **Resume from interruption**:
   Just run `python validate.py` again - it automatically skips validated problems

## Performance

- **Sequential processing** - No threading overhead or race conditions
- **Smart caching** - Skips already validated problems
- **Atomic saves** - Safe interruption at any point
- **Minimal memory** - Processes one problem at a time

## Troubleshooting

**Q: Why did it skip so many problems?**  
A: It skips problems that already have `"status": "passed"` in validation_results.json. Delete the file to start fresh.

**Q: Can I run multiple instances?**  
A: No, they would overwrite each other's results. Use one instance at a time.

**Q: How do I re-test specific problems?**  
A: Edit validation_results.json and remove those problem entries, then re-run.

**Q: What if validation_results.json gets corrupted?**  
A: Delete it and start over. The script handles missing or invalid JSON files gracefully.

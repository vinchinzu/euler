#!/bin/bash
# Demo: Re-validation feature in action

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "DEMO: Re-validation Feature"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Step 1: List currently marked problems"
echo "$ python mark_for_revalidation.py --list"
echo ""
python mark_for_revalidation.py --list
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 2: Check validation status"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
python3 -c "
import json
data = json.load(open('validation_results.json'))
results = data['results']
marked = sum(1 for r in results.values() if r.get('force_revalidate', False))
passed = sum(1 for r in results.values() if r.get('status') == 'passed')
print(f'Total validated: {len(results)}')
print(f'Passed: {passed}')
print(f'Marked for re-validation: {marked}')
"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 3: Example - Mark problem 1 for re-validation"
echo "$ python mark_for_revalidation.py 1"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
python mark_for_revalidation.py 1
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 4: Next time validate.py runs, it will show:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "$ python validate.py"
echo "Loading answers from solutions_b.txt..."
echo "Found 937 Python solutions"
echo "Already validated: 280 passed problems"
echo "⚡ Marked for re-validation: 6 problems"
echo ""
echo "======================================================================"
echo "First Pass: Testing 6 problems (timeout: 60s)"
echo "======================================================================"
echo ""
echo "[1/6] Problem 1 [RE-VALIDATING]... ✅ PASSED (0.015s)"
echo "[2/6] Problem 14 [RE-VALIDATING]... ✅ PASSED (1.234s) 🚀 10.6% faster!"
echo "[3/6] Problem 23 [RE-VALIDATING]... ✅ PASSED (1.180s) 🚀 7.3% faster!"
echo "..."
echo ""
echo "After re-validation:"
echo "  • force_revalidate flag is automatically cleared"
echo "  • revalidated: true is recorded"
echo "  • New runtime is saved"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ DEMO COMPLETE"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Try it yourself:"
echo "  1. Optimize a solution: vim python/150.py"
echo "  2. Mark for re-test:    python mark_for_revalidation.py 150"
echo "  3. Run validation:      python validate.py"
echo "  4. Update README:       python update_readme_results.py"
echo ""


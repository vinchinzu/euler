#!/usr/bin/env python3
import subprocess
import time
import statistics
import os

os.chdir("/workspace/rust")

def benchmark_version(version_name, runs=50):
    """Run the binary multiple times and return statistics"""
    binary = "./target/release/p853"
    times = []
    
    for _ in range(runs):
        start = time.perf_counter()
        result = subprocess.run([binary], capture_output=True, text=True, check=True)
        end = time.perf_counter()
        times.append(end - start)
        
        # Verify answer on first run
        if _ == 0:
            answer = result.stdout.strip()
            if answer != "44511058204":
                raise ValueError(f"Wrong answer: {answer}")
    
    # Sort times to get median
    times.sort()
    median = times[len(times)//2]
    mean = statistics.mean(times)
    
    return {
        'name': version_name,
        'median': median * 1000,  # Convert to ms
        'mean': mean * 1000,
        'min': min(times) * 1000,
        'max': max(times) * 1000,
        'runs': runs,
        'answer': '44511058204'
    }

print("=== A/B Test for p853 ===\n")

# Test Version A (master)
print("Building Version A (master)...")
subprocess.run(["git", "checkout", "master", "--", "rust/solutions/src/bin/p853.rs"], 
               cwd="/workspace", capture_output=True, check=True)
subprocess.run(["cargo", "build", "--release", "--bin", "p853"], 
               capture_output=True, check=True)

print("Benchmarking Version A...")
stats_a = benchmark_version("A (master)")

# Test Version B (optimized)
print("\nBuilding Version B (optimized)...")
subprocess.run(["git", "checkout", "cursor/optimize-p853-4d39", "--", "rust/solutions/src/bin/p853.rs"], 
               cwd="/workspace", capture_output=True, check=True)
subprocess.run(["cargo", "build", "--release", "--bin", "p853"], 
               capture_output=True, check=True)

print("Benchmarking Version B...")
stats_b = benchmark_version("B (optimized)")

# Print results
print("\n" + "="*60)
print("RESULTS")
print("="*60)
print(f"\nVersion A (master):")
print(f"  Median: {stats_a['median']:.3f} ms")
print(f"  Mean:   {stats_a['mean']:.3f} ms")
print(f"  Range:  {stats_a['min']:.3f} - {stats_a['max']:.3f} ms")
print(f"  Answer: {stats_a['answer']} ✓")

print(f"\nVersion B (optimized):")
print(f"  Median: {stats_b['median']:.3f} ms")
print(f"  Mean:   {stats_b['mean']:.3f} ms")
print(f"  Range:  {stats_b['min']:.3f} - {stats_b['max']:.3f} ms")
print(f"  Answer: {stats_b['answer']} ✓")

# Calculate improvement
median_improvement = ((stats_a['median'] - stats_b['median']) / stats_a['median']) * 100
mean_improvement = ((stats_a['mean'] - stats_b['mean']) / stats_a['mean']) * 100

print(f"\n{'='*60}")
print("IMPROVEMENT")
print("="*60)
print(f"Median speedup: {median_improvement:+.1f}%")
print(f"Mean speedup:   {mean_improvement:+.1f}%")

if median_improvement >= 5.0:
    print(f"\n✓ PASS: Exceeds 5% target ({median_improvement:.1f}% speedup)")
else:
    print(f"\n✗ FAIL: Below 5% target ({median_improvement:.1f}% speedup)")

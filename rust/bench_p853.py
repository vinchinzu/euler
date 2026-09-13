#!/usr/bin/env python3
import subprocess
import time
import statistics

binary = "./target/release/p853"
runs = 100

times = []
for _ in range(runs):
    start = time.perf_counter()
    subprocess.run([binary], stdout=subprocess.DEVNULL, check=True)
    end = time.perf_counter()
    times.append(end - start)

mean = statistics.mean(times) * 1000  # Convert to ms
stdev = statistics.stdev(times) * 1000 if len(times) > 1 else 0
median = statistics.median(times) * 1000

print(f"Runs: {runs}")
print(f"Mean:   {mean:.3f} ms")
print(f"Median: {median:.3f} ms")
print(f"Stdev:  {stdev:.3f} ms")
print(f"Min:    {min(times)*1000:.3f} ms")
print(f"Max:    {max(times)*1000:.3f} ms")

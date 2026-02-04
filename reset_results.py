
import json
import os

ids = ["154", "177", "195"]
path = "validation_results.json"

if os.path.exists(path):
    with open(path, "r") as f:
        data = json.load(f)
    
    results = data.get("results", data)
    
    for i in ids:
        if i in results:
            print(f"Removing result for {i}")
            del results[i]
            
    # Save back
    # Structure might be {"last_updated": ..., "results": ...} or just dict
    if "results" in data:
        data["results"] = results
    else:
        data = results
        
    with open(path, "w") as f:
        json.dump(data, f, indent=2)
    print("Done")
else:
    print("File not found")

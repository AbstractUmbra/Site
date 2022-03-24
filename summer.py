import json
import pathlib
import sys
from collections import defaultdict

PATH = pathlib.Path(sys.argv[1])

with open(PATH, "r") as f:
    data: dict[str, dict[str, int]] = json.load(f)

totals = defaultdict(int)
for key, value in data.items():
    if key == "total":
        continue
    for item, amount in value.items():
        totals[item] += amount

for item, value in totals.items():
    print(f"{item} has total: {value}")

data["totals"] = totals

with open(PATH, "w") as f:
    json.dump(data, f, indent=4)

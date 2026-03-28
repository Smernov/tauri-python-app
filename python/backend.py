import sys, json

data = json.loads(sys.stdin.read())
number = data.get("number", 0)
print(json.dumps({"squared": number ** 2}))

import pprint

print = pprint.pprint

orbits, pairs = [], {}
with open('src/inputs/day06.txt', 'r') as file:
    for item in file:
        a, b = item.strip().split(')')
        orbits.append((a, b))
        pairs[b] = a
        
# key orbits value
counts = {}
for pair in orbits:
    orbiter, host = pair
    tmp = host
    counts[host] = 0
    while tmp in pairs:
        counts[host] += 1
        tmp = pairs[tmp]

print(sum(counts.values()))
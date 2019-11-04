import json
import os

FILENAME = os.path.join(os.path.abspath(os.getcwd()), 'spells.json')
FILENAME_OUT = os.path.join(os.path.abspath(os.getcwd()), 'spells_new.json')


spells = []

with open(FILENAME) as f:
    data = json.load(f)
    for k, v in data.items():
        schulen = v['schulen']
        for s, l in schulen.items():
            schulen[s] = int(l) if l else 1
        v['schulen'] = schulen

        spells.append(v)

with open(FILENAME_OUT, 'w+') as f:
    f.write(json.dumps(spells, indent=4))

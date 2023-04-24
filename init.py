import os
import subprocess

A = ['a', 'b']
for i in range(1, 25 + 1):
    for a in A:
        path = "day%02d%c" % (i, a)
        os.makedirs(path, exist_ok = True)
        subprocess.run(["cargo", "init", path]) 
        
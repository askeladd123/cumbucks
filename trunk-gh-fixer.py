import sys

try:
    parent = sys.argv[1]
except IndexError:
    raise Exception("Missing argument <parent>. ")

try:
    path = sys.argv[2]
except IndexError:
    path = "index.html"

inxs = []

with open(path) as file:
    f = file.read()[::-1]

i = 0
while i < len(f):
    if f[i]=='s' and f[i+1]=='j' and f[i+2]=='.':
        while f[i]!='\"' and f[i]!='\'':
            i += 1
        inxs.append(len(f) - i)

    if f[i]=='m' and f[i+1]=='s' and f[i+2]=='a' and f[i+3]=='w' and f[i+4]=='.':
        while f[i] != "\"" and f[i] != "\'":
            i += 1
        inxs.append(len(f) - i)

    if f[i] == "s" and f[i+1] == "s" and f[i+2] == "c" and f[i+3]=='.':
        while f[i] != "\"" and f[i] != "\'":
            i += 1
        inxs.append(len(f) - i)
    i += 1

cs = list(f[::-1])

for i in inxs:
    cs[i] += parent + "/"

f = ""
for c in cs:
    f += c

with open(path, "w") as file:
    file.write(f)

from collections import deque
import sys

def LI(): return list(map(int, sys.stdin.readline().split()))
def S(): return list(sys.stdin.readline())[:-1]

def SR(n):
    l = [None for i in range(n)]
    for i in range(n):l[i] = S()
    return l

dire4 = [(1,0), (0,1), (-1,0), (0,-1)]

def bfs(H, W, sy, sx, gy, gx, c):
    if (sy, sx) == (gy, gx):
        return 0
    maximum_distance = H*W+1
    dire = [(1,0), (0,1), (-1,0), (0,-1)]
    dist = [[maximum_distance]*W for i in range(H)]
    dist[sy][sx] = 0
    q = deque()

    q_append = q.append
    q_appendleft = q.appendleft
    q_popleft = q.popleft

    q_append((sy, sx))
    while q:
        y, x = q_popleft()
        d = dist[y][x]
        if ((y, x) == (gy, gx)):
            if d <= 2:
                return d
        for dy, dx in dire:
            if (y+dy<H) and (y+dy>=0) and (x+dx<W) and (x+dx>=0):
                if (c[y+dy][x+dx] == "#"):
                    if (dist[y+dy][x+dx] > d+1):
                        dist[y+dy][x+dx] = d+1
                        if (dist[y+dy][x+dx] >= 3):
                            continue
                        q_append((y+dy, x+dx))
                else:
                    if (dist[y+dy][x+dx] > d):
                        dist[y+dy][x+dx] = d
                        q_appendleft((y+dy, x+dx))
    return (dist[gy][gx] if (dist[gy][gx] <= 2) else -1)

def main():
    H, W = LI()
    c = SR(H)

    flag = False
    for i in range(H):
        for j in range(W):
            if (c[i][j] == "s"):
                sy, sx = i, j
                if (flag):
                    break
                flag = True
            elif (c[i][j] == "g"):
                gy, gx = i, j
                if (flag):
                    break
                flag = True
        else:
            continue
        break

    print("YES" if (bfs(H, W, sy, sx, gy, gx, c) != -1) else "NO")


if __name__ == "__main__":
    main()




"""#!usr/bin/env python3
from collections import deque
import sys

def LI(): return list(map(int, sys.stdin.readline().split()))
def S(): return list(sys.stdin.readline())[:-1]

def SR(n):
    return [S() for i in range(n)]

def bfs(H, W, sy, sx, c, gy, gx, dire=[(1,0), (0,1), (-1,0), (0,-1)]):
    dist = [[[False]*3 for j in range(W)] for i in range(H)]

    q = deque()
    q_append = q.append
    q_popleft = q.popleft

    dist[sy][sx][0] = True
    q_append((sy, sx))

    while q:
        y, x = q_popleft()
        d = dist[y][x]
        for dy, dx in dire:
            if (y+dy<H) and (y+dy>=0) and (x+dx<W) and (x+dx>=0):
                dd = dist[y+dy][x+dx]
                if (c[y+dy][x+dx] == "#"):
                    if (d[2]):
                        continue
                    for k in range(2):
                        dd[k+1] = d[k]
                        q_append((y+dy, x+dx))
                else:
                    for i, fl in enumerate(d):
                        if fl:
                            break
                    if not (dd[i]):
                        for k in range(3):
                            dd[k] = d[k]
                            q_append((y+dy, x+dx))

    for i in dist:
        print(i)
    return ("YES" if (dist[gy][gx][0] or dist[gy][gx][1] or dist[gy][gx][2]) else "NO")

def main():
    H, W = LI()
    c = SR(H)

    flag = False
    for i in range(H):
        for j in range(W):
            if (c[i][j] == "s"):
                sy, sx = i, j
                if (flag):
                    break
                flag = True
            elif (c[i][j] == "g"):
                gy, gx = i, j
                if (flag):
                    break
                flag = True
        else:
            continue
        break

    print(bfs(H, W, sy, sx, c, gy, gx))

if __name__ == "__main__":
    main()
"""
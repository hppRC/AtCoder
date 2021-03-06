#!usr/bin/env python3
from collections import defaultdict
from collections import deque
from heapq import heappush, heappop
from itertools import groupby
import sys
import math
import bisect
import random
def LI(): return list(map(int, sys.stdin.readline().split()))
def I(): return int(sys.stdin.readline())
def LS():return list(map(list, sys.stdin.readline().split()))
def S(): return list(sys.stdin.readline())[:-1]
def IR(n):
    l = [None for i in range(n)]
    for i in range(n):l[i] = I()
    return l
def LIR(n):
    l = [None for i in range(n)]
    for i in range(n):l[i] = LI()
    return l
def SR(n):
    l = [None for i in range(n)]
    for i in range(n):l[i] = S()
    return l
def LSR(n):
    l = [None for i in range(n)]
    for i in range(n):l[i] = LS()
    return l
sys.setrecursionlimit(1000000)
mod = 1000000007

S = input()
L = input()

def solve(S, L):
    ans = 0
    for i in range(len(S)):
        Sdash = S[i:]
        counts = [1 if Sdash[j]==L[j] else 0 for j in range(min(len(Sdash), len(L)))]
        ct = 0
        for i in range(len(counts)):
            if (counts[i]):
                ct += 1
            else:
                ans = max(ans, ct)
                ct = 0
        ans = max(ans, ct)
    return ans

print(max(solve(S, L), solve(L, S)))

"""
i = 0
j = 0

ans = 0

for i in range(len(S)):
    for j in range(len(L)):
        k = 0
        count = 0
        while (i+k<len(S) and j+k<len(L)):
            if (S[i+k]==L[j+k]):
                k += 1
                count += 1
            else:
                break

        ans = max(ans, count)
        if (ans == len(S) or ans == len(L)):
            break
    else:
        continue
    break

print(ans)
"""
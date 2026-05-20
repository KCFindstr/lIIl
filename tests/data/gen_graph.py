import heapq
import random

def generate_and_solve(filename, n, m_target):
    edges = []
    adj = [[] for _ in range(n)]
    # Ensure connectivity
    for i in range(n - 1):
        w = random.randint(1, 10)
        edges.append((i, i + 1, w))
        adj[i].append((i + 1, w))

    # Add random edges
    for _ in range(m_target - (n - 1)):
        u = random.randint(0, n - 1)
        v = random.randint(0, n - 1)
        if u != v:
            w = random.randint(1, 20)
            edges.append((u, v, w))
            adj[u].append((v, w))

    m = len(edges)
    start = 0

    # Dijkstra
    dist = [float('inf')] * n
    dist[start] = 0
    pq = [(0, start)]

    while pq:
        d, u = heapq.heappop(pq)
        if d > dist[u]:
            continue
        for v, w in adj[u]:
            if dist[u] + w < dist[v]:
                dist[v] = dist[u] + w
                heapq.heappush(pq, (dist[v], v))

    with open(filename, 'w') as f:
        f.write(f"{n} {m}\n")
        for u, v, w in edges:
            f.write(f"{u} {v} {w}\n")
        f.write(f"{start}\n")
        f.write(" ".join(map(str, dist)) + "\n")

generate_and_solve('tests/data/graph_large.txt', 50, 200)

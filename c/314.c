/*
 * Project Euler Problem 314: The Mouse on the Moon
 *
 * Maximize area/perimeter ratio for a polygon on 500x500 grid of posts.
 * The optimal shape approaches a quarter-circle in each corner.
 *
 * Use Dijkstra-like DP on the grid boundary: for each grid point on the
 * first-quadrant boundary, compute the best (area gained)/(perimeter added)
 * ratio path from (0, 250) to (250, 0), then combine all 4 quadrants.
 *
 * The actual approach: the optimal wall goes from corner-adjacent posts along
 * a curve that approximates a circle. We discretize: for each point (x,y) on
 * the grid boundary in the first quadrant (with x+y near 250), compute the
 * enclosed area vs perimeter cost, using Dijkstra shortest path where "distance"
 * is perimeter - area/target_ratio.
 *
 * Simpler approach: dynamic programming on the grid.
 * Start from (0, n) going to (n, 0) where n=250.
 * At each step move from (x, y) to (x+dx, y-dy) where dx >= 0, dy >= 0, (dx,dy)!=(0,0).
 * Area contribution: trapezoid. Perimeter contribution: sqrt(dx^2+dy^2).
 * Maximize total_area / total_perimeter.
 *
 * Use parametric Dijkstra: binary search on ratio r, find path minimizing
 * (perimeter - area/r), then check if we can achieve ratio >= r.
 *
 * Actually, the standard approach from the PE forum:
 * Path from (0,N) to (N,0) using unit grid moves.
 * For ratio r, define edge weight = sqrt(dx^2+dy^2) - r*(area contribution).
 * Area contribution of step from (x,y) to (x+dx, y-dy) = dx*(y+y-dy)/2 = dx*(2y-dy)/2.
 * But we need the area enclosed between the path and axes.
 * Total area = sum of x_i * dy_i (using the shoelace formula for the polygon
 * formed by the path and the two axis segments).
 *
 * For the path going from (0,N) down-right to (N,0):
 * Area under path = sum of (y_prev + y_next)/2 * dx for each step.
 * Total enclosed area (with axes) = N*N - area_under_path... no.
 *
 * Actually: The full wall encloses approximately a square 500x500 = 250000 m^2.
 * We cut corners. Each corner is a path from one axis to the other.
 * The area lost at each corner is the area between the path and the corner.
 * The perimeter gained is the path length minus (N+N) = 2N for that corner.
 * But really, the full square has perimeter 2000 and area 250000.
 * Cutting corner: path of length L replaces 2*N of the perimeter.
 * Area lost = area of triangle between path and corner.
 *
 * For one corner (first quadrant): path from (0,N) to (N,0).
 * Straight edges would be along axes: length N+N = 500 total for that corner.
 * Path length = L. Perimeter change: L - 2*N.
 * Area of the cut triangle (between path and corner) = A_cut.
 * Total area = 250000 - 4*A_cut, Total perimeter = 4*L.
 * Ratio = (250000 - 4*A_cut) / (4*L) = (250000/4 - A_cut) / L
 *       = (62500 - A_cut) / L
 * Maximize this.
 *
 * A_cut = N*N/2 - A_under_path, where A_under_path is the area under the path
 * (between path and x-axis).
 * So ratio = (62500 - N*N/2 + A_under) / L = (62500 - 31250 + A_under) / L
 *          = (31250 + A_under) / L
 *
 * We want to maximize (31250 + A_under) / L.
 * Equivalently, maximize A_under / L (since 31250/L is maximized by minimizing L,
 * and A_under/L is also in the mix).
 *
 * Parametric search on r: is there a path where A_under - r*L >= C for some threshold?
 * Or: maximize (K + A_under) / L => binary search on r, check if K + A_under - r*L >= 0.
 *
 * Path from (0,N) to (N,0). Moves: from (x,y) to (x+dx, y+dy) where
 * dx >= 0, dy <= 0, not both zero, x+dx <= N, y+dy >= 0.
 * Area under step = (y + (y+dy))/2 * dx = (2*y+dy)/2 * dx.
 * Perimeter of step = sqrt(dx^2 + dy^2).
 *
 * For parametric weight: w(step) = r * sqrt(dx^2+dy^2) - (2*y+dy)/2 * dx
 * Find shortest path (min total weight). If total <= r*0 - K, i.e. if
 * K + A_under - r*L >= 0, then r is achievable.
 *
 * Use Dijkstra on grid (x,y) with x in [0,N], y in [0,N].
 * Edges: (x,y) -> (x+dx, y-dy') for small dx, dy' (limit step size to ~5).
 *
 * This is a well-known approach. Let N=250.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <float.h>

#define NN 250
#define GRID ((NN+1)*(NN+1))
#define MAX_STEP 5 /* limit step size for efficiency */

static double dist[NN+1][NN+1];
static int visited[NN+1][NN+1];

/* Priority queue (min-heap) */
typedef struct { double d; int x, y; } pq_entry;
static pq_entry *heap;
static int heap_size, heap_cap;

static void pq_push(double d, int x, int y) {
    if (heap_size >= heap_cap) {
        heap_cap = heap_cap ? heap_cap * 2 : 1024;
        heap = realloc(heap, heap_cap * sizeof(pq_entry));
    }
    int i = heap_size++;
    heap[i] = (pq_entry){d, x, y};
    while (i > 0) {
        int p = (i - 1) / 2;
        if (heap[p].d <= heap[i].d) break;
        pq_entry tmp = heap[p]; heap[p] = heap[i]; heap[i] = tmp;
        i = p;
    }
}

static pq_entry pq_pop(void) {
    pq_entry top = heap[0];
    heap[0] = heap[--heap_size];
    int i = 0;
    while (1) {
        int l = 2*i+1, r = 2*i+2, m = i;
        if (l < heap_size && heap[l].d < heap[m].d) m = l;
        if (r < heap_size && heap[r].d < heap[m].d) m = r;
        if (m == i) break;
        pq_entry tmp = heap[i]; heap[i] = heap[m]; heap[m] = tmp;
        i = m;
    }
    return top;
}

/* Precompute sqrt values for small steps */
static double step_len[MAX_STEP+1][MAX_STEP+1];

static void init_step_len(void) {
    for (int dx = 0; dx <= MAX_STEP; dx++)
        for (int dy = 0; dy <= MAX_STEP; dy++)
            step_len[dx][dy] = sqrt((double)(dx*dx + dy*dy));
}

/* Run Dijkstra for a given ratio r.
 * Weight of edge from (x,y) to (x+dx, y-dy):
 *   r * sqrt(dx^2+dy^2) - dx*(2*y - dy)/2.0
 * Start: (0, NN). End: (NN, 0).
 * Returns total weight of shortest path.
 */
static double dijkstra(double r) {
    for (int x = 0; x <= NN; x++)
        for (int y = 0; y <= NN; y++) {
            dist[x][y] = DBL_MAX;
            visited[x][y] = 0;
        }

    dist[0][NN] = 0.0;
    heap_size = 0;
    pq_push(0.0, 0, NN);

    while (heap_size > 0) {
        pq_entry e = pq_pop();
        int x = e.x, y = e.y;
        if (visited[x][y]) continue;
        visited[x][y] = 1;

        if (x == NN && y == 0) return e.d;

        /* Try all steps (dx, dy') where we move to (x+dx, y-dy') */
        int max_dx = NN - x;
        if (max_dx > MAX_STEP) max_dx = MAX_STEP;
        int max_dy = y;
        if (max_dy > MAX_STEP) max_dy = MAX_STEP;

        for (int dx = 0; dx <= max_dx; dx++) {
            for (int dy = 0; dy <= max_dy; dy++) {
                if (dx == 0 && dy == 0) continue;
                int nx = x + dx;
                int ny = y - dy;
                if (visited[nx][ny]) continue;

                double len = step_len[dx][dy];
                /* Area contribution of this step: dx * (2*y - dy) / 2 */
                double area = dx * (2.0 * y - dy) / 2.0;
                double w = r * len - area;
                double nd = e.d + w;
                if (nd < dist[nx][ny]) {
                    dist[nx][ny] = nd;
                    pq_push(nd, nx, ny);
                }
            }
        }
    }
    return DBL_MAX;
}

int main(void) {
    init_step_len();
    heap = NULL; heap_size = 0; heap_cap = 0;

    /* Binary search on ratio r.
     * (31250 + A_under) / L >= r
     * <=> A_under - r*L >= -31250
     * <=> -(r*L - A_under) >= -31250
     * <=> shortest path weight <= 31250 (where weight = r*L - A_under)
     *
     * So for ratio r, compute min(r*L - A_under) over all paths.
     * If min <= 31250 (i.e. K), then ratio r is achievable.
     * Actually: dijkstra returns sum of (r*len - area), and we need this <= K = NN*NN/2 = 31250.
     */
    double K = (double)NN * NN / 2.0;

    double lo = 125.0, hi = 140.0;
    for (int iter = 0; iter < 100; iter++) {
        double mid = (lo + hi) / 2.0;
        double w = dijkstra(mid);
        if (w <= K)
            lo = mid;
        else
            hi = mid;
    }

    printf("%.8f\n", lo);

    free(heap);
    return 0;
}

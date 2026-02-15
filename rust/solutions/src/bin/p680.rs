// Project Euler 680 - Yarra Gnisrever
// Implicit treap with lazy reversal for array operations. N=10^18, K=10^6.

const M_VAL: i64 = 1_000_000_000;
const MAX_NODES: usize = 20_000_000;

struct Node { len: i64, first: i64, diff: i32, left: i32, right: i32 }

struct Tree { nodes: Vec<Node>, count: usize }

impl Tree {
    fn new() -> Self {
        let mut nodes = Vec::with_capacity(MAX_NODES);
        for _ in 0..MAX_NODES {
            nodes.push(Node { len: 0, first: 0, diff: 1, left: -1, right: -1 });
        }
        Tree { nodes, count: 0 }
    }
    fn new_leaf(&mut self, len: i64, first: i64, diff: i32) -> i32 {
        let id = self.count; self.count += 1;
        self.nodes[id] = Node { len, first, diff, left: -1, right: -1 };
        id as i32
    }
    fn new_internal(&mut self, left: i32, right: i32) -> i32 {
        let id = self.count; self.count += 1;
        let len = self.nodes[left as usize].len + self.nodes[right as usize].len;
        self.nodes[id] = Node { len, first: 0, diff: 1, left, right };
        id as i32
    }
    fn node_reverse(&mut self, id: i32) {
        let n = &mut self.nodes[id as usize];
        if n.left == -1 { n.first += n.diff as i64 * (n.len - 1); }
        n.diff *= -1;
    }
    fn canonicalize(&mut self, id: i32) {
        let n = &self.nodes[id as usize];
        if n.left != -1 && n.diff == -1 {
            let left = n.left;
            let right = n.right;
            self.node_reverse(right);
            self.node_reverse(left);
            let n = &mut self.nodes[id as usize];
            n.left = right; n.right = left; n.diff = 1;
        }
    }
    fn ensure_cut_at(&mut self, id: i32, index: i64) {
        self.canonicalize(id);
        let n = &self.nodes[id as usize];
        if n.left == -1 {
            let first = n.first; let diff = n.diff; let len = n.len;
            let l = self.new_leaf(index, first, diff);
            let r = self.new_leaf(len - index, first + diff as i64 * index, diff);
            let n = &mut self.nodes[id as usize];
            n.left = l; n.right = r; n.diff = 1;
        } else {
            let left_len = self.nodes[n.left as usize].len;
            if index < left_len {
                let left = self.nodes[id as usize].left;
                self.ensure_cut_at(left, index);
                let ll = self.nodes[left as usize].left;
                let lr = self.nodes[left as usize].right;
                let old_right = self.nodes[id as usize].right;
                let nr = self.new_internal(lr, old_right);
                self.nodes[id as usize].left = ll;
                self.nodes[id as usize].right = nr;
            } else if index > left_len {
                let right = self.nodes[id as usize].right;
                self.ensure_cut_at(right, index - left_len);
                let rl = self.nodes[right as usize].left;
                let rr = self.nodes[right as usize].right;
                let old_left = self.nodes[id as usize].left;
                let nl = self.new_internal(old_left, rl);
                self.nodes[id as usize].left = nl;
                self.nodes[id as usize].right = rr;
            }
        }
    }
    fn do_reverse(&mut self, id: i32, start: i64, end: i64) {
        self.canonicalize(id);
        let n = &self.nodes[id as usize];
        if n.left == -1 {
            let first = n.first; let diff = n.diff; let len = n.len;
            let lp = self.new_leaf(start, first, diff);
            let mp = self.new_leaf(end - start, first + diff as i64 * (end - 1), -diff);
            let rp = self.new_leaf(len - end, first + diff as i64 * end, diff);
            let rc = self.new_internal(mp, rp);
            self.nodes[id as usize].left = lp;
            self.nodes[id as usize].right = rc;
            self.nodes[id as usize].diff = 1;
        } else {
            let left_len = self.nodes[self.nodes[id as usize].left as usize].len;
            if end <= left_len {
                let left = self.nodes[id as usize].left;
                self.do_reverse(left, start, end);
            } else if start >= left_len {
                let right = self.nodes[id as usize].right;
                self.do_reverse(right, start - left_len, end - left_len);
            } else {
                let left = self.nodes[id as usize].left;
                self.ensure_cut_at(left, start);
                let right = self.nodes[id as usize].right;
                let new_left_len = self.nodes[self.nodes[id as usize].left as usize].len;
                self.ensure_cut_at(right, end - new_left_len);
                let left = self.nodes[id as usize].left;
                let right = self.nodes[id as usize].right;
                let lr = self.nodes[left as usize].right;
                let rl = self.nodes[right as usize].left;
                self.node_reverse(lr);
                self.node_reverse(rl);
                self.nodes[left as usize].right = rl;
                self.nodes[right as usize].left = lr;
                let ll = self.nodes[left as usize].left;
                let lr2 = self.nodes[left as usize].right;
                self.nodes[left as usize].len = self.nodes[ll as usize].len + self.nodes[lr2 as usize].len;
                let rl2 = self.nodes[right as usize].left;
                let rr = self.nodes[right as usize].right;
                self.nodes[right as usize].len = self.nodes[rl2 as usize].len + self.nodes[rr as usize].len;
            }
        }
    }

    fn sum_powers_1(n: i64) -> i64 {
        let n = n % (2 * M_VAL);
        (n * (n + 1) / 2) % M_VAL
    }
    fn sum_powers_2(n: i64) -> i64 {
        let n = n % (6 * M_VAL);
        (n * (n + 1) % (6 * M_VAL) * (2 * n + 1) / 6) % M_VAL
    }
    fn compute_r(&mut self, id: i32, start: i64) -> i64 {
        self.canonicalize(id);
        let n = &self.nodes[id as usize];
        if n.len == 0 { return 0; }
        if n.left == -1 {
            let len = n.len; let first = n.first; let diff = n.diff as i64;
            let t1 = ((start % M_VAL) * (first % M_VAL) % M_VAL * (len % M_VAL) % M_VAL + M_VAL) % M_VAL;
            let coef = ((start * diff + first) % M_VAL + M_VAL) % M_VAL;
            let t2 = coef * Self::sum_powers_1(len - 1) % M_VAL;
            let t3 = (diff * Self::sum_powers_2(len - 1) % M_VAL + M_VAL) % M_VAL;
            return (t1 + t2 + t3) % M_VAL;
        }
        let left = self.nodes[id as usize].left;
        let left_len = self.nodes[left as usize].len;
        let right = self.nodes[id as usize].right;
        let lr = self.compute_r(left, start);
        let rr = self.compute_r(right, start + left_len);
        (lr + rr) % M_VAL
    }
}

fn main() {
    let n: i64 = 1_000_000_000_000_000_000;
    let k = 1_000_000;
    let mut f = vec![0i64; 2 * k + 2];
    f[1] = 1; f[2] = 1;
    for i in 3..=2*k { f[i] = (f[i-2] + f[i-1]) % n; }
    let mut tree = Tree::new();
    let root = tree.new_leaf(n, 0, 1);
    for i in 1..=k {
        let s = f[2 * i - 1];
        let t = f[2 * i];
        if s < t { tree.do_reverse(root, s, t + 1); }
        else { tree.do_reverse(root, t, s + 1); }
    }
    let ans = tree.compute_r(root, 0);
    println!("{}", ans);
}

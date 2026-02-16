// Project Euler 680 - Yarra Gnisrever
// Implicit treap with lazy reversal for array operations. N=10^18, K=10^6.
// Optimized: raw pointer node access, i64 where possible, large stack for recursion.

const M_VAL: i64 = 1_000_000_000;
const MAX_NODES: usize = 20_000_000;

#[derive(Clone, Copy)]
#[repr(C)]
struct Node {
    len: i64,
    first: i64,
    diff: i32,
    left: i32,
    right: i32,
    _pad: i32,
}

// Use raw pointer for node array to avoid borrow checker overhead.
// All operations go through the pointer. This is safe because:
// - The array is allocated once and never resized
// - All indices are valid (< count which is < MAX_NODES)
// - Single-threaded access

static mut NODES: *mut Node = std::ptr::null_mut();
static mut NODE_COUNT: usize = 0;
static mut FREE_LIST: *mut Vec<i32> = std::ptr::null_mut();

#[inline(always)]
unsafe fn nd(id: i32) -> *mut Node {
    NODES.add(id as usize)
}

#[inline(always)]
unsafe fn alloc_node() -> i32 {
    let fl = &mut *FREE_LIST;
    if let Some(id) = fl.pop() {
        id
    } else {
        let id = NODE_COUNT;
        NODE_COUNT += 1;
        id as i32
    }
}

#[inline(always)]
unsafe fn free_node(id: i32) {
    (*FREE_LIST).push(id);
}

#[inline(always)]
unsafe fn new_leaf(len: i64, first: i64, diff: i32) -> i32 {
    let id = alloc_node();
    let p = nd(id);
    (*p).len = len;
    (*p).first = first;
    (*p).diff = diff;
    (*p).left = -1;
    (*p).right = -1;
    id
}

#[inline(always)]
unsafe fn new_internal(left: i32, right: i32) -> i32 {
    let id = alloc_node();
    let p = nd(id);
    (*p).len = (*nd(left)).len + (*nd(right)).len;
    (*p).first = 0;
    (*p).diff = 1;
    (*p).left = left;
    (*p).right = right;
    id
}

#[inline(always)]
unsafe fn node_reverse(id: i32) {
    let p = nd(id);
    if (*p).left == -1 {
        (*p).first += (*p).diff as i64 * ((*p).len - 1);
    }
    (*p).diff *= -1;
}

#[inline(always)]
unsafe fn canonicalize(id: i32) {
    let p = nd(id);
    if (*p).left != -1 && (*p).diff == -1 {
        let left = (*p).left;
        let right = (*p).right;
        node_reverse(right);
        node_reverse(left);
        (*p).left = right;
        (*p).right = left;
        (*p).diff = 1;
    }
}

unsafe fn ensure_cut_at(id: i32, index: i64) {
    canonicalize(id);
    let p = nd(id);
    if (*p).left == -1 {
        let first = (*p).first;
        let diff = (*p).diff;
        let len = (*p).len;
        let l = new_leaf(index, first, diff);
        let r = new_leaf(len - index, first + diff as i64 * index, diff);
        // Re-read p since alloc may have invalidated nothing (static array), but be safe
        let p = nd(id);
        (*p).left = l;
        (*p).right = r;
        (*p).diff = 1;
    } else {
        let left = (*p).left;
        let left_len = (*nd(left)).len;
        if index < left_len {
            ensure_cut_at(left, index);
            let ll = (*nd(left)).left;
            let lr = (*nd(left)).right;
            let old_right = (*nd(id)).right;
            let nr = new_internal(lr, old_right);
            free_node(left);
            (*nd(id)).left = ll;
            (*nd(id)).right = nr;
        } else if index > left_len {
            let right = (*nd(id)).right;
            ensure_cut_at(right, index - left_len);
            let rl = (*nd(right)).left;
            let rr = (*nd(right)).right;
            let old_left = (*nd(id)).left;
            let nl = new_internal(old_left, rl);
            free_node(right);
            (*nd(id)).left = nl;
            (*nd(id)).right = rr;
        }
    }
}

unsafe fn do_reverse(id: i32, start: i64, end: i64) {
    canonicalize(id);
    let p = nd(id);
    if (*p).left == -1 {
        let first = (*p).first;
        let diff = (*p).diff;
        let len = (*p).len;
        let lp = new_leaf(start, first, diff);
        let mp = new_leaf(end - start, first + diff as i64 * (end - 1), -diff);
        let rp = new_leaf(len - end, first + diff as i64 * end, diff);
        let rc = new_internal(mp, rp);
        let p = nd(id);
        (*p).left = lp;
        (*p).right = rc;
        (*p).diff = 1;
    } else {
        let left_len = (*nd((*nd(id)).left)).len;
        if end <= left_len {
            let left = (*nd(id)).left;
            do_reverse(left, start, end);
        } else if start >= left_len {
            let right = (*nd(id)).right;
            do_reverse(right, start - left_len, end - left_len);
        } else {
            let left = (*nd(id)).left;
            ensure_cut_at(left, start);
            let right = (*nd(id)).right;
            let new_left_len = (*nd((*nd(id)).left)).len;
            ensure_cut_at(right, end - new_left_len);

            let left = (*nd(id)).left;
            let right = (*nd(id)).right;
            let lr = (*nd(left)).right;
            let rl = (*nd(right)).left;
            node_reverse(lr);
            node_reverse(rl);
            (*nd(left)).right = rl;
            (*nd(right)).left = lr;

            let ll = (*nd(left)).left;
            let lr2 = (*nd(left)).right;
            (*nd(left)).len = (*nd(ll)).len + (*nd(lr2)).len;
            let rl2 = (*nd(right)).left;
            let rr = (*nd(right)).right;
            (*nd(right)).len = (*nd(rl2)).len + (*nd(rr)).len;
        }
    }
}

#[inline(always)]
fn sum_powers_1(n: i64) -> i64 {
    let n = n % (2 * M_VAL);
    (n * (n + 1) / 2) % M_VAL
}

#[inline(always)]
fn sum_powers_2(n: i64) -> i64 {
    let n = (n % (6 * M_VAL)) as i128;
    let m = M_VAL as i128;
    ((n * (n + 1) % (6 * m) * (2 * n + 1) / 6) % m) as i64
}

unsafe fn compute_r(id: i32, start: i64) -> i64 {
    canonicalize(id);
    let p = nd(id);
    if (*p).len == 0 {
        return 0;
    }
    if (*p).left == -1 {
        let m = M_VAL as i128;
        let len = (*p).len as i128;
        let first = (*p).first as i128;
        let diff = (*p).diff as i128;
        let st = start as i128;
        let s1 = sum_powers_1((*p).len - 1) as i128;
        let s2 = sum_powers_2((*p).len - 1) as i128;
        let t1 = (st % m * (first % m) % m * (len % m) % m + m * m) % m;
        let t2 = ((st * diff + first) % m + m) % m * s1 % m;
        let t3 = (diff * s2 % m + m) % m;
        return ((t1 + t2 + t3) % m) as i64;
    }
    let left = (*p).left;
    let left_len = (*nd(left)).len;
    let right = (*p).right;
    let lr = compute_r(left, start);
    let rr = compute_r(right, start + left_len);
    (lr + rr) % M_VAL
}

fn solve() {
    let n: i64 = 1_000_000_000_000_000_000;
    let k = 1_000_000;
    let mut f = vec![0i64; 2 * k + 2];
    f[1] = 1;
    f[2] = 1;
    for i in 3..=2 * k {
        f[i] = (f[i - 2] + f[i - 1]) % n;
    }

    // Allocate node array and free list
    let mut nodes_vec = vec![
        Node { len: 0, first: 0, diff: 1, left: -1, right: -1, _pad: 0 };
        MAX_NODES
    ];
    let mut fl = Vec::<i32>::new();

    unsafe {
        NODES = nodes_vec.as_mut_ptr();
        NODE_COUNT = 0;
        FREE_LIST = &mut fl as *mut Vec<i32>;

        let root = new_leaf(n, 0, 1);
        for i in 1..=k {
            let s = f[2 * i - 1];
            let t = f[2 * i];
            if s < t {
                do_reverse(root, s, t + 1);
            } else {
                do_reverse(root, t, s + 1);
            }
        }
        let ans = compute_r(root, 0);
        println!("{}", ans);
    }
}

fn main() {
    // Need large stack for deep recursion in tree operations
    let builder = std::thread::Builder::new().stack_size(64 * 1024 * 1024);
    let handler = builder.spawn(solve).unwrap();
    handler.join().unwrap();
}

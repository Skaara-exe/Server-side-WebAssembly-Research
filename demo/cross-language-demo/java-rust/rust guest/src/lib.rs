/// Returns a Fibonacci sequence of length `n`.
///
/// - n <= 0  → empty Vec
/// - n == 1  → [1]
/// - n >= 2  → [0, 1, 1, 2, 3, 5, …]  (n elements)
#[no_mangle]
pub extern "C" fn fibonacci(n: i32) -> *mut i32 {
    let seq = fibonacci_inner(n);

    // Encode as [len, v0, v1, …] so the caller knows the length
    let mut out = Vec::with_capacity(seq.len() + 1);
    out.push(seq.len() as i32);
    out.extend_from_slice(&seq);

    let ptr = out.as_mut_ptr();
    std::mem::forget(out); // hand ownership to the caller
    ptr
}

/// Free memory previously returned by `fibonacci`.
#[no_mangle]
pub extern "C" fn free_fibonacci(ptr: *mut i32, n: i32) {
    if ptr.is_null() {
        return;
    }
    let len = (n + 1) as usize; // +1 for the length prefix
    unsafe {
        let _ = Vec::from_raw_parts(ptr, len, len);
    }
}

// ── pure logic ────────────────────────────────────────────────────────────────

fn fibonacci_inner(n: i32) -> Vec<i32> {
    if n <= 0 {
        return vec![];
    }
    if n == 1 {
        return vec![1];
    }

    let mut seq = vec![0i32, 1];
    for i in 2..n as usize {
        let next = seq[i - 1] + seq[i - 2];
        seq.push(next);
    }
    seq
}
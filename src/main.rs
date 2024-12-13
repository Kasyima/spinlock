use std::thread;

use guard::SpinLock;

pub mod guard;

fn main() {
    let x = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| x.lock().push(4));
        s.spawn(|| {
            let mut h = x.lock();
            h.push(3);
            h.push(1);
        });
    });
    let g = x.lock();
    assert!(g.as_slice() == [4, 3, 1] || g.as_slice() == [3, 1, 4]);
}

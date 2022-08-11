use std::sync::mpsc::channel;
use std::thread;
fn linspace(start: f64, stop: f64, num: usize) -> impl Iterator<Item = f64> {
    let interval = (stop - start) / (num - 1) as f64;
    (0..num).map(move |n| interval.mul_add(n as f64, start))
}
fn interval(a: f64, b: f64, n: usize, threads: usize, thread: usize) -> (f64, f64, usize) {
    let a = (b / threads as f64).mul_add(thread as f64, a);
    let b = a + (b / threads as f64);
    let n = n / threads;
    (a, b, n)
}
fn trapz<F>(f: F, start: f64, stop: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let x = linspace(start, stop, n + 1);
    let y: f64 = x.map(&f).sum();
    let y = (y * 2.) - f(start) - f(stop);
    let dx = (stop - start) / n as f64;
    (dx / 2.) * y
}
fn main() {
    let (a, b, n) = (0.0f64, 100_000.0_f64, 1_000_000_usize);
    let threads = 8usize;
    let (tx, rx) = channel();
    for thread in 0..threads {
        let tx = tx.clone();
        thread::spawn(move || {
            let (a, b, n) = interval(a, b, n, threads, thread);
            let t = trapz(
                |x| 4.0f64.mul_add(x, 5.0f64.mul_add(x.powi(3), 3. * x.powi(2))) + 20.,
                a,
                b,
                n,
            );
            tx.send(t).unwrap();
        });
    }
    let res: f64 = (0..threads).map(|_| rx.recv().unwrap()).sum();
    println!("{}", res);
}

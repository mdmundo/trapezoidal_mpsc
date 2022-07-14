use std::sync::mpsc::channel;
use std::thread;

fn linspace(start: f64, stop: f64, num: usize) -> impl Iterator<Item = f64> {
    let interval = (stop - start) / (num - 1) as f64;
    (0..num).map(move |n| start + (interval * n as f64))
}

fn interval(a: f64, b: f64, n: usize, s: usize, r: usize) -> (f64, f64, usize) {
    let a = a + (b / s as f64) * r as f64;
    let b = a + (b / s as f64);
    let n = n / s;
    (a, b, n)
}

fn trapz<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let x = linspace(a, b, n + 1);
    let y: f64 = x.map(&f).sum();
    let y = (y * 2.) - f(a) - f(b);
    let dx = (b - a) / n as f64;
    (dx / 2.) * y
}

fn main() {
    let (a, b, n) = (0.0f64, 1.0f64, 100usize);

    let threads = 8usize;
    // ***
    let (tx, rx) = channel();
    for i in 0..threads {
        let tx = tx.clone();
        thread::spawn(move || {
            let (a, b, n) = interval(a, b, n, threads, i);
            let t = trapz(|x| x * x, a, b, n);
            tx.send(t).unwrap();
        });
    }

    let mut res = 0.0f64;
    for _ in 0..threads {
        res += rx.recv().unwrap();
    }

    println!("{}", res);
}

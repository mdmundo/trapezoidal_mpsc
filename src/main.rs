use std::sync::mpsc::channel;
use std::thread;

fn linspace(start: f64, stop: f64, num: usize) -> impl Iterator<Item = f64> {
    let interval = (stop - start) / (num - 1) as f64;
    (0..num).map(move |n| start + (interval * n as f64))
}

fn trapz<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let x = linspace(a, b, n + 1);
    let y: f64 = x.map(&f).sum();
    let y = (y * 2.) - f(a) - f(b);
    let dx = (b - a) / n as f64;
    let t = (dx / 2.) * y;
    t
}

fn main() {
    let t = trapz(|x| x * x, 0., 1., 100);
    println!("{}", t);

    // ***
    let (tx, rx) = channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }

    for _ in 0..10 {
        let j = rx.recv().unwrap();
        assert!((0..10).contains(&j));
    }
}

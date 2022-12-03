use std::sync::mpsc;
use std::thread;
use std::time::Instant;

//Función para calcular pi de forma paralela
pub fn parallel_calculation(n: usize, thread_count: usize) -> (f64, f32) {
    let now = Instant::now();
    let (tx, rx) = mpsc::channel();

    let h: f64 = 1.0 / n as f64;
    for t in 0..thread_count {
        let tx = tx.clone();

        thread::spawn(move || {
            let init_val: usize = t * 2 + 1;
            let local_pi: f64 =
                (init_val..n)
                    .into_iter()
                    .step_by(thread_count)
                    .fold(0.0, |acc, i| {
                        let x: f64 = h * (i as f64);
                        acc + 4.0 / (1.0 + x * x)
                    });

            tx.send(local_pi).unwrap();
        });
    }
    drop(tx);
    let pi: f64 = rx.iter().sum();
    let new_now = Instant::now();
    let time = new_now.duration_since(now);

    (pi * h, time.as_secs_f32())
}

//Función para calcular pi
pub fn normal_calculation(n: usize) -> (f64, f32) {
    let now = Instant::now();
    let h: f64 = 1.0 / n as f64;

    let mut sum: f64 = 0.0;
    for i in 1..n {
        let x: f64 = h * (i as f64);
        sum += 4.0 / (1.0 + x * x);
    }
    let pi: f64 = sum * h;
    let new_now = Instant::now();
    let time = new_now.duration_since(now);

    (pi, time.as_secs_f32())
}

//Función para calcular pi con la fórmula de Leibniz
pub fn leibniz_normal_calculation(n: usize) -> (f64, f32) {
    let now = Instant::now();

    let init_val: usize = 1;
    let mut sum: f64 = 0.0;
    for i in (init_val..n).step_by(2) {
        if ((i - 1) % 4) == 0 {
            sum += 1.0 / i as f64;
            continue;
        }
        sum -= 1.0 / i as f64;
    }

    let new_now = Instant::now();
    let time = new_now.duration_since(now);
    (sum * 4.0, time.as_secs_f32())
}

//Función para calcular pi de forma paralela con la fórmula de Leibniz
pub fn leibniz_parallel_calculation(n: usize, thread_count: usize) -> (f64, f32) {
    let now = Instant::now();

    let (tx, rx) = mpsc::channel();

    for t in 0..thread_count {
        let tx = tx.clone();

        thread::spawn(move || {
            let init_val: usize = t * 2 + 1;
            let local_pi: f64 =
                (init_val..n)
                    .into_iter()
                    .step_by(2 * thread_count)
                    .fold(0.0, |acc, i| {
                        acc + if ((i - 1) % 4) == 0 {
                            1.0 / i as f64
                        } else {
                            -1.0 / i as f64
                        }
                    });

            tx.send(local_pi).unwrap();
        });
    }

    drop(tx);

    let pi: f64 = rx.iter().sum();
    let new_now = Instant::now();
    let time = new_now.duration_since(now);

    (pi * 4.0, time.as_secs_f32())
}

use std::{io::{self, Write}, f64::consts::PI};
mod pi_calculation;

fn main() {
    let mut n = String::new();
    println!("Ingresa el número de iteraciones a realizar (valor por defecto: 1000000): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: usize = n.trim().parse().unwrap_or(1000000);

    let mut thread_count = String::new();
    println!("Ingresa el número de hilos a utilizar): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut thread_count)
        .expect("failed to read input.");
    let thread_count: usize = thread_count.trim().parse().unwrap_or(1);

    println!("\nValor verdadero de pi: {}\n",PI);
    
    let (p1,t1) = pi_calculation::normal_calculation(n);
    println!("Aproximación realizada con ejecución normal en un tiempo de: {} segundos",t1);
    println!("pi: {}  err: {}\n",p1,(PI-p1).abs());

    let (p2,t2) = pi_calculation::parallel_calculation(n,thread_count);
    println!("Aproximación realizada de forma paralela con un tiempo de: {} segundos",t2);
    println!("pi: {}  err: {}\n",p2,(PI-p2).abs());

    let (p3,t3) = pi_calculation::leibniz_normal_calculation(n);
    println!("Aproximación realizada con ejecución normal usando la fórmula de Leibniz en un tiempo de: {} segundos",t3);
    println!("pi: {}  err: {}\n",p3,(PI-p3).abs());

    let (p4,t4) = pi_calculation::leibniz_parallel_calculation(n,thread_count);
    println!("Aproximación realizada de forma paralela usando la fórmula de Leibniz con un tiempo de: {} segundos",t4);
    println!("pi: {}  err: {}\n",p4,(PI-p4).abs());

}

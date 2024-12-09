use std::time::Instant;

// Native Fibonacci implementation
fn fib_native(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        fib_native(n - 1) + fib_native(n - 2)
    }
}

// Simulated circle drawing (just calculations, no actual drawing)
fn simulate_circle_drawing(count: i32) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    for _ in 0..count {
        // Simulate the same calculations as in WebAssembly version
        let _x = rng.gen::<f64>() * 800.0;
        let _y = rng.gen::<f64>() * 600.0;
        let r = rng.gen::<f64>() * 255.0;
        let g = rng.gen::<f64>() * 255.0;
        let b = rng.gen::<f64>() * 255.0;
        let a = rng.gen::<f64>();
        
        // Create color string to match the same operations
        let _color = format!("rgba({}, {}, {}, {})", r as u8, g as u8, b as u8, a);
    }
}

fn main() {
    println!("Running native performance tests...\n");

    // Test Fibonacci
    println!("Testing Fibonacci(45)...");
    let start = Instant::now();
    let result = fib_native(45);
    let duration = start.elapsed();
    println!("Native Fibonacci(45) = {} completed in: {:?}\n", result, duration);

    // Test circle calculations
    println!("Testing 100,000 circle calculations...");
    let start = Instant::now();
    simulate_circle_drawing(100_000);
    let duration = start.elapsed();
    println!("Native circle calculations completed in: {:?}\n", duration);
}

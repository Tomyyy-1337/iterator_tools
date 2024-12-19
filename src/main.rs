use std::thread::sleep;

use iterator_tools::ParallelPosition;

fn main() {
    let start = std::time::Instant::now();
    let result = ParallelPosition::find(
        0..,
        |x| {
            sleep(std::time::Duration::from_micros(1));
            x == 10_000
        },
        28,
        100,
    );
    assert_eq!(result, Some(10_000));
    println!("Time elapsed: {:?}", start.elapsed());
    println!("{:?}", result);

    let start = std::time::Instant::now();
    let result = (0..).find(|&x| {
        sleep(std::time::Duration::from_micros(1));
        x == 10_000
    });
    
    println!("Time elapsed: {:?}", start.elapsed());
    println!("{:?}", result);

    

}

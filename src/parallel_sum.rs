use std::ops::AddAssign;

pub struct ParallelSum;

impl ParallelSum {
    pub fn sum() {

    }


    /// Sums the elements of an iterator after applying a function to each element in parallel.
    /// 
    /// * `iterator` - The iterator to sum
    /// * `map` - The function to apply to each element
    /// * `num_threads` - The number of threads to use
    /// * `chunk_size` - The number of elements per thread
    /// 
    /// Example:
    /// ```rust
    /// let result = ParallelSum::map_sum(0..=100u64, |x| x, 10, 1000);
    /// assert_eq!(result, 100 * 101 / 2);
    /// ```
    pub fn map_sum<T, O> (
        mut iterator: impl Iterator<Item = T>,
        map: fn(T) -> O,
        num_threads: usize,
        chunk_size: usize,
    ) -> O  
    where
        T: Send + 'static,
        O: Send + 'static + std::iter::Sum + Default + AddAssign,
    {
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut iterator = iterator.by_ref().peekable();

        for _ in 0..num_threads {
            if iterator.peek().is_none() {
                break;
            }
            Self::spawn_thread_from_iter(chunk_size, &mut iterator, &sender, map);
        }
        let mut sum = O::default();
        for part_sum in receiver.iter() {
            sum += part_sum;
            if iterator.peek().is_none() {
                break;
            }
            Self::spawn_thread_from_iter(chunk_size, &mut iterator, &sender, map);
        }
        drop(sender);
        for part_sum in receiver.iter() {
            sum += part_sum;
        }
        sum
    }

    fn spawn_thread_from_iter<T, O>(
        chunk_size: usize,
        iterator: &mut impl Iterator<Item = T>,
        sender: &std::sync::mpsc::Sender<O>,
        map: fn(T) -> O,
    ) 
    where
        T: Send + 'static,
        O: Send + 'static + std::iter::Sum,
    {
        let sender = sender.clone();
        let chunk: Vec<_> = iterator.take(chunk_size).collect();
        std::thread::spawn(move || {
            let sum: O = chunk.into_iter().map(|item| map(item)).sum();
            sender.send(sum).unwrap();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_sum() {
        let result = ParallelSum::map_sum(
            0..=100u64,
            |x| x,
            10,
            1000,
        );
        assert_eq!(result, 100 * 101 / 2);
        let result = ParallelSum::map_sum(
            0..=100u64,
            |x| x,
            10,
            100,
        );
        assert_eq!(result, 100 * 101 / 2);
        let result = ParallelSum::map_sum(
            0..=100u64,
            |x| x,
            10,
            10,
        );
        assert_eq!(result, 100 * 101 / 2);
    }

    #[test]
    fn test_parallel_sum_2() {
        let result = ParallelSum::map_sum(
            0..=100u64,
            |x| x * x,
            10,
            10,
        );
        assert_eq!(result, 100 * 101 * 201 / 6);
    }
}
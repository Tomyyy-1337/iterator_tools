pub struct ParallelPosition;

impl ParallelPosition {
    pub fn find<T> (
        mut iterator: impl Iterator<Item = T>,
        condition: impl Fn(T) -> bool + Send + Copy + 'static,
        num_threads: usize,
        chunk_size: usize,
    ) -> Option<usize> 
    where
        T: Send + 'static,
    {
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut indx = 0;
        let mut iterator = iterator.by_ref().peekable();
    
        for _ in 0..num_threads {
            if iterator.peek().is_none() {
                break;
            }
            Self::spawn_thread_from_iter(chunk_size, &mut iterator, &sender, &mut indx, condition);
        }
    
        let mut min = None;
        for pos in receiver.iter() {
            match pos {
                Some(pos) => {
                    min = Some(pos);
                    break;
                }
                None if iterator.peek().is_some() => Self::spawn_thread_from_iter(chunk_size, &mut iterator, &sender, &mut indx, condition),
                None => break,
            }
        }
        drop(sender);
        for pos in receiver.iter().flatten() {
            match &mut min {
                None => min = Some(pos),
                Some(min_value) if pos < *min_value => *min_value = pos,
                _ => {}
            }
        }
        min
    }
    
    fn spawn_thread_from_iter<T>(
        chunk_size: usize,
        iterator: &mut impl Iterator<Item = T>,
        sender: &std::sync::mpsc::Sender<Option<usize>>,
        indx: &mut usize,
        condition: impl Fn(T) -> bool + Send + 'static,
    ) 
    where
        T: Send + 'static,
    {
        let sender = sender.clone();
        let chunk: Vec<_> = iterator.take(chunk_size).collect();
        let indx_clone = *indx;
        std::thread::spawn(move || {
            let pos = chunk.into_iter().position(condition);
            sender.send(pos.map(|pos_value| indx_clone * chunk_size + pos_value)).unwrap();
        });
        *indx += 1;
    }
}
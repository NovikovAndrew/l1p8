use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Создаем HashMap и оборачиваем его в Mutex
    let map = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    for i in 0..10 {
        let map_clone = Arc::clone(&map);
        let handle = thread::spawn(move || {
            let mut map = map_clone.lock().unwrap();
            map.insert(i, i * 2);
            println!("Inserted {}: {}", i, i * 2);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let map = map.lock().unwrap();
    println!("Final map: {:?}", *map);
}

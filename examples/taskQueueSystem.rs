use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;


 struct Task {
    id: u32,
    perocessing_time: u32,
}

 struct TaskQueue {
    tasks: VecDeque<Task>,
    completed_tasks: Vec<u32>,
    total_processed: u32,
}

impl TaskQueue {
    fn new() -> Self {
        TaskQueue {
            tasks: VecDeque::new(),
            completed_tasks: Vec::new(),
            total_processed: 0,
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push_back(task);
    }

    fn get_next_task(&mut self) -> Option<Task> {
        self.tasks.pop_front()
    }

    fn complete_task(&mut self, task_id: u32) {
        self.completed_tasks.push(task_id);
        self.total_processed += 1;
    }

    fn remaining_tasks(&self) -> usize {
        self.tasks.len()
    }

    fn get_stats(&self) -> (usize, usize, u32) {
        (
            self.tasks.len(),
            self.completed_tasks.len(),
            self.total_processed,
        )
    }
}

fn main () {
    let queue = Arc::new(Mutex::new(TaskQueue::new()));

    {
        let mut queue = queue.lock().unwrap();
        for i in 1..=10 {
            queue.add_task(Task {
                id:i,
                processing_time: i * 100,
            });
        }
    }

    let producer_queue = Arc::clone(&queue);

    let producer = thread::spawn(move || {
        for i in  11..=15 {
            thread::sleep(Duration::from_millis(500));

            if let Ok(mut queue) = producer_queue.lock() {
                queue.add_task(Task {
                    id: i,
                    processing_time: i*50,
                });
                println!("Producer: Added task {}", i);
            }
        }
    });


    let mut consumers = vec![];
    for consumer_id in 1..=3 {
        let consumer_queue = Arc::clone(&queue);

        let consumer = thread::spawn(move || {
            loop {
                if let Ok(mut queue) = consumer_queue.lock() {
                    if let Some(task) = queue.get_next_task() {
                        let task_id = task.id;
                        let processing_time = task.processing_time;
                        println!("Consumer {}: Processing task {} ({}ms)",
                                consumer_id, task_id, processing_time);

                        drop(queue);

                        thread::sleep(Duration::from_millis(processing_time as u64));


                        if let Ok(mut queue) = consumer_queue.lock() {
                            queue.complete_task(task_id);
                            println!("Consumer {}: Completed task {}", consumer_id, task_id);

                            if queue.remaining_tasks() == 0 {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        });
        consumers.push(consumer);
    }

    producer.join().unwrap();

    for consumer in consumers {
        consumer.join().unwrap();
    }

    if let Ok(queue) = queue.lock() {
        let (remaining, completed, total) = queue.get_stats();
        println!("\nFinal Statistics:");
        println!("Remaining tasks: {}", remaining);
        println!("completed tasks: {}", completed);
        println!("Total processed: {}", total);
    }
}


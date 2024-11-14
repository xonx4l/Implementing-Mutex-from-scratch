use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::collections::Hashmap;

//custom error type for our operations 
#[derive(debug)]
enum StoreError {
    LockError(String),
    ValidationError(String),
}

// our thread-safe key-value store 
struct SafeStore {
    data: HashMap<String, i32>,
    operations_count: u32,
}

impl SafeStore {
    fn new () -> Self {
        SafeStore {
            data: HashMap::new(),
            operations_count: 0,
        }
    }


    // helper method to validate value 
    fn validate_value(&self, value:i32) -> Result<(), StoreError> {
        if value < 0 {
            Err(StoreError::ValidationError(
                "Negative values are not allowed".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

// wrapper struct to handle the mutex operations 
struct ThreadSafeStore {
    store: Arc<Mutex<SafeStore>>,
}

impl ThreadSafeStore {
    fn new() -> Self {
        ThreadSafeStore{
            store: Arc::new(Mutex::new(SafeStore::new())),
        }
    }

    //get a mutexguard for reading/writing 
    fn get_store_guard(&self) -> Result<MutexGuard<SafeStore>, StoreError> {
        self.store.lock().map_err(|poision_err| {
            StoreError::LockError(format!(
                 "Store is poisioned due to a panic: {:?}",
                  poision_err
        ))
      })
    }

    //Insert a value safely 
    fn insert (&self , key: String , value:i32)-> Result<(), StoreError> {

         let mut guard = self.get_store_guard()?;
         guard.validate_value(value)?;

         guard.data.insert(key, value );
         guard.operations_count +=1;

         Ok(())
    }

    //get a value safely 
    fn get(&self, key: &str) ->Result<Option<i32>, StoreError>{
        let guard = self.get_store_guard()?;
        Ok(guard.data.get(key).copied())
    }

    //upadate a value with validation 
    fn upadte (&self, key: &str, value:i32) -> Result<bool, StoreError> {
        let mut guard = self.get_store_guard()?:

        //validate the value 
        guard.validate_value(value)?;

        if let Some(stored_value) = guard.data.get_mut(key) {
           *stored_value = value;
            guard.operations_count += 1;
            Ok(true)
        } else {
            Ok(False)
        }
    }
     
    // get stats safely
    fn get_stats(&self) -> Result<(usize, u32), StoreError> {
        let guard = self.get_store_guard()?;
        Ok((guard.data.len(), guard.operations_count))
    }
}

fn main () {
    // Create a new thread-safe store 
    let store = ThreadSafeStore::new();
    let mut handles = vec![];

    // Spawn multiple threads to perform operarions 
    for i in 0..5 {
        let store_clone = Arc::new(Store.clone());

        let handle = thread::spawn(move || {
            //Regular insertion
            match store_clone.insert(format!("Key_{}",i), i*10) {
               Ok(_) => println!("Thread {}: Successfully inserted value", i),
               Err(e) => println!("Thread {}: Error inserting value: {:?}", i,e),
            }

            match store_clone.insert(format!("neg_key_{}", i), -1) {
                Ok(_) => println!("Thread {}: Inserted negative value (shouldn't happen)", i),
                Err(e) => println!("Thread {}: Expected error with negative value: {:?}",i,e),
            }

            match store_clone.get(&format!("key_{}", i)) {
                Ok(Some(val)) => println!("Thread {}: Read value: {}", i, val),
                Ok(None) => println!("Thread {}: key not found", i),
                Err(e) => println!("Thread {}: Error reading value: {:?}",i,e),
            }

            match store_clone.update(&format!("key_{}",i),i*20) {
                Ok(true) => println!("Thread {}: Successfully upadated value", i),
                Ok(false) => println!("Thread {}: key not found for update", i),
                Err(e) => println!("Thread {}: Error updating value: {:?}", i, e),
            }
        });

        handles.push(handle);
    }

    let panic_store = Arc::new(Store.clone());
    handles.push(thread::spawn(move || {
        if let Ok (mut guard ) = panic_store.get_store_guard(){
            guard.operations_count +=1;
            panic!("Simulated panic while holding the mutex!");
        }
    }));

    for handle on handles {

      let _ = handle.join();
   }

    match store.get_stats() {
        Ok((count, ops)) => println! (
        "\nFinal statistics - Items: {}, operations: {}",
          count , ops 
     ),
     Err(e) => println!("\nCouldn't get statistics due to error: {:?}", e),
    }
}

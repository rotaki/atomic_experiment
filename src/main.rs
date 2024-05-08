use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use std::{mem, thread};

fn test1() {
    // Define a byte array
    let mut data: [u8; 4] = [0; 4];

    // Ensure the array is aligned properly for AtomicI32
    let data_ptr = data.as_mut_ptr() as *mut AtomicI32;

    unsafe {
        // Check alignment
        assert_eq!(
            data_ptr.align_offset(mem::align_of::<AtomicI32>()),
            0,
            "data is not properly aligned"
        );

        // Perform an atomic store
        (*data_ptr).store(123456, Ordering::Release);
    }

    // For demonstration: print the stored value by copying it back to an i32
    let stored_val: i32;
    unsafe {
        stored_val = *(data_ptr as *const i32);
    }
    println!("Stored value: {}", stored_val);
}

pub struct Page {
    data: UnsafeCell<[u8; 40]>, // 10 * 32-bit integers
}

impl Page {
    pub fn data(&self) -> &mut [u8; 40] {
        unsafe { &mut *self.data.get() }
    }
}

unsafe impl Sync for Page {}

fn test2() {
    let page = Page {
        data: UnsafeCell::new([0; 40]),
    };

    // Prepare 10 threads. Each thread will increment the 10 integers in the page by 100,000 times.
    // Scoped threads are used to ensure the threads are joined before the function returns.
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                for _ in 0..100_000 {
                    for i in 0..10 {
                        let offset = i * 4;
                        let data = page.data();
                        let data_ptr = data[offset..offset + 4].as_mut_ptr() as *mut AtomicU32;
                        unsafe {
                            (*data_ptr).fetch_add(1, Ordering::AcqRel);
                        }
                    }
                }
            });
        }
    });

    // Print the final values of the 10 integers
    let data = page.data();
    for i in 0..10 {
        let offset = i * 4;
        let data_ptr = data[offset..offset + 4].as_ptr() as *const AtomicU32;
        let value = unsafe { (*data_ptr).load(Ordering::Acquire) };
        println!("Value {}: {}", i, value); // num_threads * num_iterations = 10 * 100,000 = 1,000,000
    }
}

fn main() {
    test2();
}

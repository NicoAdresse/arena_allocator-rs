#![no_std]
#![no_main]

use core::alloc::Layout;
use core::ptr;
use core::panic::PanicInfo;

struct ArenaAllocator {
    buffer: *mut u8,
    capacity: usize,
    offset: usize,
}

impl ArenaAllocator {
    pub fn new(backing_buffer: &mut [u8]) -> Self {
        Self {
            buffer: backing_buffer.as_mut_ptr(),
            capacity: backing_buffer.len(),
            offset: 0
        }
    }

    pub fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let alignment = layout.align();

        let current_ptr = unsafe {
            self.buffer.add(self.offset) as usize
        };
        let aligned_ptr = (current_ptr + (alignment - 1)) & !(alignment - 1);
        let padding = aligned_ptr - current_ptr;

        if size > self.capacity || self.offset > self.capacity - size - padding {
            return ptr::null_mut();
        } 

        self.offset += padding + size;
        aligned_ptr as *mut u8
    }

    pub fn alloc_zeroed(&mut self, layout: Layout) -> *mut u8 {
        let ptr = self.alloc(layout);
        if !ptr.is_null() {
            unsafe {
                ptr::write_bytes(ptr, 0, layout.size());
            }
        }
        ptr
    }

    pub fn create<T>(&mut self, value: T) -> Option<&mut T> {
        let layout = Layout::new::<T>();
        let raw_mem = self.alloc(layout);

        if !raw_mem.is_null() {
            unsafe {
                let typed_ptr = raw_mem as *mut T;
                ptr::write(typed_ptr, value);
                Some(&mut *typed_ptr)
            }
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.offset = 0;
    } 
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut buffer: [u8; 15000] = [0; 15000];
    let mut allocator =  ArenaAllocator::new(&mut buffer);

    let _my_number = allocator.create::<i32>(42);

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
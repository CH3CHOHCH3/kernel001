// Physical memory manager in early stage.

#![allow(dead_code)]

use crate::config::PAGE_SIZE;
use crate::config::PHSY_MEM_TOP;

extern "C" {
    // defined in kernel.ld
    fn ekernel();
}

struct List {
    next: *mut List,
}

struct ListBox {
    head: *mut List,
}

unsafe impl Sync for ListBox {}

// start of all free pages
static mut FREE_PAGES: ListBox = ListBox{head: core::ptr::null_mut()};

// free the page at physicall address pa
// and add it to the FREE_PAGES list.
pub fn bootmem_free(pa: usize) {
    if pa % PAGE_SIZE != 0 || pa < ekernel as usize || pa >= PHSY_MEM_TOP {
        panic!("invalid physical address.");
    }
    
    (pa..(pa + PAGE_SIZE)).for_each(|p| {
        unsafe {
            (p as *mut u8).write_volatile(1)
        }
    });
    
    let free_page = pa as *mut List;
    unsafe {
        (*free_page).next = FREE_PAGES.head;
        FREE_PAGES.head = free_page;        
    }
}

// alloc a free page from FREE_PAGES list.
pub fn bootmem_alloc() -> usize {
    unsafe{
        match FREE_PAGES.head.is_null() {
            false => {
                let free_page = FREE_PAGES.head;
                FREE_PAGES.head = (*free_page).next;
                free_page as usize
            },
            true => 0
        }
    }
}

// add all free pages to FREE_PAGES list.
// (from the end of kernel to the top of RAM)
pub fn bootmem_init() {
    for pa in (ekernel as usize..PHSY_MEM_TOP).step_by(PAGE_SIZE) {
        bootmem_free(pa);
    }
}
#[warn(unused_assignments)]

fn main() {
    let a: i64 = 42;
    let b: i64 = 80;

    let mut a_ptr = a as *mut i64;

    let b_ptr = &a as *const i64;

    let _addr: usize = unsafe {
        std::mem::transmute(b_ptr)
    };

    println!("{}, {:p}", a, a_ptr);

    println!("a: {}, ({:p}..0x{:x})", a, b_ptr, _addr + 7);

    unsafe {  // something you shouldn't do unless the situation is dire
        // *a_ptr = 88_i64;  // can't be done, complaining a_ptr is *const
        // a_ptr = b as *mut i64;  // doable, if a_ptr is mutable
                                   // it means that anology of 'const int*' and 'int *const' from
                                   // C, doesn't fit perfectly here, I think (it's opposite here)
        *a_ptr = 88_i64;
        a_ptr = b as *mut i64;  // a_ptr needs to be mutable for this work
    }
}


/* block of C code showing const behavior with pointer
int a = 10;
int c = 20;
const int* b = &a;
int* const d = &c;
b = &c;
// *b = 30;  // not doable, because b is 'pointer to constant int'
// d = &a;  // not doable, because d is 'const pointer to int'
*d = a; */

use std::mem::size_of;

static B: [u8; 10] = [99, 98, 97, 114, 114, 121, 116, 111, 115, 120];
static C: [u8; 11] = [99, 94, 107, 115, 114, 111, 16, 111, 150, 20, 0];

fn main() {
    let a: usize = 42;  // usize is the memory address size for the cpu the code is compiled for

    let b: &[u8; 10] = &B;

    // Box<[u8]> is 'boxed byte slice'. Ownership of value placed inside box moves to the owner
    // of the box
    let c: Box<[u8]> = Box::new(C);

    println!("a (an unsigned integer):");
    println!("  location:   {:p}", &a);
    println!("  size:       {:?} bytes", size_of::<usize>());
    println!("  value:      {:?} bytes", a);

    println!("b (a reference to B):");
    println!("  location:   {:p}", &b);
    println!("  size:       {:?} bytes", size_of::<&[u8; 10]>());
    println!("  pointing:   {:p} bytes", b);  // same address as B
    println!("  value:      {:?} bytes", b);

    println!("c (a \"box\" for C):");
    println!("  location:   {:p}", &c);
    println!("  size:       {:?} bytes", size_of::<Box<[u8]>>());
    println!("  pointing:   {:p} bytes", c);  // not the same address as &C, maybe due to Box
    println!("  value:      {:?} bytes", c);

    println!("B (an array of 10 bytes):");
    println!("  location:   {:p}", &B);
    println!("  size:       {:?} bytes", size_of::<[u8; 10]>());
    println!("  value:      {:?} bytes", B);

    println!("C (an array of 11 bytes):");
    println!("  location:   {:p}", &C);
    println!("  size:       {:?} bytes", size_of::<[u8; 11]>());
    println!("  value:      {:?} bytes", C);
}

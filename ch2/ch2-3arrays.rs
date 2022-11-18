fn main() {
    let one = [1, 2, 3];
    let two: [u16; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u16; 3] = [0; 3];  // can't make on u8 and other one u16, because below arrays
                                    // must have same type for arrays

    let arrays = [one, two, blank1, blank2];

    for a in arrays {
        print!("{:?}", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}

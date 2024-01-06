use std::mem; 
fn main() {
    let a:u8 = 128;     // u - unsigned,    8-8bit -> 0-255,    its immutable means we cannot overring new number
                                        // 0 ..... 2^(N-1)
    println!("a = {}",a);
    // a = 54;      --we cannot assign new values it will throw error

    let mut b:i8 = 23;              // we have defined mut keyword before variable so it will going to override
                            //i - signed,   -2^(N-1) .... 2^(N-1)
    println!("Before b = {}",b);
    b = 44;
    println!("After b = {}",b);

    let mut c = 123456789;          // bydefault it takes i32 data size
    print!("c = {} and it takes size in bytes = {}",c,mem::size_of_val(&c));    //i32 --> 32bits --> 32/8 = 4bytes
}

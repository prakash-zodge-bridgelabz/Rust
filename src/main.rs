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
    println!("c = {} and it takes size in bytes = {}",c,mem::size_of_val(&c));    //i32 --> 32bits --> 32/8 = 4bytes

    let c = -123;
    println!("c = {}",c);
    //usize and isize
    let z:isize = 123;
    let size_of_z:usize = mem::size_of_val(&z);
    println!("z = {}, size of z = {}, {}bit OS",z,size_of_z,size_of_z*8);

    let x:char = 'x';
    println!("{} is a character, {} is size of character & its using {}bit OS",x,mem::size_of_val(&x),mem::size_of_val(&x)*8);

    let res:bool = false;
    println!("{}, size = {}byte, {}bit OS",res,mem::size_of_val(&res),mem::size_of_val(&res)*8);
}

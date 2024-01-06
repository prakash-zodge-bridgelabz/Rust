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
}

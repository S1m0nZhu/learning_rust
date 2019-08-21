use std::mem;

fn scope_and_shadowing()
{
    let a = 123;
    let a = 777;
    {
        let b = 456;
        println!("inside,b ={}", b);
        println!("inside,a ={}", a);
    }
    println!("outside, a = {}", a);
}
fn main() {
    //unsigned 0..255
    let a:u8 = 123;//8bits
    println!("a = {}",a)

    a = 456;

    //mut
    let mut b:i8 = 0;
    println!("b = {},",b);
    b = 42;
    println!("b = {},",b);

    let mut c = 123456789;
    println!("c = {},size = {} bytes",c,mem::size_of_val(&c));
    c = -1;
    println!("c = {},after modification",c);

    //i8 u8 i16 u16 i32 u32 i64 u64
    let z:isize = 123;// isize/usize
    let size_of_z =mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit os",z,size_of_z,size_of_z * 8);

    let d = 'x';
    println!("d={},size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; //double-precision, 8 bytes or 64bits, f64
    println!("e={},size = {} bytes", d, mem::size_of_val(&e));

    //true false
    let g = false;
    println!("g={},size = {} bytes", g, mem::size_of_val(&g));

    let f = 4>0;//true
}
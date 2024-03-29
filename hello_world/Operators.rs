use std::mem;

fn operators()
{
    //arithmetic

    let mut a =2+3*4;
    println!("{}",a);
    a = a+1;// -- ++
    a -= 2;// a = a - 2
    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let b = 2.5
    let a_cubed = f64::powi(b,3);
    let b_to_pi = f64::powf(b,std::f64::consts::PI);
    println!("{} cubed = {},{}^pi = {}",b,b_cubed,b,b_to_pi);

    //bitwise
    let c = 1 | 2; // | OR & AND ^XOR ! NOR
                        // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10; // >>
    println!("2^10 = {}", two_to_10);

    //logical
    let pi_less_4 =std::f64::consts::PI < 4.0; // true
    // > <= >= ==
    let x = 5;
    let x_is_5 = x == 5; //true
}
fn while_and_loop()
{
    let mut x = 1;
    while x < 1000
        {
            x *= 2;
            if x == 64 { continue; }
            println!("x={}", x);
        }
    let mut y = 1;
    loop
        {
        y *= 2;
            println!("y={}", y);
            if y == 1 << 10 { break; }
    }
}

fn for_loop()
{
    for x in 1..11
        {
            if x == 3 { continue; }
            if x == 8 { break; }
             println!("x={}", x);
        }
    for (pos,y) in (30..41).enumerate()
        {
            println!("{}:{}", pos, y)
        }
}

fn main() {
    for_loop();
    //while_and_loop();
}
fn rectangle(width: i32, height: i32) {
    for _i in 0..width {
        for _j in 0..height {
            print!("* ");
        }
        println!();
    }
}

fn hollow_rectangle(width: i32, height: i32) {
    for i in 0..width {
        for j in 0..height {
            if i == 0 || i == width - 1 || j == 0 || j == height - 1 {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!()
    }
}

fn diamond(height: i32) {
    let mid_height: i32 = height / 2;
    for i in 0..height {
        if i <= mid_height {
            print!("{}", "  ".repeat((mid_height - i) as usize));
            println!("{}", "* ".repeat((2 * i + 1) as usize));
        } else {
            print!("{}", "  ".repeat((i - mid_height) as usize));
            println!("{}", "* ".repeat(((height - i) * 2 - 1) as usize));
        }
    }
}
fn main() {
    println!("Pattern Printing in Rust");
    println!("Solid Rectangle Pattern : ");
    rectangle(5, 10);
    println!("\nHollow Rectangle Pattern : ");
    hollow_rectangle(5, 10);
    println!("\nDiamond : ");
    diamond(5);
}

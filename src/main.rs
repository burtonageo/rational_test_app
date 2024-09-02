use rational::{self, Rational};

fn main() {
    println!("is dylib: {}", rational::is_dynamically_linked());
    println!("version: {:?}", rational::version());

    let rat1 = Rational::new(41, 64);
    let mut rat2 = Rational::new(22, 128);

    rational::normalize(&mut rat2);
    println!("rat2 = {:?}", rat2);

    let mut result = rational::add(&rat1, &rat2);
    rational::normalize(&mut result);

    println!("{:?} + {:?} = {:?}", rat1, rat2, result);
}


pub fn hello_world() {
    println!("Hello world!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        hello_world();
    }
}

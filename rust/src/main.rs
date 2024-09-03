fn main() {
    println!("Hello World!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn passing_test() {
        assert_eq!(1, 1)
    }
}

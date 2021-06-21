#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn another() {
        panic!("test fails");
    }
}





fn add(a:i8,b:i8) -> i8 {
    a + b
}
fn main() {

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(1, 3),4);
    }


}


pub fn sum(x: i32, y: i32) -> i32{
    let s = x + y;
    s
}

#[cfg(test)]
mod tests {
    use sum;
    #[test]
    fn test() {assert_eq!(3, sum(2,1));}
}
fn main() {
    println!("Hello, rust!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_example() {
        // 测试代码
        assert_eq!(2 + 2, 4);
    }
}

pub mod sqrt;
pub mod log;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let test1 = sqrt::sqrt(9.0);
        assert_eq!(test1, 3.0); //Square Root Of 9 Is 3
        let test2 = log::log2n(8.0);
        assert_eq!(test2, 3.0); //Log Of 8 Base 2 Is 3
        let test3 = log::log(100.0, 10.0);
        assert_eq!(test3, 2.0); //Log Of 100 Base 10 Is 2
    }
}
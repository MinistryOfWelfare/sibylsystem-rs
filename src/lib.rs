mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[allow(non_snake_case)]
pub mod PsychoPass {
    #[derive(Debug)]
    pub struct PsychoPass {
        token: String,
        host: String
    }

    impl PsychoPass {
        pub fn new(mut host: String, token: String) -> PsychoPass{
            if !host.ends_with("/") {
                host = host + "/"
            }
            PsychoPass {token, host}
        }
    }
}
mod types;

#[allow(non_snake_case)]
pub mod PsychoPass {
    use reqwest::header::{HeaderMap, HeaderValue};
    use crate::types::{BanResult, GetAllBansResult, StatsResult};

    #[derive(Debug)]
    pub struct PsychoPass {
        pub token: String,
        pub host: String,
        pub client: reqwest::blocking::Client,
    }
    #[test]
    pub fn test_stuff() {
        let ins = PsychoPass::new("https://psychopass.animekaizoku.com".parse().unwrap(), String::from("token here"));
        println!("{:?}", ins.check_info(185416795));
        println!("{:?}", ins.get_stats());
        println!("{:?}", ins.get_all_bans());
    }

    impl PsychoPass {
        pub fn new(mut host: String, token: String) -> PsychoPass {
            if !host.ends_with("/") {
                host = host + "/"
            }
            let client = reqwest::blocking::Client::new();
            PsychoPass { token, host, client }
        }
        pub fn check_info(&self, user_id: i64) -> reqwest::Result<BanResult> {
            let mut headers = HeaderMap::new();
            headers.insert("token", self.token.parse().unwrap());
            headers.insert("userid", HeaderValue::from(user_id));
            let res = self.client.get(format!("{}getInfo", self.host))
                .headers(headers)
                .send()?
                .json::<BanResult>();
            res
        }
        pub fn get_stats(&self) -> reqwest::Result<StatsResult> {
            let mut headers = HeaderMap::new();
            headers.insert("token", self.token.parse().unwrap());
            let res = self.client.get(format!("{}getStats", self.host))
                .headers(headers)
                .send()?
                .json::<StatsResult>();
            res
        }
        pub fn get_all_bans(&self) -> reqwest::Result<GetAllBansResult> {
            let mut headers = HeaderMap::new();
            headers.insert("token", self.token.parse().unwrap());
            let res = self.client.get(format!("{}getAllBans", self.host))
                .headers(headers)
                .send()?
                .json::<GetAllBansResult>();
            res
        }

    }
}
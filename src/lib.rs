mod types;

#[allow(non_snake_case)]
pub mod PsychoPass {
    use reqwest::header::{HeaderMap, HeaderValue};
    use std::fs;
    use crate::types::{BanResult, GetAllBansResult, GetGeneralInfoResult, ReportResult, StatsResult};

    #[derive(Debug)]
    pub struct PsychoPass {
        pub token: String,
        pub host: String,
        pub client: reqwest::blocking::Client,
    }
    #[test]
    pub fn test_stuff() {
        let ins = PsychoPass::new("https://psychopass.animekaizoku.com".parse().unwrap(), fs::read_to_string("sibyl.token").unwrap());
        println!("{:?}", ins.check_info(185416795));
        println!("{:?}", ins.get_stats());
        // println!("{:?}", ins.get_all_bans());
        // println!("{:?}", ins.report_user(69, "test".parse().unwrap(), "test".parse().unwrap(), "test".parse().unwrap(), "test", true));
        println!("{:?}", ins.get_general_info(895373440))
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
        pub fn report_user(&self, user_id: i64, reason: String, message: String, source: String,
                           source_group: &str, is_bot: bool) -> reqwest::Result<ReportResult> {
            let mut headers = HeaderMap::new();
            headers.insert("token", self.token.parse().unwrap());
            headers.insert("user-id", HeaderValue::from(user_id));
            headers.insert("reason", reason.parse().unwrap());
            headers.insert("source-group", source_group.parse().unwrap());
            headers.insert("message", message.parse().unwrap());
            headers.insert("source", source.parse().unwrap());
            headers.insert("bot", format!("{}", is_bot).parse().unwrap());
            let res = self.client.get(format!("{}report", self.host))
                .headers(headers)
                .send()?
                .json::<ReportResult>();
            res
        }

        pub fn get_general_info(&self, user_id: i64) -> reqwest::Result<GetGeneralInfoResult> {
            let mut headers = HeaderMap::new();
            headers.insert("token", self.token.parse().unwrap());
            headers.insert("userid", HeaderValue::from(user_id));
            let res = self.client.get(format!("{}getGeneralInfo", self.host))
                .headers(headers)
                .send()?
                .json::<GetGeneralInfoResult>();
            res
        }

    }
}
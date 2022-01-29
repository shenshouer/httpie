use anyhow::{anyhow, Error, Result};
use reqwest::Url;
use std::str::FromStr;

pub(crate) fn parse_url(s: &str) -> Result<String> {
    // 这里仅仅只检查url的合法性
    let _url: Url = s.parse()?;
    Ok(s.into())
}

pub(crate) fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, PartialEq)]
pub(crate) struct KvPair {
    pub k: String,
    pub v: String,
}

impl FromStr for KvPair {
    type Err = Error;

    fn from_str(kv: &str) -> Result<Self, Self::Err> {
        let mut split = kv.split("=");
        let err = || anyhow!(format!("解析{}错误", kv));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }
    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                k: "a".into(),
                v: "1".into()
            }
        );
        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair {
                k: "b".into(),
                v: "".into()
            }
        );
    }
}

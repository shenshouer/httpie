use super::{print::print_response, utils::parse_url};
/// get 子命令
use anyhow::Result;
use clap::Parser;
use reqwest::Client;

/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
pub struct Get {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str=parse_url))]
    url: String,
}

impl Get {
    // 执行命令操作
    pub async fn exec(&self, client: Client) -> Result<()> {
        get(client, self).await
    }
}

/// 处理get命令
async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_response(resp).await?)
}

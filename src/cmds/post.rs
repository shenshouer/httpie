// post 子命令。需要输入一个 URL，和若干个可选的 key=value，用于提供 json body
use anyhow::Result;
use clap::Parser;
use reqwest::Client;
use std::collections::HashMap;

use super::{
    print::print_response,
    utils::{parse_kv_pair, parse_url, KvPair},
};

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Parser, Debug)]
pub(super) struct Post {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str=parse_url))]
    url: String,
    /// HTTP 请求的 body
    #[clap(parse(try_from_str=parse_kv_pair))]
    body: Vec<KvPair>,
}

impl Post {
    pub async fn exec(&self, client: Client) -> Result<()> {
        post(client, self).await
    }
}

/// 处理post命令
async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;

    Ok(print_response(resp).await?)
}

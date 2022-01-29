use anyhow::Result;
use clap::Parser;
use reqwest::{header, Client};

mod get;
mod post;
mod print;
mod utils;

use get::Get;
use post::Post;

/// 人性化命令行http请求工具
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Sope Shen <shenshouer51@gmail.com>")]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Opts {
    pub async fn exec(&self) -> Result<()> {
        let mut headers = header::HeaderMap::new(); // 为我们的 HTTP 客户端添加一些缺省的 HTTP 头
        headers.insert("X-POWERED-BY", "Rust".parse()?);
        headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        self.subcmd.exec(client).await
    }
}

// 子命令分别对应不同的 HTTP 方法，目前只支持 get / post
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
    // 我们暂且不支持其它 HTTP 方法
}

impl SubCommand {
    async fn exec(&self, client: Client) -> Result<()> {
        use SubCommand::*;
        match self {
            Get(ref args) => args.exec(client).await,
            Post(ref args) => args.exec(client).await,
        }
    }
}

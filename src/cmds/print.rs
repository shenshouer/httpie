use anyhow::Result;
use colored::*;
use mime::Mime;
use reqwest::{header, Response};

/// 打印整个响应体
pub(crate) async fn print_response(resp: Response) -> Result<()> {
    print_status(&resp);
    print_header(&resp);
    let m = get_context_type(&resp);
    let body = resp.text().await?;
    print_body(m, &body);
    Ok(())
}

/// 打印服务器返回的HTTP body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan());
        }
        // Some(v) if v == mime::TEXT_HTML => {
        //     use syntect::{
        //         highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet,
        //     };
        //     let ss = SyntaxSet::load_defaults_nonewlines();
        //     // let syntax = ss.find_syntax_plain_text();
        //     let syntax = ss.find_syntax_by_name("HTML").unwrap();
        //     let ts = ThemeSet::load_defaults();
        //     println!(
        //         "{}",
        //         highlighted_html_for_string(body, &ss, syntax, &ts.themes["base16-ocean.dark"])
        //     )
        // }
        _ => println!("{}", body),
    }
}

/// 获取相应类型
fn get_context_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

/// 打印服务器版本号 + 状态码
fn print_status(resp: &Response) {
    let status = format!("{:?}: {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

/// 打印服务器返回的HTTP Header
fn print_header(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!("\n");
}

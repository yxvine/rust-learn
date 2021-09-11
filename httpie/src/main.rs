use anyhow::{anyhow, Result};
use clap::{AppSettings, Clap};
use colored::*;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};

// define the cli entry point

/// A native httpie impl with rust, can you image how easy it is
#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "yyxx")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
    // other http method
}

// get sub cmd
/// feed get with an url and retrieve the resp
#[derive(Clap, Debug)]
struct Get {
    // http request url
    url: String,
}

fn parse_url(url: &str) -> Result<String> {
    // check url validity?
    let _url: Url = url.parse()?;
    Ok(url.into())
}

// post sub cmd, url key=val json body
/// feed post with an url and optional key=val pairs,
/// will post body as Json, and retrieve the resp
#[derive(Clap, Debug)]
struct Post {
    #[clap(parse(try_from_str = parse_url))]
    url: String,

    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}
/// key=val -> KvPair by parse_kv_pair
#[derive(Debug, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

/// impl FormStr trait, str.parse() -> KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to pars {}", s));

        Ok(Self {
            // first is key, return T/None, trans Ok(T)/Err(E) and use ?
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

/// KvPair impl FormStr, can use s.parse()
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    // print!("{:?}", resp.text().await?);
    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    // print!("{:?}", resp.text().await?);
    Ok(print_resp(resp).await?)
}

/// print version and status_code
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

fn print_headers(resp: &Response) {
    for (name, val) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), val);
    }
    print!("\n");
}

fn print_body(m: Option<Mime>, body: &String) {
    match m {
        //json for pretty print
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan());
        }
        //other print the body direct
        _ => println!("{}", body),
    }
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime_type = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime_type, &body);
    return Ok(());
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    return resp
        .headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap());
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    // add default http header
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
    // create a http client
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}

// cargo test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_work() {
        assert!(parse_url("url").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_work() {
        assert!(parse_kv_pair("s").is_err());
        assert_eq!(
            parse_kv_pair("k=1").unwrap(),
            KvPair {
                k: "k".into(),
                v: "1".into()
            }
        );

        assert_eq!(
            parse_kv_pair("k=").unwrap(),
            KvPair {
                k: "k".into(),
                v: "".into()
            }
        );
    }
}

#![doc = include_str!("../README.md")]

use fulbo;

fn main() {
    fulbo::run();
}

// use error_chain::error_chain;
//
// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }
//
// #[tokio::main]
// async fn main() -> Result<()> {
//     let mut headers = reqwest::header::HeaderMap::new();
//     headers.insert(
//         reqwest::header::USER_AGENT,
//         reqwest::header::HeaderValue::from_static(
//             "Mozilla/5.0 (X11; Linux i686; rv:109.0) Gecko/20100101 \
//              Firefox/119.0",
//         ),
//     );
//
//     let client = reqwest::Client::builder().default_headers(headers).build()?;
//
//     let res: reqwest::Response =
//         client.get("http://promiedos.com.ar").send().await?;
//
//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());
//
//     let body = res.text().await?;
//     println!("Body:\n{}", body);
//
//     Ok(())
// }

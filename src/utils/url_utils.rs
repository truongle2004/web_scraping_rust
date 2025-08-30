use reqwest::Client;

// pub async fn fetch_html_from_url(url: &String) -> Result<String, String> {
//     if url.is_empty() {
//         return Err("url cannot be empty".to_string());
//     }
//
//     let client = Client::new();
//     let response = client.get(url).send().await;
//
//     match response {
//         Ok(res) => {
//             let body = res.text().await;
//             match body {
//                 Ok(text) => Ok(text),
//                 Err(err) => Err(format!("Error to read response body: {}", err)),
//             }
//         }
//         Err(err) => Err(format!("Request failed: {}", err)),
//     }
// }
//

pub async fn fetch_html_from_url(url: &String) -> Result<String, String> {
    if url.is_empty() {
        return Err("url cannot be empty".to_string());
    }

    let client = Client::new();
    let response = client.get(url).send().await;

    match response {
        Ok(res) => {
            let body = res.text().await;
            match body {
                Ok(text) => Ok(text),
                Err(err) => Err(format!("Error to read response body: {}", err)),
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

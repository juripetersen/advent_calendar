use actix_web::{web, HttpResponse, Responder};
use futures::future::join_all;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::post().to(handle_winners_request)));
}

#[derive(Deserialize, Debug)]
struct NumbersRequestBody {
    numbers: Vec<String>,
}

async fn handle_winners_request(body: web::Json<NumbersRequestBody>) -> impl Responder {
    match fetch_winners(&body.numbers).await {
        Ok(response) => {
            return HttpResponse::Ok().json(response);
        }
        Err(error) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", error));
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
struct WinnerResponse {
    url: String,
    numbers: Vec<String>,
}

async fn fetch_winners(numbers: &Vec<String>) -> Result<Vec<WinnerResponse>, reqwest::Error> {
    let base_url = "https://www.tsv-huerup.de";
    let mut winner_responses: Vec<WinnerResponse> = Vec::new();
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.55 Safari/537.36")
        .build()
        .unwrap();

    let body = client
        .get(&format!("{}{}", base_url, "/adventskalender-2021"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a._2zDfX").unwrap();

    let mut handles: Vec<_> = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if href != "/adventskalender-2021/abwarten/" {
                let concat_url = format!("{}{}", base_url, href);
                handles.push(lookup_numbers_on_day(concat_url, &numbers));
            }
        }
    }

    let day_results = join_all(handles).await;
    for result in day_results {
        if let Some((url, numbers)) = result {
            winner_responses.push(WinnerResponse { url, numbers });
        }
    }

    Ok(winner_responses)
}

async fn lookup_numbers_on_day(
    url: String,
    numbers: &Vec<String>,
) -> Option<(String, Vec<String>)> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.55 Safari/537.36")
        .build()
        .unwrap();

    let body = client.get(&url).send().await.unwrap().text().await.unwrap();

    let document = Html::parse_document(&body);
    let selector = Selector::parse("h3").unwrap();

    let matching_numbers: Vec<String> = document
        .select(&selector)
        .filter(|element| numbers.contains(&element.text().next().unwrap().trim().to_owned()))
        .map(|element| element.text().next().unwrap().trim().to_owned())
        .collect();
    if matching_numbers.len() > 0 {
        return Some((url, matching_numbers));
    }

    None
}

use chrono::prelude::*;
use futures::stream::{self, StreamExt};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use telegram_bot::*;

mod room_list;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now();
    let payload1 = room_list::RequestPayload::new(
        "2020-10-27".to_string(),
        "2020-10-30".to_string(),
        "all".to_string(),
        "all".to_string(),
        "한국/제주/제주".to_string(),
        "none".to_string(),
        "none".to_string(),
        "none".to_string(),
        "none".to_string(),
        "none".to_string(),
        "none".to_string(),
        "none".to_string(),
        Arc::new(room_list::PageObj {
            current_page: 1,
            total_count: 1,
            per_page: 1,
        }),
        1,
        100,
    );
    let payload2 = room_list::RequestPayload {
        city: "한국/제주/서귀포".to_string(),
        ..(payload1.clone())
    };
    let payload3 = room_list::RequestPayload {
        check_in: "2020-12-25".to_string(),
        check_out: "2020-12-27".to_string(),
        ..(payload1.clone())
    };
    let payload4 = room_list::RequestPayload {
        city: "한국/제주/서귀포".to_string(),
        ..(payload3.clone())
    };
    let filter_list = vec![
        "spaceduck",
        "hwaoo_house",
        "monogarden",
        "vintage-jeju",
        "podo-hotel",
        "paulstay",
        "af-camp",
        "aroundfollie",
        "comfy-house",
        "dumogong",
        //"editorial-jeju",
        "diving-mat",
        "pyungdae-raum",
        "daily_rental_house",
        "hadabookstay",
        "quando-jeju",
        "byulado",
        "ononbellmoon",
        "uonaestay",
        "dolchae",
        "acoustic-mansion",
        "pyeongdae-stay",
        "pyeongdae-panorama",
        "stay-sodo",
        "harunharu",
        "jeju-tokki",
        "yeonamje",
        "af-cabin",
    ];
    let inner_filter = vec![("A동", "ilsanghosa")];

    let start = Local::now();

    let payloads = vec![payload1, payload2, payload3, payload4];

    let tasks: Vec<_> = payloads.into_iter().map(|payload| tokio::spawn(async move { get_room_list_result(payload).await.unwrap() })).collect();

    let results = stream::iter(tasks)
        .then(|f| async move { f.await })
        .collect::<Vec<_>>()
        .await;

    let finish = Local::now();
    let spend_time = finish.timestamp_millis() - start.timestamp_millis();
    println!("all spend time {}", spend_time);

    let return_value: Result<(), Box<dyn std::error::Error>> =
        match results.iter().find(|x| x.is_err()) {
            Some(_) => Err(Box::new(
                results
                    .into_iter()
                    .find(|x| x.is_err())
                    .unwrap()
                    .unwrap_err(),
            )),
            None => {
                let r: Vec<Arc<room_list::Item>> = results
                    .into_iter()
                    .flat_map(|x| x.unwrap().items)
                    .filter(|x| {
                        filter_list
                            .iter()
                            .find(|y| x.place.identifier.as_str() == **y)
                            .is_none()
                    })
                    .filter(|x| {
                        inner_filter
                            .iter()
                            .find(|(y1, y2)| {
                                x.name.as_str() == *y1 && x.place.identifier.as_str() == *y2
                            })
                            .is_none()
                    })
                    .collect();
                println!("[{}] {}", now.to_string(), serde_json::to_string(&r)?);
                if !r.is_empty() {
                    let msgs = r
                        .iter()
                        .map(|x| format!("{} ({})", x.name, x.place.name_kr))
                        .collect::<Vec<_>>();

                    let commands =
                        vec![format!("지금이니! (https://booking.stayfolio.com)").to_string()];

                    send_telegram([msgs, commands].concat::<String>().join("\n").as_str()).await?
                } else {
                    ()
                }
                Ok(())
            }
        };
    return_value
}

#[allow(dead_code)]
async fn send_watch_tower(msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://api.noti.daumkakao.io/send/group/kakaotalk";
    let mut params = HashMap::new();
    params.insert("to", "9016");
    params.insert("msg", msg);
    reqwest::Client::builder()
        .gzip(true)
        .build()?
        .post(url)
        .form(&params)
        .send()
        .await?;
    Ok(())
}

async fn send_telegram(msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    let telegram_token = env::var("TELEGRAM_TOKEN")?;
    let api = Api::new(telegram_token);
    let req = requests::SendMessage::new(ChannelId::new(-1001331000957), msg);
    api.send(req).await?;
    Ok(())
}

async fn get_room_list_result(
    payload: room_list::RequestPayload,
) -> Result<room_list::Response, Box<dyn std::error::Error>> {
    let url = "https://booking.stayfolio.com/places/room_list.json";
    let start = Local::now();
    let resp = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .gzip(true)
        .build()?
        .post(url)
        .json(&payload)
        .send()
        .await?;
    let result_text = resp.text().await?;
    let finish = Local::now();
    let spend_time = finish.timestamp_millis() - start.timestamp_millis();
    println!("spend time {}", spend_time);
    Ok(serde_json::from_str(result_text.as_str())?)
}

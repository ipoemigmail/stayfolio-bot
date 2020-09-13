use serde::{Deserialize, Serialize};
use std::rc::Rc;

/*
{
  "check_in": "2020-10-28",
  "check_out": "2020-10-30",
  "placeType": "all",
  "promotion": "all",
  "city": "한국/제주/제주",
  "order": "none",
  "adultCnt": "none",
  "childCnt": "none",
  "babyCnt": "none",
  "order_keyword": "none",
  "order_service": "none",
  "order_facility": "none",
  "pageObj": {
    "current_page": 1,
    "total_count": 1,
    "per_page": 1
  },
  "page": 1,
  "per": 12,
  "adult_cnt": "none",
  "child_cnt": "none",
  "baby_cnt": "none",
  "place_type": "all"
}
*/

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestPayload {
  pub check_in: String,
  pub check_out: String,
  pub placeType: String,
  pub promotion: String,
  pub city: String,
  pub order: String,
  pub adultCnt: String,
  pub childCnt: String,
  pub babyCnt: String,
  pub order_keyword: String,
  pub order_service: String,
  pub order_facility: String,
  pub pageObj: Rc<PageObj>,
  pub page: i32,
  pub per: i32,
  pub adult_cnt: String,
  pub child_cnt: String,
  pub baby_cnt: String,
  pub place_type: String,
}

#[allow(non_snake_case)]
impl RequestPayload {
  pub fn new(
    check_in: String,
    check_out: String,
    place_type: String,
    promotion: String,
    city: String,
    order: String,
    adult_cnt: String,
    child_cnt: String,
    baby_cnt: String,
    order_keyword: String,
    order_service: String,
    order_facility: String,
    pageObj: Rc<PageObj>,
    page: i32,
    per: i32,
  ) -> RequestPayload {
    let adultCnt = adult_cnt.clone();
    let childCnt = child_cnt.clone();
    let babyCnt = baby_cnt.clone();
    let placeType = place_type.clone();

    RequestPayload {
      check_in,
      check_out,
      placeType,
      promotion,
      city,
      order,
      adultCnt,
      childCnt,
      babyCnt,
      order_keyword,
      order_service,
      order_facility,
      pageObj,
      page,
      per,
      adult_cnt,
      child_cnt,
      baby_cnt,
      place_type,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageObj {
  pub current_page: i32,
  pub total_count: i32,
  pub per_page: i32,
}

/*
 "items": [
   {
     "id": 24,
     "name": "오리동",
     "description": "연인 또는 친구와 함께 머물기 좋은 2인 객실입니다. 원형계단과 해치, 1층의 담 사이로 숨겨진 노천탕 등 독특한 공간 구성이 돋보입니다. 오리의 눈에 해당하는 작은 창을 통해 마을 풍경과 멀리 오름이 보입니다. ",
     "passenger_cnt_min": 1,
     "passenger_cnt_max": 2,
     "place": {
       "id": 168,
       "identifier": "spaceduck",
       "name": "SpaceDuck",
       "name_kr": "우주오리",
     }
   }
 ],
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
  pub items: Vec<Rc<Item>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
  pub id: i64,
  pub name: String,
  pub description: String,
  pub passenger_cnt_min: i32,
  pub passenger_cnt_max: i32,
  pub place: Rc<Place>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
  pub id: i64,
  pub identifier: String,
  pub name: String,
  pub name_kr: String,
}

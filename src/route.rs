use querystring::stringify;
use reqwest;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use crate::BING_MAPS_API_KEY;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RouteResource {
    travelDistance: f64,
    travelDuration: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RouteResourceSet {
    estimatedTotal: u32,
    resources: Vec<RouteResource>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RoutesAPIResponse {
    resourceSets: Vec<RouteResourceSet>,
}

pub async fn get_distance(wp_1: (f64, f64), wp_2: (f64, f64)) -> (f64, u32) {
    // Craft URL
    let url = "http://dev.virtualearth.net/REST/v1/Routes";
    let params = stringify(vec![
        ("wayPoint.1", &format!("{},{}", wp_1.0, wp_1.1)),
        ("waypoint.2", &format!("{},{}", wp_2.0, wp_2.1)),
        ("key", BING_MAPS_API_KEY),
    ]);

    // Make request
    let client = reqwest::Client::new();
    let res = client
        .get(&format!("{}?{}", url, params))
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap();

    // Parse response
    let body = res.text().await.unwrap();
    let response: RoutesAPIResponse = serde_json::from_str(&body).unwrap();
    let distance = response.resourceSets[0].resources[0].travelDistance;
    let duration = response.resourceSets[0].resources[0].travelDuration;
    (distance, duration)
}

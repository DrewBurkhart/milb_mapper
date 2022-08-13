use querystring::stringify;
use reqwest;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use crate::BING_MAPS_API_KEY;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    coordinates: Vec<f64>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct LocationResource {
    point: Point,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct LocationResourceSet {
    estimatedTotal: u32,
    resources: Vec<LocationResource>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct LocationsAPIResponse {
    resourceSets: Vec<LocationResourceSet>,
}

pub async fn get_coords(address: String) -> (f64, f64) {
    // Format address line
    let mut address_string = stringify(vec![("addressLine", &address)]);
    address_string.pop();

    // Craft URL
    let url = format!(
        "http://dev.virtualearth.net/REST/v1/Locations/US/{}?",
        address_string
    );
    let params = stringify(vec![("maxReults", "1"), ("key", BING_MAPS_API_KEY)]);

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
    let response: LocationsAPIResponse = serde_json::from_str(&body).unwrap();
    let coords = &response.resourceSets[0].resources[0].point.coordinates;
    (coords[0], coords[1])
}

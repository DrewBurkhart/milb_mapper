use querystring::stringify;
use reqwest;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    coordinates: Vec<f64>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct LocationResource {
    point: Point,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct RouteResource {
    travelDistance: f64,
    travelDuration: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct LocationResourceSet {
    estimatedTotal: u32,
    resources: Vec<LocationResource>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct RouteResourceSet {
    estimatedTotal: u32,
    resources: Vec<RouteResource>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct RoutesAPIResponse {
    resourceSets: Vec<RouteResourceSet>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct LocationsAPIResponse {
    resourceSets: Vec<LocationResourceSet>,
}

const BING_MAPS_API_KEY: &str = "AvMjRjCSlLHs7MX4Fa9dAMPsGxtOMSfuKxExT4-Fb22YuDSCurIzZd2x_Iu8O3qu";

fn log_distance(distance: f64, duration: u32) {
    println!("Distance: {} km", distance);
    println!("Duration: {} minutes", duration);
}

async fn get_coords(address: &str) -> (f64, f64) {
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

async fn get_distance(wp_1: (f64, f64), wp_2: (f64, f64)) -> (f64, u32) {
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

#[tokio::main]
async fn main() {
    // Get addresses from command line
    let args: Vec<String> = env::args().collect();
    let address1 = &args[1];
    let address2 = &args[2];

    // Get coordinates
    let (lat1, lon1) = get_coords(address1).await;
    let (lat2, lon2) = get_coords(address2).await;

    // Get distance and duration from Bing Maps API
    let distance = get_distance((lat1, lon1), (lat2, lon2)).await;

    // Log distance and duration
    log_distance(distance.0, distance.1);
}

// let url = format!(
//     "http://dev.virtualearth.net/REST/v1/Routes?wayPoint.1={wp_1_lat},{wp_1_long}&waypoint.2={wp_2_lat},{wp_2_long}&key={bing_key}",
//     wp_1_lat = coords1.0,
//     wp_1_long = coords1.1,
//     wp_2_lat = coords2.0,
//     wp_2_long = coords2.1,
//     bing_key = BING_MAPS_API_KEY
// );
// let client = reqwest::Client::new();
// let response = client
//     .get(url)
//     .header(CONTENT_TYPE, "application/json")
//     .header(ACCEPT, "application/json")
//     .send()
//     .await
//     .unwrap();
// match response.status() {
//     reqwest::StatusCode::OK => {
//         match response.json::<RoutesAPIResponse>().await {
//             Ok(api_response) => {
//                 let resource_set = &api_response.resourceSets[0];
//                 let resource = &resource_set.resources[0];
//                 log_distance(resource.travelDistance, resource.travelDuration);
//             }
//             Err(parsed) => println!(
//                 "Hm, the response didn't match the shape we expected. {:?}",
//                 parsed
//             ),
//         };
//     }
//     reqwest::StatusCode::UNAUTHORIZED => {
//         println!("Issue with the token");
//     }
//     other => {
//         panic!("Uh oh! Something unexpected happened: {:?}", other);
//     }
// };

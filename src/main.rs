extern crate csv;
use std::env;
use tabled::Table;

use crate::{
    affiliate::{get_affiliates, Affiliate},
    location::get_coords,
    route::get_distance,
};

mod affiliate;
mod location;
mod route;

const BING_MAPS_API_KEY: String = env::var("BING_MAPS_API_KEY").unwrap();

#[tokio::main]
async fn main() {
    // Get addresses from command line
    let args: Vec<String> = env::args().collect();
    let club = &args[1];

    // Get coordinates
    let affiliates = get_affiliates(club).await;
    let affiliates_list = &affiliates;
    let mut affiliate_coords = Vec::new();
    match affiliates_list {
        Ok(affiliates_list) => {
            for affiliate in affiliates_list {
                affiliate_coords
                    .push(get_coords(format!("{}, {}", &affiliate.city, &affiliate.state)).await);
            }
        }
        Err(e) => println!("Error {:?}", e),
    }

    // Table to hold values
    let mut teams: Vec<Affiliate> = Vec::new();

    // Get distance and duration from Bing Maps API
    for (i, coord) in affiliate_coords.iter().enumerate() {
        for (j, coord_2) in affiliate_coords.iter().enumerate() {
            if i != j {
                let distance = get_distance(*coord, *coord_2).await;
                teams.push(Affiliate {
                    level: affiliates.as_ref().unwrap()[i].level.clone(),
                    name: affiliates.as_ref().unwrap()[j].team.clone(),
                    city: affiliates.as_ref().unwrap()[j].city.clone(),
                    state: affiliates.as_ref().unwrap()[j].state.clone(),
                    distance: distance.0,
                    duration: distance.1,
                });
            }
        }
    }

    let table = Table::new(teams).to_string();
    println!("{}", table);
}

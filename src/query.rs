// Possible Star Wars data types
// that a GET request can return
#[derive(Debug, Deserialize)]
pub enum StarWarsType {
    People(super::People),
    Films,
    Starships,
    Vehicles,
    Species,
    Planets(super::Planet),
}

pub fn api_query(endpoint: &str) -> Result<reqwest::Response, String> {
    // Base URL for all API requests
    let base_url: String = "https://swapi.co/api".to_owned();
    // Concatenate endpoint onto base_url
    let query_url: &str = &(base_url + &endpoint);

    let response = reqwest::get(query_url).expect("Failed request!");

    match response.status().is_success() {
        false => Err(format!("{} is an invalid request!", query_url)),
        true => Ok(response),
    }
}

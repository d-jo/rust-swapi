#[derive(Debug, Deserialize, Default)]
pub struct People {
    name: String,
    birth_year: String,
    eye_color: String,
    gender: String,
    hair_color: String,
    height: String,
    mass: String,
    skin_color: String,
    homeworld: String,
    films: Vec<String>,
    species: Vec<String>,
    starships: Vec<String>,
    vehicles: Vec<String>,
    url: String,
    created: String,
    edited: String,
}

pub fn query_people(people_num: &str) -> Result<People, String> {
    // Base URL for a people request
    let base_url: String = "/people/".to_owned();
    let people_url: &str = &(base_url + &people_num);

    // use the generic query
    let resp: Result<reqwest::Response, String> = super::query::api_query(people_url);
    
    // match the result
    match resp {
        // if its OK, get a mutable version
        Ok(mut v) => {
            // use the reqwest json method to unpack the response
            let person: Result<People, reqwest::Error> = v.json::<People>();

            // shadow person with the correct type to prepare to return
            let person: Result<People, String> = match person {
                // person was unpacked correctly
                Ok(p) => Ok(p), 
                // return the error if json unpacking was bad
                Err(er) => Err(String::from(format!("{:?}", er))), 
            };
            person // return the person
        },

        // if the response failed, pass that up
        Err(e) => Err(e), 
    }
}

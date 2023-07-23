use reqwest;

pub mod model;

/// Get the fridge data from the Foodles API
/// 
/// # Arguments
/// 
/// * `session_secret` - The session secret to use for the request
/// 
pub fn get(session_secret: &str) -> Result<model::Fridge, reqwest::Error> {
    // Create an HTTP client with the cookies in the headers
    let client = reqwest::blocking::Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::COOKIE,
                reqwest::header::HeaderValue::from_str(&session_secret).unwrap(),
            );
            headers
        })
        .build()?;

    let response = client
        .get("https://api.foodles.co/api/fridge/")
        .send()?;

    // Make sure the response is successful
    response.error_for_status_ref()?;

    // Read the JSON data from the response
    response.json()
}
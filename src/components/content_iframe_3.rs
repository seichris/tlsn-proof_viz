use yew::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use log;
use spansy::http::parse_response;

const KEY: &str = "app::twitter_data";

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub bytes: Vec<u8>,
}

fn load_from_storage(key: &str) -> Option<String> {
    let storage = LocalStorage::raw();
    storage.get(key).unwrap_or_default()
}

fn save_to_storage(key: &str, data: &str) {
    let storage = LocalStorage::raw();
    storage.set(key, data).unwrap_or_else(|_| {
        log::error!("Error saving to storage");
    });
}

#[derive(Debug)]
enum ContentType {
    Html,
    Json,
    Other,
}

fn get_content_type(bytes: &[u8]) -> (ContentType, String) {
    match parse_response(&bytes) {
        Ok(x) => {
            // log!(format!("Test {:?}", x.headers));

            let content_type = (&x)
                .header("Content-Type")
                .map_or(ContentType::Other, |header| {
                    let type_string = String::from_utf8_lossy(header.value.as_bytes());
                    match type_string {
                        s if s.contains("text/html") => ContentType::Html,
                        s if s.contains("application/json") => ContentType::Json,
                        _ => ContentType::Other,
                    }
                });

            let body = x.body.map_or(String::new(), |body| {
                String::from_utf8_lossy(body.as_bytes()).to_string()
            });

            // log!(format!("Test {:?}", content_type));

            (content_type, body)
        }
        Err(e) => (ContentType::Other, e.to_string()),
    }
}

fn render_twitter_data(content: &str) -> String {
    // Check if the content starts with a specific pattern or contains specific keys
    // Adjust these conditions based on the actual content you expect
    if content.starts_with("{\"users\":[") {
        // Extract the relevant information from the string
        // This is a simplistic example, you might need a more sophisticated approach
        let screen_name = extract_screen_name(content);
        let is_verified = extract_is_verified(content);

        format!("Screen Name: {}\nIs Verified: {}\n\n", screen_name, is_verified)
    } else {
        // Return a default message if the content doesn't match the expected format
        "No Twitter data found or format is not recognized.".to_string()
    }
}

// Function to extract screen name
fn extract_screen_name(content: &str) -> String {
    // Look for the "screen_name" key and extract the value
    let key = "\"screen_name\":\"";
    content.find(key).and_then(|start| {
        let remaining = &content[start + key.len()..];
        remaining.split('"').next().map(String::from)
    }).unwrap_or_default()
}

// Function to extract is_verified status
fn extract_is_verified(content: &str) -> bool {
    // Look for the "is_verified" key and extract the value
    let key = "\"is_verified\":";
    content.find(key).and_then(|start| {
        let remaining = &content[start + key.len()..];
        remaining.split(',').next().map(|value| {
            value.trim().eq("true")
        })
    }).unwrap_or(false)
}

#[function_component]
pub fn ContentIFrame3(props: &Props) -> Html {
    let initial_data = if !props.bytes.is_empty() {
        String::from_utf8_lossy(&props.bytes).to_string()
    } else {
        load_from_storage(KEY).unwrap_or_default()
    };

    let twitter_data = use_state(|| initial_data);
    let twitter_data_clone = twitter_data.clone();

    {
        let bytes_clone = props.bytes.clone();
        use_effect(move || {
            // Determine the content type of the data
            let (content_type, content) = get_content_type(&bytes_clone);
        
            // Initialize a variable to hold the data to be set
            let mut data_to_set = None;
        
            // Match against the content type to process the data accordingly
            match content_type {
                ContentType::Json => {
                    // If the content type is JSON, process it using `render_twitter_data`
                    let data = render_twitter_data(&content);
                    data_to_set = Some(data);
                },
                ContentType::Html => {
                    // Handle HTML content here if needed
                },
                ContentType::Other => {
                    // For unrecognized content types, set a default message
                    data_to_set = Some("Non-JSON format or unrecognized data.".to_string());
                },
            }
        
            // If there is data to set, update the state and save it to local storage
            if let Some(data) = data_to_set {
                // Check if the current state is different from the new data
                if *twitter_data_clone != data {
                    // Update the state with the new data
                    twitter_data_clone.set(data.clone());
                    // Save the new data to local storage
                    save_to_storage(KEY, &data);
                }
            }
        
            // Cleanup function (optional)
            || ()
        });
        
    }    

    html! {
        <div class="bg-black text-white p-4 rounded-md overflow-x-auto">
            {(*twitter_data).clone()}
        </div>
    }
}

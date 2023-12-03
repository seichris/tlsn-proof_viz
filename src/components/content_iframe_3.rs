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
        // log::error!("Error saving to storage: {:?}", err);
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

// fn render_twitter_data(json_str: &str) -> String {
//     // Attempt to parse the JSON string
//     match serde_json::from_str::<serde_json::Value>(json_str) {
//         Ok(json) => {
//             // Check if 'users' array is present and iterate through it
//             if let Some(users) = json.get("users").and_then(|u| u.as_array()) {
//                 let mut output = String::new();
//                 for user in users.iter() {
//                     // Extract 'screen_name' and 'is_verified' fields
//                     let screen_name = user.get("screen_name").and_then(|sn| sn.as_str()).unwrap_or_default();
//                     let is_verified = user.get("is_verified").and_then(|iv| iv.as_bool()).unwrap_or(false);

//                     // Append extracted data to the output string
//                     output += &format!("Screen Name: {}\nIs Verified: {}\n\n", screen_name, is_verified);
//                 }
//                 output
//             } else {
//                 "No Twitter data found.".to_string()
//             }
//         }
//         Err(_) => "Invalid JSON format.".to_string(),
//     }
// }
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
    let twitter_data = use_state(|| String::new());
    let twitter_data_clone = twitter_data.clone();
    // let (content_type, content) = get_content_type(&props.bytes);

    // Load data from storage when the component mounts
    {
        let twitter_data = twitter_data.clone();
        use_effect_with((), move |_| {
            if let Some(loaded_data) = load_from_storage(KEY) {
                twitter_data.set(loaded_data);
            }
            || ()
        });
    }

    // Save new data to storage when bytes change
    {
        let bytes_clone = props.bytes.clone();

        use_effect(move || {
            // Determine content type
            let (content_type, content) = get_content_type(&bytes_clone);

            // Initialize a variable to hold the data to be set
            let mut data_to_set = None;
        
            match content_type {
                ContentType::Json => {
                    // Process as JSON
                    let data = render_twitter_data(&content);
                    save_to_storage(KEY, &data);
                    // twitter_data_clone.set(data);
                    // if *twitter_data_clone != data {
                    //     twitter_data_clone.set(data);
                    // }
                    data_to_set = Some(data);
                },
                ContentType::Html => {
                    // Handle HTML content (or ignore if not relevant)
                },
                ContentType::Other => {
                    // Handle other content types or show a message
                    // if *twitter_data_clone != data {
                    //     twitter_data_clone.set("Non-JSON format or unrecognized data.".to_string());
                    // }
                    data_to_set = Some("Non-JSON format or unrecognized data.".to_string());
                },
            }

            if let Some(data) = data_to_set {
                if *twitter_data_clone != data {
                    twitter_data_clone.set(data);
                }
            }

            || ()
        });
    }

    html! {
        <div class="bg-black text-white p-4 rounded-md overflow-x-auto">
            {(*twitter_data).clone()}
        </div>
    }

}



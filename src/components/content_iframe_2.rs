// use gloo::console::log;
// use std::fmt;

use spansy::http::parse_response;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub bytes: Vec<u8>,
}

fn render_json(content: &str) -> String {
    let json = serde_json::from_str::<serde_json::Value>(content);
    if let Ok(json) = json {
        serde_json::to_string_pretty(&json).unwrap()
    } else {
        content.to_string()
    }
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

#[function_component]
pub fn ContentIFrame2(props: &Props) -> Html {
    use_effect(move || highlight_code());

    fn render_twitter_data(content: &str) -> Html {
        // Check if the content starts with a specific pattern or contains specific keys
        // Adjust the conditions based on the actual content you expect
        if content.starts_with("{\"users\":[") {
            // Extract the relevant information from the string
            // This is a simplistic example, you might need a more sophisticated approach
            let screen_name = extract_screen_name(content);
            let is_verified = extract_is_verified(content);
    
            html! {
                <>
                    <h2>{"Twitter Data"}</h2>
                    <div>
                        <p>{format!("Screen Name: {}", screen_name)}</p>
                        <p>{format!("Is Verified: {}", is_verified)}</p>
                    </div>
                </>
            }
        } else {
            html! { <p>{"No Twitter data found or format is not recognized."}</p> }
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


    let content = match get_content_type(&props.bytes) {
        (ContentType::Html, content_html) => html! {
            <details class="p-4 w-5/6" open={true}>
                <summary><b>{"Received HTML content:"}</b></summary>
                <iframe class="w-full h-64" srcdoc={content_html} src="demo_iframe_srcdoc.htm">
                    <p>{">Your browser does not support iframes."}</p>
                </iframe>
            </details>
        },
        (ContentType::Json, content_json) => html! {
            <div class="p-4 w-5/6">
                {render_twitter_data(&content_json)}
                <details class="p-4 w-5/6" open={true}>
                    <summary><b>{"Received JSON content :"}</b></summary>
                    <div class="bg-black text-white p-4 rounded-md overflow-x-auto">
                        <pre>
                            <code class="lang-json">
                                {render_json(&content_json)}
                            </code>
                        </pre>
                    </div>
                </details>
            </div>
        },
        _ => html! {},
    };

    content
}

#[wasm_bindgen(inline_js = "export function highlight_code() { Prism.highlightAll(); }")]
extern "C" {
    fn highlight_code();
}

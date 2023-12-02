use serde_json::Value;
use yew::{function_component, html, Properties, use_state};
use yew::Html;

#[derive(Properties, PartialEq)]
pub struct PassportStampsProps {
    pub server_domain: String,
    pub json_data: String,
}

#[function_component(PassportStamps)]
pub fn passport_stamps(props: &PassportStampsProps) -> Html {
    let data = use_state(|| None);

    if props.server_domain == "api.twitter.com" {
        let parsed_json: Result<Value, serde_json::Error> = serde_json::from_str(&props.json_data);
        if let Ok(json) = parsed_json {
            // Convert the JSON value to a string for logging
            let json_string = serde_json::to_string(&json).unwrap_or_else(|_| "Failed to serialize JSON".to_string());
            gloo::console::log!("Parsed JSON:", &json_string);

            if let Some(users) = json.get("users").and_then(|u| u.as_array()) {
                for user in users {
                    let screen_name = user.get("screen_name").and_then(|n| n.as_str()).unwrap_or_default();
                    let is_verified = user.get("is_verified").and_then(|v| v.as_bool()).unwrap_or(false);
                    gloo::console::log!("User:", screen_name, "Is Verified:", is_verified);
                    data.set(Some((screen_name.to_string(), is_verified)));
                }
            }
        } else {
            gloo::console::log!("Failed to parse JSON");
        }
    }

    html! {
        if let Some((screen_name, is_verified)) = (*data).clone() {
            <div>
                <p>{ format!("Screen Name: {}", screen_name) }</p>
                <p>{ format!("Is Verified: {}", is_verified) }</p>
            </div>
        } else {
            <p>{ "No Twitter data available. :(" }</p>
        }
    }
}

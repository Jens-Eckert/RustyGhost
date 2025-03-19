#![allow(nonstandard_style)]

use std::collections::HashMap;

use BungieTypes::*;
use gloo_console::log;
use gloo_net::http::Request;
use load_dotenv::load_dotenv;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

pub mod BungieTypes;

load_dotenv!();

const API_ROOT: &str = "https://www.bungie.net/Platform";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct AppGlobalData {
    api_key: String,
    client_id: String,
    manifest: Destiny2Manifest,
}

impl AppGlobalData {
    fn init() -> AppGlobalData {
        let api_key = env!("BUNGIE_API_KEY").to_string();
        let client_id = env!("OAUTH_CLIENT_ID").to_string();

        Self {
            api_key,
            client_id,
            manifest: Destiny2Manifest::default(),
        }
    }
}

#[function_component]
fn App() -> Html {
    let mut app_data = AppGlobalData::init();

    html! {
        <div class="flex flex-col items-center h-screen w-full">
            <div class="flex flex-row items-center h-full">
            <p>{"Under construction!"}</p>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

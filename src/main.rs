#![allow(nonstandard_style)]

use std::sync::{Arc, Mutex};

use BungieTypes::*;
use load_dotenv::load_dotenv;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

pub mod BungieTypes;

load_dotenv!();

const API_ROOT: &str = "https://www.bungie.net/Platform";

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
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

#[derive(Properties)]
struct AppProps {
    app_data: Arc<Mutex<AppGlobalData>>,
}

impl PartialEq for AppProps {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

#[function_component]
fn App(props: &AppProps) -> Html {
    let app_data = props.app_data.clone();

    let count = use_state(|| 0);

    let onclick = {
        let count = count.clone();

        Callback::from(move |_| {
            let c = *count;
            count.set(c + 1);
        })
    };

    html! {
        <div class="flex flex-col items-center h-screen w-full">
            <div class="flex flex-row items-center h-full">
            <div class="flex flex-col items-center">
            <p>{"Under construction!"}</p>
            <p>{"While you're here, press this button a lot!"}</p>

            <button
            type="button"
            class="border-solid border-2 border-x-indigo-500 rounded-lg bg-cyan-500 shadow-lg shadow-cyan-500/50"
            {onclick}>
                {"Wao"}
            </button>

            <p>{format!("{}", *count)}</p>
            if *count == 69 {
                <p>{"nice"}</p>
            }
            </div>
            </div>
        </div>
    }
}

fn main() {
    let app_data = Arc::new(Mutex::new(AppGlobalData::init()));

    yew::Renderer::<App>::with_props(AppProps { app_data }).render();
}

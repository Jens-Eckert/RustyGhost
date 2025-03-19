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

    let manifest = use_state(|| Destiny2Manifest::default());

    {
        let manifest = manifest.clone();
        let mut app_data = app_data.clone();

        use_effect_with((), move |_| {
            let req = API_ROOT.to_string() + "/Destiny2/Manifest";

            let manifest = manifest.clone();
            let mut app_data = app_data.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let response: GenericResponse<Destiny2Manifest> = Request::get(&req)
                    .header("X-API-Key", &app_data.api_key)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                app_data.manifest = response.Response.unwrap();
                manifest.set(app_data.manifest);
            });

            || ()
        });
    }

    html! {
        <div class="flex flex-col items-center">
            <p>{format!("{:?}", *manifest)}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#![allow(non_snake_case)]

use dioxus::prelude::*;
use grind_engine::models::Task;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

static CSS: Asset = asset!("/assets/styles.css");

pub async fn App() -> Element {
    let tasks: Vec<Task> = invoke("get_tasks", JsValue::null()).await;

    rsx! {
        document::Stylesheet { href: CSS }
        h1 {
            "hello worldo"
        }
        h2 {
            "baka baka"
        }
        for task in tasks {
            TaskView { id: task.id, name: task.name }
        }
    }
}

#[component]
fn TaskView(id: u32, name: String) -> Element {
    rsx! {
        div {
            h1 {
                {id}
            }
            h2 {
                {name}
            }
        }
    }
}

use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use reqwasm::http::Request;

use std::ops::Deref;

#[derive(Default, Clone, PartialEq)]
pub struct StatesData {
    pub input_name: String,
    pub input_password: String,
}

#[function_component(Form)]
pub fn form_function() -> Html {
    let states_data = use_state(|| StatesData::default());

    let states_data_clone = states_data.clone();
    let onchange_name = Callback::from(move |input_event: Event| {
        let input_event_target = input_event.target().unwrap();
        let current_input_text = input_event_target.unchecked_into::<HtmlInputElement>();
        states_data_clone.set(
            StatesData {
                input_name: current_input_text.value(),
                ..states_data_clone.deref().clone()
            }
        )
    });
    let states_data_clone = states_data.clone();
    let onchange_password = Callback::from(move |input_event: Event| {
        let input_event_target = input_event.target().unwrap();
        let current_input_text = input_event_target.unchecked_into::<HtmlInputElement>();
        states_data_clone.set(
            StatesData {
                input_password: current_input_text.value(),
                ..states_data_clone.deref().clone()
            }
        )
    });

    let actix_url = "http://localhost:8080";
    let add_endpoint = format!("{}/add", actix_url);
    let save_password: Callback<MouseEvent> = Callback::from(move |_| {
        let states_data_clone = states_data.clone();
        let add_endpoint_clone = add_endpoint.clone();
        if *states_data.deref() != StatesData::default() {
            wasm_bindgen_futures::spawn_local(async move {
                Request::post(&add_endpoint_clone)
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(format!("name={}&password={}", states_data_clone.input_name, states_data_clone.input_password))
                    .send()
                    .await
                    .expect("Failed to execute request.");
            });
        }
    });

    html! {
        <ul>
            <div>
                <p> { "Name: " } </p>
                <input type="string" onchange={onchange_name} />
                <p> { "Password: " } </p>
                <input type="password" onchange={onchange_password} />
                <button> { "Random password" }</button>
            </div>
            <p></p>
            <button onclick={save_password}> { "Save" } </button>
        </ul>
    }
}
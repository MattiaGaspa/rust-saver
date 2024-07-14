pub mod components;

use yew::prelude::*;
use components::{show_passwords::ShowPasswords, form::Form};

#[function_component(App)]
fn app() -> Html {    
    html! {
        <>
            <h1> { "RUST-SAVER" } </h1>
            <Form />
            <hr />
            <div>
                <p> { "Saved password:" } </p>
                <ShowPasswords />
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
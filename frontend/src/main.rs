use reqwasm::http::Request;
use types::*;
use yew::prelude::*;


#[function_component(HttpGetExample)]
fn get_example() -> Html {
    let actix_url: String = format!("http://localhost:8080");
    let hello_response = Box::new(use_state(|| None));
    let error = Box::new(use_state(|| None));
    let endpoint = Box::new(format!(
        "{actix_url}/show",
        actix_url = actix_url,
    ));
    let retry = {
        let hello_response = hello_response.clone();
        let error = error.clone();
        let endpoint = endpoint.clone();
        Callback::from(move |_| {
            let hello_response = hello_response.clone();
            let error = error.clone();
            let endpoint = endpoint.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_response = Request::get(&endpoint).send().await;
                match fetched_response {
                    Ok(response) => {
                        let json: Result<PasswordList, _> = response.json().await;
                        match json {
                            Ok(f) => {
                                hello_response.set(Some(f));
                            }
                            Err(e) => error.set(Some(e.to_string())),
                        }
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }
            });
        })
    };

    match (*hello_response).as_ref() {
        Some(response) => html! {
            <>
                <ol>
                    { for response.list.clone().into_iter().map(|elem| html_nested!{ <li> { format!("{}: {}", elem.name, elem.password) } </li> }) }
                </ol>
                <div>
                    <button onclick={retry}>{"fetch"}</button>
                </div>
            </>
        },
        None => match (*error).as_ref() {
            Some(e) => {
                html! {
                    <>
                        {"error"} {e}
                        <button onclick={retry}>{"retry"}</button>
                    </>
                }
            }
            None => {
                html! {
                    <>
                        <button onclick={retry}>{"Call GET "}{endpoint}</button>
                    </>
                }
            }
        },
    }
}

fn main() {
    yew::Renderer::<HttpGetExample>::new().render();
}
use reqwasm::http::Request;
use types::*;
use yew::prelude::*;

#[function_component]
pub fn ShowPasswords() -> Html {
    let actix_url = "http://127.0.0.1:8080";
    let password_list = use_state(|| None);
    let error = use_state(|| None);
    let show_endpoint = format!("{}/show", actix_url);
    
    let password_list_clone = password_list.clone();
    let error_clone = error.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let fetched_response = Request::get(&show_endpoint).send().await;
        match fetched_response {
            Ok(response) => {
                let json: Result<PasswordList, _> = response.json().await;
                match json {
                    Ok(f) => {
                        password_list_clone.set(Some(f));
                    }
                    Err(e) => error_clone.set(Some(e.to_string())),
                }
            }
            Err(e) => error_clone.set(Some(e.to_string())),
        }
    });
    

    match (*password_list).as_ref() {
        Some(response) => html! {
            <>
                <ol>
                    { for response.list.clone().into_iter().map(|elem| html_nested!{ <li> { format!("{}: {}", elem.name, elem.password) } </li> }) }
                </ol>
            </>
        },
        None => match (*error).as_ref() {
            Some(e) => {
                html! {
                    <>
                        {"error"} {e}
                    </>
                }
            }
            None => {
                html! {}
            }
        },
    }
}
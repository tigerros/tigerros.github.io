use gloo_net::http::Request;

// pub fn http_get_text(url: &str) -> String {
//     let response = Arc::new(Mutex::new(String::from("Def")));
//     let response_thread = Arc::clone(&response);
//
//     let set_response = async move {
//         let mut guard = response_thread.lock().unwrap();
//
//         (*guard).push_str("New");
//         info!("Changed response");
//     };
//
//     wasm_bindgen_futures::spawn_local(set_response);
//     info!("Spawned");
//
//     let a = response.lock().unwrap().to_owned();
//
//     info!("Final value: {}", a);
//
//     a
// }

pub async fn http_get_text(url: &str) -> String {
    let request = Request::get(url);
    let response = request.send().await.unwrap().text().await.unwrap();

    response
}

use crate::user::GoogleIdentifiedUser;
use enclose::enc;
use google_sign_in_wasm::GoogleUser;
use seed::{prelude::*, *};
use serde::Deserialize;

mod user;
// ------ ------
//     Init
// ------ ------

pub fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        // Load config from some json
        // You can have a specific Api key here for youtube or gmail for example
        Msg::ConfigFetched(
            async { fetch("/config.json").await?.check_status()?.json().await }.await,
        )
    });
    Model {
        config: None,
        user: None,
        error: None,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    config: Option<Config>, /* useless in this example, just here to show how to load some
                             * config with client_id or api */
    user: Option<GoogleIdentifiedUser>,
    error: Option<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub api_key: String,
    pub client_id: String,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    SignedIn(GoogleUser),
    SignedFailed(String),
    ConfigFetched(fetch::Result<Config>),
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ConfigFetched(Ok(config)) => model.config = Some(config),
        Msg::ConfigFetched(Err(fetch_error)) => error!("Config fetch failed! Be sure to have config.json at the root of your project with client:di and api_key", fetch_error),
        Msg::SignedIn(user) => {
           model.user = Some(
               GoogleIdentifiedUser::new(
                   user
                   .getBasicProfile()
                   .expect("Should have get profile"), user.getAuthResponse(true).unwrap().access_token().unwrap()));

            // You can use `model.user.access_token()` to send it to backend, and verify it to extract the sub with the user_id from google in it
            // You can use the api_key from the config and the access_token to make queries to other google services : Ex Youtube https://developers.google.com/youtube/v3/docs/channels/list?apix=true&apix_params=%7B%22part%22%3A%22contentDetails%22%2C%22mine%22%3A%22true%22%7D
        }
        Msg::SignedFailed(err) => {model.error = Some(err)}
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        google_sign_in_wasm::sign_in::default_google_button(
            success(),
            fail(),
            "profile email",
            &250,
            &50,
            "dark"
        ),
        p![display_user_information(&model.user)],
        p![
            style! { St::Color=> "red"},
            model.error.as_ref().unwrap_or(&"".to_string())
        ]
    ]
}

/// Contains the js function callback for google when the sign in succeeds.
const fn success() -> &'static str {
    "
    function on_success(user){
        sign_in(user);
    }
    "
}

/// Contains the js function callback for google when the sign in succeeds.
const fn fail() -> &'static str {
    "
    function on_failure(err){
        sign_failed(err);
    }
    "
}

fn display_user_information(user: &Option<GoogleIdentifiedUser>) -> String {
    match user {
        None => "no user connected".to_string(),
        Some(u) => u.name().to_string() + " " + &*u.email().to_string(),
    }
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen]
// `wasm-bindgen` cannot transfer struct with public closures to JS (yet) so we
// have to send slice.
pub fn start() -> Box<[JsValue]> {
    let app = App::start("app", init, update, view);

    create_closures_for_js(&app)
}

/// Closure that triggers messages when getting update from js
fn create_closures_for_js(app: &App<Msg, Model, Node<Msg>>) -> Box<[JsValue]> {
    let sign_in = wrap_in_permanent_closure(enc!((app) move |user| {
        app.update(Msg::SignedIn(user))
    }));
    let sign_failed = wrap_in_permanent_closure(enc!((app) move |err| {
        app.update(Msg::SignedFailed(err))
    }));

    vec![sign_in, sign_failed].into_boxed_slice()
}

/// Make a perma closure
fn wrap_in_permanent_closure<T>(f: impl FnMut(T) + 'static) -> JsValue
where
    T: wasm_bindgen::convert::FromWasmAbi + 'static,
{
    // `Closure::new` isn't in `stable` Rust (yet) - it's a custom implementation
    // from Seed. If you need more flexibility, use `Closure::wrap`.
    let closure = Closure::new(f);
    let closure_as_js_value = closure.as_ref().clone();
    // `forget` leaks `Closure` - we should use it only when
    // we want to call given `Closure` more than once.
    closure.forget();
    closure_as_js_value
}

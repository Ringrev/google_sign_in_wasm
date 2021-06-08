//! Importing into rust the class from the google client
//! Official documentation there https://developers.google.com/identity/sign-in/web/reference#users

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[cfg(feature = "seed")]
pub mod sign_in;

#[wasm_bindgen]
extern "C" {
    pub type GoogleUser;
    #[wasm_bindgen(catch, method)]
    pub fn getBasicProfile(this: &GoogleUser) -> Result<BasicProfile, JsValue>;
    #[wasm_bindgen(catch, method)]
    pub fn getAuthResponse(
        this: &GoogleUser,
        #[allow(non_snake_case)] includeAuthorizationData: bool,
    ) -> Result<AuthResponse, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    pub type AuthResponse;
    #[wasm_bindgen(catch, method, getter)]
    pub fn access_token(this: &AuthResponse) -> Result<String, JsValue>;
    #[wasm_bindgen(catch, method, getter)]
    pub fn id_token(this: &AuthResponse) -> Result<String, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    pub type BasicProfile;
    #[wasm_bindgen(catch, method)]
    pub fn getId(this: &BasicProfile) -> Result<String, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub fn getName(this: &BasicProfile) -> Result<String, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub fn getGivenName(this: &BasicProfile) -> Result<String, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub fn getFamilyName(this: &BasicProfile) -> Result<String, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub fn getImageUrl(this: &BasicProfile) -> Result<String, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub fn getEmail(this: &BasicProfile) -> Result<String, JsValue>;

}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch,js_namespace = ["window", "gapi"])]
    pub fn load(lib: &str, callback: JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, js_namespace = ["window", "gapi","auth2"])]
    pub fn init(config: JsValue) -> Result<GoogleAuth, JsValue>;
    #[wasm_bindgen(catch, js_namespace = ["window", "gapi","auth2"])]
    pub fn authorize(
        config: JsValue,
        callback: &mut dyn FnMut(AuthResponse),
    ) -> Result<GoogleAuth, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    pub type GoogleAuth;
    #[wasm_bindgen(catch, method,js_namespace =  ["window", "gapi","auth2"])]
    pub fn attachClickHandler(
        this: &GoogleAuth,
        container_id: String,
        options: SignInOptions,
        onsuccess: &JsValue,
        onfailure: &mut dyn FnMut(),
    ) -> Result<(), JsValue>;
}

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    pub client_id: String,
    pub cookie_policy: String,
    pub scope: String,
    pub fetch_basic_profile: bool,
    pub hosted_domain: String,
    pub redirect_uri: String,
}
// impl ClientConfig {
//     pub fn new(
//         client_id: &str,
//         scope: &str,
//         cookie_policy: &str,
//         fetch_basic_profile: bool,
//         hosted_domain: &str,
//         redirect_uri: &str,
//     ) -> Self {
//         Self {
//             client_id: client_id.to_string(),
//             cookie_policy: cookie_policy.to_string(),
//             scope: scope.to_string(),
//             fetch_basic_profile,
//             hosted_domain: hosted_domain.to_string(),
//             redirect_uri: redirect_uri.to_string(),
//         }
//     }
// }

#[wasm_bindgen]
pub struct SignInOptions {}

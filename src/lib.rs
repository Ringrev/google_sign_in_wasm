//! Importing into rust the class from the google client
//! Official documentation there https://developers.google.com/identity/sign-in/web/reference#users


use wasm_bindgen::prelude::*;

#[cfg(feature="seed")]
pub mod sign_in;

#[wasm_bindgen]
extern "C" {
    pub type GoogleUser;
    #[wasm_bindgen(catch, method)]
    pub fn getBasicProfile(this: &GoogleUser) -> Result<BasicProfile, JsValue>;
    #[wasm_bindgen(catch, method)]
    pub fn getAuthResponse(
        this: &GoogleUser,
        #[allow(non_snake_case)]
        includeAuthorizationData: bool,
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
  pub  fn getId(this: &BasicProfile) -> Result<String, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub   fn getName(this: &BasicProfile) -> Result<String, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub   fn getGivenName(this: &BasicProfile) -> Result<String, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub     fn getFamilyName(this: &BasicProfile) -> Result<String, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub   fn getImageUrl(this: &BasicProfile) -> Result<String, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub   fn getEmail(this: &BasicProfile) -> Result<String, JsValue>;

}

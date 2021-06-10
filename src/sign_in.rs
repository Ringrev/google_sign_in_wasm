#[cfg(feature = "seed")]
use seed::{prelude::*, *};

/// Function that contain customized button and methods to use google sign in.
/// The button use this example  `https://developers.google.com/identity/sign-in/web/build-button`
/// You need to inject javascript function named on_success &  on_failure.
#[cfg(feature = "seed")]
pub fn default_google_button<Ms>(
    on_success: &str,
    on_failure: &str,
    scope: &str,
    width: &u16,
    height: &u16,
    theme: &str,
) -> Vec<Node<Ms>> {
    vec![
        Script![attrs! {
          At::Src=>"https://apis.google.com/js/platform.js?onload=renderButton",
          At::Async=>true,
          At::Defer=>true,
        }],
        Script![format!(
            "
                      {}

                      {}

                function renderButton() {{
                  gapi.signin2.render('my-signin2', {{
                    'scope': '{}',
                    'width': {},
                    'height': {},
                    'longtitle': true,
                    'theme': '{}',
                    'onsuccess': on_success,
                    'onfailure': on_failure
                   }}
                );
                 }}  ",
            on_success, on_failure, scope, width, height, theme
        )],
        div![
            id!("my-signin2"),
            C!["centered"],
            style! {
                St::MarginBottom=>"40px"
            }
        ],
    ]
}

#[cfg(feature = "seed")]
#[wasm_bindgen(inline_js = "export async function signOut() {
        var auth2 = gapi.auth2.getAuthInstance();
        return auth2.signOut()
  }")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn signOut() -> Result<JsValue, JsValue>;
}

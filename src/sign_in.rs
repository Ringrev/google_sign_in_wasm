#[cfg(feature="seed")]
use seed::{prelude::*, *};

/// Function that contain customized button and methods to use google sign in.
/// Todo could add variables for customizing the button.
#[cfg(feature="seed")]
pub fn button<Ms>() -> Vec<Node<Ms>> {
    vec![
                Script![attrs! {
                  At::Src=>"https://apis.google.com/js/platform.js?onload=renderButton",
                  At::Async=>true,
                  At::Defer=>true,
                }],
                Script![
                    "function onFailure(error) {
                      console.log(error);
                  }
                  function onSuccess(googleUser) {
                        user(googleUser);
                  }
                function renderButton() {
                  gapi.signin2.render('my-signin2', {
                    'scope': 'profile email https://www.googleapis.com/auth/youtube.force-ssl https://www.googleapis.com/auth/youtube.readonly',
                    'width': 350,
                    'height': 80,
                    'longtitle': true,
                    'theme': 'dark',
                    'onsuccess': onSuccess,
                    'onfailure': onFailure
                  },
                );
                }",
                ],
        div![
            id!("my-signin2"),
            C!["centered"],
            style! {
                St::MarginBottom=>"40px"
            }
        ],
    ]
}

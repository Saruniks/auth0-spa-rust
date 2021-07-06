pub mod timeout;

use auth0_spa_rust::Auth0Service;
use futures::executor::block_on;
use yew::{html, web_sys::window, Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::NeqAssign;

use std::{sync::Once, time::Duration};

use self::timeout::Timeout;

static INIT: Once = Once::new();

pub async fn setup() {
    INIT.call_once(|| {
        let window = window().expect("should have a document on window");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");
        let auth0_script_elem = document
            .create_element("script")
            .expect("can't create element");
        auth0_script_elem
            .set_attribute(
                "src",
                r#"https://cdn.auth0.com/js/auth0-spa-js/1.13/auth0-spa-js.production.js"#,
            )
            .expect("can't set attribute");

        let output_elem = document.get_element_by_id("output").unwrap();
        body.insert_before(&auth0_script_elem, body.first_child().as_ref())
            .expect("cant append");
    });
    // Temporal workaround
    Timeout::new(Duration::new(1, 0)).await;
}

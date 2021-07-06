mod model;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use once_cell::sync::OnceCell;

use std::convert::TryFrom;

use lazy_static::lazy_static;
use yew::Callback;

use crate::{
    Auth0Client, Auth0ClientOptions, LogoutOptions,
};

pub use self::model::{Claim, ConfigOptions, User, AuthLogoutOptions};

pub static AUTH0_DOMAIN: OnceCell<String> = OnceCell::new();
pub static AUTH0_CLIENT_ID: OnceCell<String> = OnceCell::new();
pub static AUTH0_REDIRECT_URI: OnceCell<String> = OnceCell::new();
pub static AUTH0_USE_REFRESH_TOKENS: OnceCell<bool> = OnceCell::new();
pub static AUTH0_CACHE_LOCATION: OnceCell<String> = OnceCell::new();

lazy_static! {
    pub static ref AUTH0_SERVICE: Auth0Service = Auth0Service::new();
}

pub struct Auth0Service(Auth0Client);

impl Auth0Service {
    fn new() -> Self {

        let options = ConfigOptions {
            domain: AUTH0_DOMAIN.get().expect("AUTH0_DOMAIN not set").to_string(),
            client_id: AUTH0_CLIENT_ID.get().expect("AUTH0_CLIENT_ID not set").to_string(),
            redirect_uri: AUTH0_REDIRECT_URI.get().expect("AUTH0_REDIRECT_URI not set").to_string(),
            useRefreshTokens: *AUTH0_USE_REFRESH_TOKENS.get().expect("AUTH0_USE_REFRESH_TOKENS not set"),
            cacheLocation: AUTH0_CACHE_LOCATION.get().expect("AUTH0_CACHE_LOCATION not set").to_string(),
        };

        Auth0Service(Auth0Client::new(
            Auth0ClientOptions::try_from(JsValue::from_serde(&options).unwrap()).unwrap(),
        ))
    }

    pub fn login_with_redirect() {
        spawn_local(async move {
            AUTH0_SERVICE.0.login_with_redirect(None).await;
        });
    }

    pub fn login_with_popup() {
        spawn_local(async move {
            AUTH0_SERVICE.0.login_with_popup(None, None).await;
        });
    }

    pub fn handle_redirect_callback(callback: Callback<Result<JsValue, JsValue>>) {
        spawn_local(async move {
            let result = AUTH0_SERVICE.0.handle_redirect_callback(None).await;
            callback.emit(result);
        });
    }

    pub fn get_user(callback: Callback<Option<User>>) {
        spawn_local(async move {
            let user_js = AUTH0_SERVICE.0.get_user(None).await;
            match JsValue::into_serde(&user_js) {
                Ok(user) => {
                    callback.emit(Some(user));
                }
                Err(_) => {
                    callback.emit(None);
                }
            }
        });
    }

    pub fn get_token(callback: Callback<Option<String>>) {
        spawn_local(async move {
            let access_token = AUTH0_SERVICE.0.get_id_token_claims(None).await;

            match JsValue::into_serde::<Claim>(&access_token) {
                Ok(token) => {
                    callback.emit(Some(token.__raw));
                }
                Err(_) => {
                    callback.emit(None);
                }
            }
        });
    }

    pub fn is_authenticated(callback: Callback<bool>) {
        spawn_local(async move {
            let result = AUTH0_SERVICE.0.is_authenticated().await.as_bool().unwrap();
            callback.emit(result);
        });
    }

    pub fn logout() {
        spawn_local(async move {
            let logout_options = AuthLogoutOptions {
                returnTo: AUTH0_REDIRECT_URI.get().expect("AUTH0_REDIRECT_URI not set").to_string(),
            };

            AUTH0_SERVICE.0.logout(Some(
                LogoutOptions::try_from(JsValue::from_serde(&logout_options).unwrap()).unwrap(),
            ));
        });
    }
}
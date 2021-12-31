use std::{collections::HashSet, sync::Mutex};
use crate::{Auth0Service, User};
use wasm_bindgen::prelude::*;

use lazy_static::{__Deref, lazy_static};
use wasm_bindgen::JsValue;
use serde::Deserialize;
use gloo_timers::callback::{Timeout, Interval};
use wasm_bindgen_futures::spawn_local;
use yew_agent::*;

lazy_static! {
    static ref PERMISSIONS: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref USER: Mutex<Option<User>> = Mutex::new(None);
    static ref ACCESS_TOKEN: Mutex<Option<String>> = Mutex::new(None);
}

pub struct PermissionsService;

impl PermissionsService {
    pub fn has_permission(permission: String) -> bool {
        let permissions = PERMISSIONS.lock().unwrap();
        permissions.contains(&permission)
    }

    pub fn get_user() -> Option<User> {
        USER.lock().unwrap().deref().clone()
    }

    pub fn get_access_token() -> Option<String> {
        ACCESS_TOKEN.lock().unwrap().deref().clone()
    }
}

pub enum Msg {
    GetAccessToken(Result<String, JsValue>),
    UserData(Option<User>),
    CheckSession,
    CheckSessionResponse(bool),
}

pub enum Input {
    Start,
}

pub enum Output {
    Initialized,
}

pub struct PermissionsAgent {
    subscribers: HashSet<HandlerId>,
    link: AgentLink<Self>,
    timeout: Timeout,
}

impl Agent for PermissionsAgent {
    type Reach = Context<Self>;

    type Message = Msg;

    type Input = Input;

    type Output = Output;

    fn create(link: AgentLink<Self>) -> Self {

        let callback = link.callback(|()| Msg::CheckSession);
        // Idea: maybe instead of timer, check session only on actions
        let timeout = Timeout::new(500, move || {
            callback.emit(())
        });

        // let update_interval = Interval::new(500, move || link_cloned.callback(|()| Msg::CheckSession));

        log::warn!("BrGr");
        // web_sys::console::log("Rbwadw");

        Self {
            subscribers: HashSet::new(),
            link,
            timeout
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::GetAccessToken(result) => {
                match result {
                    Ok(token) => {
                        *ACCESS_TOKEN.lock().unwrap() = Some(token.clone());
                        self.parse_permissions(token);
                    }
                    Err(_err) => {
                        *ACCESS_TOKEN.lock().unwrap() = None;
                        for id in &self.subscribers {
                            self.link.respond(*id, Output::Initialized);
                        }
                    }
                }
            }
            Msg::UserData(user) => {
                *USER.lock().unwrap() = user;
            }
            Msg::CheckSession => {
                log::warn!("Brrr");
                Auth0Service::is_authenticated(self.link.callback(Msg::CheckSessionResponse));
            }
            Msg::CheckSessionResponse(_is_authenticated) => {
                // Idea: maybe instead of timer, check session only on actions
                let callback = self.link.callback(|()| Msg::CheckSession);
                self.timeout = Timeout::new(500, move || {
                    callback.emit(())
                });
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
        match msg {
            Input::Start => {
                Auth0Service::get_user(self.link.callback(Msg::UserData));
                self.fetch_permissions();
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}


#[derive(Deserialize, Clone, PartialEq, Debug)]
struct Permissions {
    permissions: Vec<String>,
}

impl PermissionsAgent {
    fn fetch_permissions(&self) {
        let link = self.link.clone();
        spawn_local(async move {
            let result = Auth0Service::get_access_token().await;
            link.callback(move |()| Msg::GetAccessToken(result.clone()));
        });
    }

    fn parse_permissions(&self, token: String) {
        let claims = parse_jwt(&token);

        let permissions: Permissions = JsValue::into_serde(&claims).unwrap();

        *PERMISSIONS.lock().unwrap() = permissions.permissions;

        for id in &self.subscribers {
            self.link.respond(*id, Output::Initialized);
        }
    }
}

#[wasm_bindgen(inline_js = r#"
(function (factory) {
    typeof define === 'function' && define.amd ? define(factory) :
    factory();
}((function () { 'use strict';

    /**
     * The code was extracted from:
     * https://github.com/davidchambers/Base64.js
     */

    var chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

    function InvalidCharacterError(message) {
        this.message = message;
    }

    InvalidCharacterError.prototype = new Error();
    InvalidCharacterError.prototype.name = "InvalidCharacterError";

    function polyfill(input) {
        var str = String(input).replace(/=+$/, "");
        if (str.length % 4 == 1) {
            throw new InvalidCharacterError(
                "'atob' failed: The string to be decoded is not correctly encoded."
            );
        }
        for (
            // initialize result and counters
            var bc = 0, bs, buffer, idx = 0, output = "";
            // get next character
            (buffer = str.charAt(idx++));
            // character found in table? initialize bit storage and add its ascii value;
            ~buffer &&
            ((bs = bc % 4 ? bs * 64 + buffer : buffer),
                // and if not first of each 4 characters,
                // convert the first 8 bits to one ascii character
                bc++ % 4) ?
            (output += String.fromCharCode(255 & (bs >> ((-2 * bc) & 6)))) :
            0
        ) {
            // try to find character in table (0-63, not found => -1)
            buffer = chars.indexOf(buffer);
        }
        return output;
    }

    var atob = (typeof window !== "undefined" &&
        window.atob &&
        window.atob.bind(window)) ||
    polyfill;

    function b64DecodeUnicode(str) {
        return decodeURIComponent(
            atob(str).replace(/(.)/g, function(m, p) {
                var code = p.charCodeAt(0).toString(16).toUpperCase();
                if (code.length < 2) {
                    code = "0" + code;
                }
                return "%" + code;
            })
        );
    }

    function base64_url_decode(str) {
        var output = str.replace(/-/g, "+").replace(/_/g, "/");
        switch (output.length % 4) {
            case 0:
                break;
            case 2:
                output += "==";
                break;
            case 3:
                output += "=";
                break;
            default:
                throw "Illegal base64url string!";
        }

        try {
            return b64DecodeUnicode(output);
        } catch (err) {
            return atob(output);
        }
    }

    function InvalidTokenError(message) {
        this.message = message;
    }

    InvalidTokenError.prototype = new Error();
    InvalidTokenError.prototype.name = "InvalidTokenError";

    function jwtDecode(token, options) {
        if (typeof token !== "string") {
            throw new InvalidTokenError("Invalid token specified");
        }

        options = options || {};
        var pos = options.header === true ? 0 : 1;
        try {
            return JSON.parse(base64_url_decode(token.split(".")[pos]));
        } catch (e) {
            throw new InvalidTokenError("Invalid token specified: " + e.message);
        }
    }

    /*
     * Expose the function on the window object
     */

    //use amd or just through the window object.
    if (window) {
        if (typeof window.define == "function" && window.define.amd) {
            window.define("jwt_decode", function() {
                return jwtDecode;
            });
        } else if (window) {
            window.jwt_decode = jwtDecode;
        }
    }

})));
//# sourceMappingURL=jwt-decode.js.map

export function parse_jwt (token) {
    var decoded = jwt_decode(token);
    console.log(decoded);
    return decoded;
};"#)]
extern "C" {
    fn parse_jwt(token: &str) -> JsValue;
}


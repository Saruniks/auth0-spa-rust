mod route;

use yew::{html, services::console::ConsoleService, Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::NeqAssign;
use auth0_spa_rust::{Auth0Service, User};
use wasm_bindgen::prelude::*;
use yew::services::timeout::TimeoutTask;
use yew::services::TimeoutService;
use std::time::Duration;
use auth0_spa_rust::{AUTH0_DOMAIN, AUTH0_CLIENT_ID, AUTH0_REDIRECT_URI, AUTH0_USE_REFRESH_TOKENS, AUTH0_CACHE_LOCATION};

pub struct TestComponent {
    link: ComponentLink<Self>,
    user: Option<User>,
    token: Option<String>,
    is_authenticated: Option<bool>,
    props: Props,
    #[allow(dead_code)]
    timer_job: Option<TimeoutTask>,
}

pub enum Msg {
    LoginWithRedirect,
    LoginWithPopup,
    Logout,
    GetUser(Option<User>),
    GetToken(Result<String, JsValue>),
    IsAuthenticated(bool),
    Refresh,
    HandleRedirectCallback(Result<JsValue, JsValue>),
}

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {}

impl Component for TestComponent {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        match yew::web_sys::window() {
            Some(window) => match window.location().search() {
                Ok(path) => {
                    if path.contains("code=") {
                        ConsoleService::log("path.contains(code)");
                        Auth0Service::handle_redirect_callback(
                            link.callback(Msg::HandleRedirectCallback),
                        );
                    }
                }
                Err(err) => {
                    ConsoleService::log(&format!("window.location().search() error = {:?}", err));
                }
            },
            None => {
                ConsoleService::log(&format!("yew::web_sys::window() is None"));
            }
        }

        Auth0Service::get_user(link.callback(Msg::GetUser));

        let timer_job = Some(TimeoutService::spawn(
            Duration::from_millis(100),
            link.callback(|_| Msg::Refresh),
        ));

        Self { link, props, user: None, token: None, is_authenticated: None, timer_job }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoginWithRedirect => {
                Auth0Service::login_with_redirect();
            }
            Msg::LoginWithPopup => {
                Auth0Service::login_with_popup();
            }
            Msg::Logout => {
                Auth0Service::logout();
            }
            Msg::GetUser(user) => {
                self.user = user;
            }
            Msg::IsAuthenticated(is_authenticated) => {
                self.is_authenticated = Some(is_authenticated);
            }
            Msg::GetToken(res) => {
                match res {
                    Ok(token) => {
                        self.token = Some(token);
                    }
                    Err(err) => {
                        ConsoleService::log(&format!("GetToken err = {:?}", err));
                        self.token = None;
                    }
                }
            }
            Msg::Refresh => {
                Auth0Service::is_authenticated(self.link.callback(Msg::IsAuthenticated));
                Auth0Service::get_user(self.link.callback(Msg::GetUser));
                Auth0Service::get_token(self.link.callback(Msg::GetToken));

                self.timer_job = Some(TimeoutService::spawn(
                    Duration::from_millis(100),
                    self.link.callback(|_| Msg::Refresh),
                ));
            }
            Msg::HandleRedirectCallback(Ok(_)) => {
                ConsoleService::log("HandleRedirectCallback success");
            }
            Msg::HandleRedirectCallback(Err(err)) => {
                ConsoleService::error(&format!("{:?}", err));
            }
        }

        true
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }
    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <button id="login-with-redirect" onclick=self.link.callback(|_| Msg::LoginWithRedirect)>{ "Login with redirect" }</button>
                </div>
                <div>
                    <button id="login-with-popup" onclick=self.link.callback(|_| Msg::LoginWithPopup)>{ "Login with popup" }</button>
                </div>
                <div>
                    <button id="logout" onclick=self.link.callback(|_| Msg::Logout)>{ "Logout" }</button>
                </div>
                <div>
                    <button id="refresh" onclick=self.link.callback(|_| Msg::Refresh)>{ "Refresh" }</button>
                </div>
                <div>
                    <p>{"IsAuthenticated:"} {format!("{:?}", self.is_authenticated)}</p>
                    <p>{"User:"} {format!("{:?}", self.user)}</p>
                    <p style="overflow-wrap: break-word; max-width: 70ch;">{"Token:"} {format!("{:?}", self.token)}</p>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

fn main() {
    AUTH0_DOMAIN.set("vendenic.eu.auth0.com".to_string()).expect("Couldn't set AUTH0_DOMAIN");
    AUTH0_CLIENT_ID.set("eN3jUJzJAsaCmygamUrGKKeTjLQm4yIb".to_string()).expect("Couldn't set AUTH0_CLIENT_ID");
    AUTH0_REDIRECT_URI.set("http://localhost:8000".to_string()).expect("Couldn't set AUTH0_REDIRECT_URI");
    AUTH0_USE_REFRESH_TOKENS.set(false).expect("Couldn't set AUTH0_USE_REFRESH_TOKENS");
    AUTH0_CACHE_LOCATION.set("localstorage".to_string()).expect("Couldn't set AUTH0_CACHE_LOCATION");
    
    yew::start_app::<TestComponent>();
}

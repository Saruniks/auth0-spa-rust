mod route;

use yew::{Component, Html, Properties, html};
use auth0_spa_rust::{Auth0Service, User, permissions::{Input, Output, PermissionsAgent}};
use wasm_bindgen::prelude::*;
use auth0_spa_rust::{AUTH0_DOMAIN, AUTH0_CLIENT_ID, AUTH0_REDIRECT_URI, AUTH0_USE_REFRESH_TOKENS, AUTH0_CACHE_LOCATION};
use gloo_timers::callback::Timeout;
use yew_agent::{Bridge, Bridged};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub struct TestComponent {
    user: Option<User>,
    token: Option<String>,
    is_authenticated: Option<bool>,
    #[allow(dead_code)]
    timer_job: Option<Timeout>,
    permissions_agent: Box<dyn Bridge<PermissionsAgent>>,
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
    PermissionsInitialized,
}

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {}

impl Component for TestComponent {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {

        let mut permissions_agent = PermissionsAgent::bridge(ctx.link().callback(|msg| match msg {
            Output::Initialized => Msg::PermissionsInitialized,
        }));

        permissions_agent.send(Input::Start);

        match web_sys::window() {
            Some(window) => match window.location().search() {
                Ok(path) => {
                    if path.contains("code=") {
                        Auth0Service::handle_redirect_callback(
                            ctx.link().callback(Msg::HandleRedirectCallback),
                        );
                    }
                }
                Err(err) => {
                }
            },
            None => {
            }
        }

        Auth0Service::get_user(ctx.link().callback(Msg::GetUser));

        let link_cloned = ctx.link().clone();
        let timer_job = Some(Timeout::new(100, move || {
            link_cloned.callback(|()| Msg::Refresh);
        }));

        Self { user: None, token: None, is_authenticated: None, timer_job, permissions_agent }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                        self.token = None;
                    }
                }
            }
            Msg::Refresh => {
                Auth0Service::is_authenticated(ctx.link().callback(Msg::IsAuthenticated));
                Auth0Service::get_user(ctx.link().callback(Msg::GetUser));

                let link = ctx.link().clone();
                spawn_local(async move {
                    let result = Auth0Service::get_access_token().await;
                    link.callback(move |()| Msg::Refresh);
                });

                let link = ctx.link().clone();
                self.timer_job = Some(Timeout::new(100, move || {
                    link.callback(|()| Msg::Refresh);
                }));        
            }
            Msg::HandleRedirectCallback(Ok(_)) => {
            }
            Msg::HandleRedirectCallback(Err(err)) => {
            }
            Msg::PermissionsInitialized => {
            }
        }

        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div>
                    <button id="login-with-redirect" onclick={ctx.link().callback(|_| Msg::LoginWithRedirect)}>{ "Login with redirect" }</button>
                </div>
                <div>
                    <button id="login-with-popup" onclick={ctx.link().callback(|_| Msg::LoginWithPopup)}>{ "Login with popup" }</button>
                </div>
                <div>
                    <button id="logout" onclick={ctx.link().callback(|_| Msg::Logout)}>{ "Logout" }</button>
                </div>
                <div>
                    <button id="refresh" onclick={ctx.link().callback(|_| Msg::Refresh)}>{ "Refresh" }</button>
                </div>
                <div>
                    <p>{"IsAuthenticated:"} {format!("{:?}", self.is_authenticated)}</p>
                    <p>{"User:"} {format!("{:?}", self.user)}</p>
                    <p style="overflow-wrap: break-word; max-width: 70ch;">{"Token:"} {format!("{:?}", self.token)}</p>
                </div>
            </div>
        }
    }
}

fn main() {
    AUTH0_DOMAIN.set("vendenic.eu.auth0.com".to_string()).expect("Couldn't set AUTH0_DOMAIN");
    AUTH0_CLIENT_ID.set("eN3jUJzJAsaCmygamUrGKKeTjLQm4yIb".to_string()).expect("Couldn't set AUTH0_CLIENT_ID");
    AUTH0_REDIRECT_URI.set("http://localhost:8000".to_string()).expect("Couldn't set AUTH0_REDIRECT_URI");
    AUTH0_USE_REFRESH_TOKENS.set(false).expect("Couldn't set AUTH0_USE_REFRESH_TOKENS");
    AUTH0_CACHE_LOCATION.set("localstorage".to_string()).expect("Couldn't set AUTH0_CACHE_LOCATION");
    
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<TestComponent>();
}

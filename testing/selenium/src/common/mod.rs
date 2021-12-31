use auth0_spa_rust::Auth0Service;
use futures::executor::block_on;
use yewtil::NeqAssign;
use yew::prelude::*;

use std::{sync::Once, time::Duration};

pub struct TestComponent {
}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {}

impl Component for TestComponent {
    type Message = Msg;
    type Properties = Props;
    fn create(_ctx: &Context<Self>) -> Self {
        Auth0Service::login_with_popup();

        Self { }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {}
    }
}

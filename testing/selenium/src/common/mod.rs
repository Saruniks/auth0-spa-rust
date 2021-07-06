use auth0_spa_rust::Auth0Service;
use futures::executor::block_on;
use yew::{html, web_sys::window, Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::NeqAssign;

use std::{sync::Once, time::Duration};

pub struct TestComponent {
    props: Props,
}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {}

impl Component for TestComponent {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Auth0Service::login_with_popup();

        Self { props }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }
    fn view(&self) -> Html {
        html! {}
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

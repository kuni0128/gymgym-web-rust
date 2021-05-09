use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::app::components::{header::Header};

pub struct App {
    link: ComponentLink<Self>,
    value: i64,
}

pub enum Msg {
    AddOne,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Header />
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

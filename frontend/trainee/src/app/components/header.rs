use yew::{ComponentLink, Component, ShouldRender, Html, html, Properties};

pub struct Header {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>{ "header works!" }</div>
        }
    }
}

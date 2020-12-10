use yew::prelude::*;
use yew::services::ConsoleService;

pub struct App {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                ConsoleService::log("print in js console!");
            },
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="app">
                    <p>
                        { "this is  " } <code>{ "yew & parcel" }</code> { " starter!" }
                    </p>
                    <button onclick=self.link.callback(|_|Msg::Click)>{"run js"}</button>
            </div>
        }
    }
}

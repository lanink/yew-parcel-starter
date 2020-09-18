use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

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
            Msg::Click => run_js_fun(),
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

// use js_call_back
#[wasm_bindgen]
extern "C" {
    fn run_js_fun();
}

use yew::prelude::*;

pub struct Model {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <div class="overflow-hidden h-2 mb-4 text-xs flex rounded bg-gray-200">
            <div style="width: 10%" class="shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-red-500"></div>
            <div style="width: 15%" class="shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-green-500"></div>
            <div style="width: 25%" class="shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-blue-500"></div>
        </div>
        }
    }
}

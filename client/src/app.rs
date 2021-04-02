use yew::prelude::*;

use crate::counter::Model as Counter;
use crate::nav::Model as Nav;
use crate::progressbar::Model as Progressbar;
use crate::queue::Model as Queue;
use crate::table::Model as Table;
use crate::wsding::Model as WSDing;

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
        <>
            <Nav />
            <Progressbar />
            <main class="grid grid-cols-2">
            <div>
                <Queue />
                <section>
                    <Counter />
                    <Counter />
                    <Counter />
                    <Counter />
                </section>
                <Table />
            </div>
            <div>
                <Table />
                <WSDing />
            </div>

            </main>
        </>
                }
    }
}

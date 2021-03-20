use yew::prelude::*;

use crate::counter::Model as Counter;
use crate::header::Model as Header;
use crate::nav::Model as Nav;
use crate::table::Model as Table;

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
            <Header />
            <main>


            <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">

            <div class="px-4 py-4 sm:px-0">
              <div class="border-4 border-dashed border-gray-200 rounded-lg">

                <section>

                <Counter />
                <Counter />
                <Counter />
                <Counter />

                </section>

              </div>
            </div>

          </div>

          <div class="flex flex-col">
          <div class="-my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
          <div class="py-2 align-middle inline-block min-w-full sm:px-6 lg:px-8">

              <Table />

              <Table />

          </div>
          </div>
          </div>

            </main>
            </>
        }
    }
}

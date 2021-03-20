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
          <nav class="bg-gray-500">
          <div class="mx-auto px-4 py-2">
            <div class="flex justify-between w-full">
                <div class="ml-4 flex space-x-2">
                <img class="h-10" src="/static/dinxperfm-logo.png" alt="DinxperFM" />
                  <button class="bg-blue-500 hover:bg-blue-700 text-white p-2 rounded h-10 w-10">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                  </button>
                  <button class="bg-blue-500 hover:bg-blue-700 text-white p-2 rounded h-10 w-10">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                  </button>
                  <button class="bg-blue-500 hover:bg-blue-700 text-white p-2 rounded h-10 w-10">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 9v6m4-6v6m7-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                  </button>
                  <button class="bg-blue-500 hover:bg-blue-700 text-white p-2 rounded h-10 w-10">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 10a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 01-1-1v-4z" />
                    </svg>
                  </button>
              </div>

              <div class="flex flex-col ml-4 text-green-900">
                <div>{"Queen"}</div>
                <div class="font-bold">{"Fat bottom girls (make the world go round)"}</div>
              </div>

              <div class="flex w-96 flex-shrink-0 ml-4">
                  <div class="flex flex-col text-red-900 w-1/4">
                    <div>{"Intro"}</div>
                    <div class="font-bold">{"--:--"}</div>
                  </div>

                  <div class="flex flex-col text-orange-900 w-1/4">
                    <div>{"Outro"}</div>
                    <div class="font-bold">{"--:--:--"}</div>
                  </div>

                  <div class="flex flex-col text-white w-1/4">
                    <div>{"Remaining"}</div>
                    <div class="font-bold">{"00:00:51"}</div>
                  </div>

                  <div class="flex flex-col text-green-900 w-1/4">
                    <div>{"Clock"}</div>
                    <div class="font-bold">{"22:28:26"}</div>
                  </div>
              </div>
            </div>
          </div>
          </nav>
        }
    }
}

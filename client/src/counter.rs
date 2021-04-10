// use yew::prelude::*;

// pub struct Model {
//     link: ComponentLink<Self>,
//     value: i64,
// }

// pub enum Msg {
//     AddOne,
//     SubstractOne,
//     Reset,
// }

// impl Component for Model {
//     type Message = Msg;
//     type Properties = ();
//     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
//         Self { link, value: 0 }
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         match msg {
//             Msg::AddOne => self.value += 1,
//             Msg::SubstractOne => self.value -= 1,
//             Msg::Reset => self.value = 0,
//         }
//         true
//     }

//     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//         // Should only return "true" if new properties are different to
//         // previously received properties.
//         // This component has no properties so we will always return "false".
//         false
//     }

//     fn view(&self) -> Html {
//         html! {
//             <div class="p-1 inline-block">
//             <p class="text-white p-1">{ format!("Counter: {}", self.value) }</p>
//             <div class="inline-flex">
//                 <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
//                 <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick=self.link.callback(|_| Msg::SubstractOne)>{ "-1" }</button>
//                 <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick=self.link.callback(|_| Msg::Reset)>{ "reset" }</button>
//             </div>
//             </div>
//         }
//     }
// }

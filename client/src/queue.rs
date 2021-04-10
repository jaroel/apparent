// use yew::prelude::*;

// #[derive(Clone, PartialEq, Properties)]
// pub struct RowProps {
//     pub position: String,
//     pub song: String,
//     pub artist: String,
//     pub album: String,
// }

// pub struct RowModel {
//     link: ComponentLink<Self>,
//     props: RowProps,
// }

// pub enum RowMsg {}

// impl Component for RowModel {
//     type Message = RowMsg;
//     type Properties = RowProps;
//     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
//         Self { link, props }
//     }

//     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
//         true
//     }

//     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//         false
//     }

//     fn view(&self) -> Html {
//         html! {
//             <div class="flex hover:bg-green-900">
//                 <div class="flex justify-center items-center w-8 min-h-full flex-shrink-0 bg-green-200 text-white">{&self.props.position}</div>
//                 <div class="px-2 w-full">
//                     <div>{{&self.props.song}}</div>
//                     <div>{{&self.props.artist}}</div>
//                     <div>{{&self.props.album}}</div>
//                     <div>{"--"}</div>
//                 </div>
//                 <div class="flex flex-shrink-0 flex-row">
//                     <div class="mx-1 w-16 flex flex-col justify-between items-center">
//                         <button class="flex h-10 w-full items-center justify-center bg-blue-500 hover:bg-blue-700 rounded text-green-500">
//                             <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-8">
//                                 <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
//                                 <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
//                             </svg>
//                         </button>
//                         <div class="text-green-900">{"22:29:18"}</div>
//                     </div>
//                     <div class="mx-1 w-16 flex flex-col justify-between items-center">
//                         <button class="flex h-10 w-full items-center justify-center bg-blue-500 hover:bg-blue-700 rounded text-orange-500">{"V"}</button>
//                         <div class="text-white">{"00:00"}</div>
//                     </div>
//                     <div class="mx-1 w-16 flex flex-col justify-between items-center">
//                         <button class="flex h-10 w-full items-center justify-center bg-blue-500 hover:bg-blue-700 rounded text-red-900">
//                             <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-8">
//                                 <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
//                                 <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 10a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 01-1-1v-4z" />
//                             </svg>
//                         </button>
//                         <div class="text-white">{"00:06:46"}</div>
//                     </div>
//                 </div>
//             </div>
//         }
//     }
// }

// pub struct Model {
//     link: ComponentLink<Self>,
// }

// pub enum Msg {}

// impl Component for Model {
//     type Message = Msg;
//     type Properties = ();
//     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
//         Self { link }
//     }

//     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
//         true
//     }

//     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//         false
//     }

//     fn view(&self) -> Html {
//         html! {
//             <>
//             <RowModel position="1" song="zing" artist="Jos Henken" album="analbumoflove" />
//             <RowModel position="2" song="zing" artist="Jos Henken" album="analbumoflove" />
//             <RowModel position="3" song="zing" artist="Jos Henken" album="analbumoflove" />
//             <RowModel position="4" song="zing" artist="Jos Henken" album="analbumoflove" />
//             <RowModel position="5" song="zing" artist="Jos Henken" album="analbumoflove" />
//             </>
//         }
//     }
// }

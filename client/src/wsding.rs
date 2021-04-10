// use anyhow::Error;
// use serde_derive::{Deserialize, Serialize};
// // use wasm_bindgen::JsValue;
// use yew::format::Json;
// // use yew::services::fetch::{FetchService, FetchTask, Request, Response};
// use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
// use yew::{html, Component, ComponentLink, Html, ShouldRender};

// use web_sys::console;

// type AsBinary = bool;

// // pub enum Format {
// //     Json,
// // }

// pub enum WsAction {
//     Connect,
//     SendData(AsBinary),
//     Disconnect,
//     Lost,
// }

// pub enum Msg {
//     WsAction(WsAction),
//     WsReady(Result<WsResponse, Error>),
// }

// impl From<WsAction> for Msg {
//     fn from(action: WsAction) -> Self {
//         Msg::WsAction(action)
//     }
// }

// // /// This type is used to parse data from `./static/data.json` file and
// // /// have to correspond the data layout from that file.
// // #[derive(Deserialize, Debug)]
// // pub struct DataFromFile {
// //     value: u32,
// // }

// /// This type is used as a request which sent to websocket connection.
// #[derive(Serialize, Debug)]
// struct WsRequest {
//     value: u32,
// }

// /// This type is an expected response from a websocket connection.
// #[derive(Deserialize, Debug)]
// pub struct WsResponse {
//     value: Option<u32>,
// }

// pub struct Model {
//     link: ComponentLink<Model>,
//     data: Option<u32>,
//     ws: Option<WebSocketTask>,
// }

// impl Model {
//     fn view_data(&self) -> Html {
//         if let Some(value) = self.data {
//             html! {
//                 <p>{ value }</p>
//             }
//         } else {
//             html! {
//                 <p>{ "Data hasn't fetched yet." }</p>
//             }
//         }
//     }
// }

// impl Component for Model {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
//         Self {
//             link,
//             data: None,
//             ws: None,
//         }
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         match msg {
//             Msg::WsAction(action) => match action {
//                 WsAction::Connect => {
//                     let callback = self.link.callback(|Json(data)| Msg::WsReady(data));
//                     let notification = self.link.batch_callback(|status| match status {
//                         WebSocketStatus::Opened => vec![],
//                         WebSocketStatus::Closed | WebSocketStatus::Error => {
//                             vec![WsAction::Lost.into()]
//                         }
//                     });
//                     let task = WebSocketService::connect(
//                         "ws://localhost:8000/ws/",
//                         callback,
//                         notification,
//                     )
//                     .unwrap();
//                     self.ws = Some(task);
//                     true
//                 }
//                 WsAction::SendData(binary) => {
//                     let request = WsRequest { value: 321 };
//                     if binary {
//                         self.ws.as_mut().unwrap().send_binary(Json(&request));
//                     } else {
//                         self.ws.as_mut().unwrap().send(Json(&request));
//                     }
//                     false
//                 }
//                 WsAction::Disconnect => {
//                     self.ws.take();
//                     true
//                 }
//                 WsAction::Lost => {
//                     self.ws = None;
//                     true
//                 }
//             },
//             Msg::WsReady(response) => {
//                 unsafe {
//                     console::log_1(&"hier".into());
//                 }

//                 match response {
//                     Ok(ws_response) => {
//                         unsafe {
//                             console::log_1(&ws_response.value.into());
//                         }
//                         self.data = ws_response.value;
//                     }
//                     Err(error) => {
//                         unsafe {
//                             let kek = format!("{}", error);
//                             console::log_1(&kek.into());
//                         }
//                         self.data = None;
//                     }
//                 }

//                 // let js: JsValue = 4.into();
//                 // console::log_2(&"Logging arbitrary values looks like".into(), &js);
//                 // self.data = response.map(|data| data.value).ok();
//                 true
//             }
//         }
//     }

//     fn change(&mut self, _: Self::Properties) -> ShouldRender {
//         false
//     }

//     fn view(&self) -> Html {
//         html! {
//             <div class="bg-white space-x-2">
//                     { self.view_data() }
//                     <button class="bg-blue-500" disabled=self.ws.is_some()
//                             onclick=self.link.callback(|_| WsAction::Connect)>
//                         { "Connect To WebSocket" }
//                     </button>
//                     <button class="bg-blue-500" disabled=self.ws.is_none()
//                             onclick=self.link.callback(|_| WsAction::SendData(false))>
//                         { "Send To WebSocket" }
//                     </button>
//                     <button class="bg-blue-500" disabled=self.ws.is_none()
//                             onclick=self.link.callback(|_| WsAction::SendData(true))>
//                         { "Send To WebSocket [binary]" }
//                     </button>
//                     <button class="bg-blue-500" disabled=self.ws.is_none()
//                             onclick=self.link.callback(|_| WsAction::Disconnect)>
//                         { "Close WebSocket connection" }
//                     </button>
//             </div>
//         }
//     }
// }

#![allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
type Model = i32;

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        C!["bg-gray-600"],
        view_nav(),
        view_progressbar(),
        main![
            C!["grid grid-cols-2"],
            div![
                view_queue_item(
                    1,
                    "zing".into(),
                    "Jos Henken".into(),
                    "analbumoflove".into()
                ),
                view_queue_item(
                    2,
                    "zing".into(),
                    "Jos Henken".into(),
                    "analbumoflove".into()
                ),
                view_queue_item(
                    3,
                    "zing".into(),
                    "Jos Henken".into(),
                    "analbumoflove".into()
                ),
                view_queue_item(
                    4,
                    "zing".into(),
                    "Jos Henken".into(),
                    "analbumoflove".into()
                ),
                view_queue_item(
                    5,
                    "zing".into(),
                    "Jos Henken".into(),
                    "analbumoflove".into()
                ),
            ],
            div![
                C!["counter"],
                "This is a counter: ",
                button![model, ev(Ev::Click, |_| Msg::Increment),],
                view_tabel()
            ],
        ],
    ]
}

// fn view(&self) -> Html {
//     html! {
//     <>
//         <Nav />
//         <Progressbar />
//         <main C!["grid grid-cols-2"]
//         div![
//             <Queue />
//             <section>
//                 <Counter />
//                 <Counter />
//                 <Counter />
//                 <Counter />
//             </section>
//             <Table />
//         ]
//         div![
//             <Table />
//             <WSDing />
//         ]

//         </main>
//     </>
//             }
// }

pub fn start() {
    App::start("app", init, update, view);
}

fn view_nav() -> Node<Msg> {
    div![nav![
        C!["bg-gray-500"],
        div![
            C!["mx-auto px-4 py-2"],
            div![
                C!["flex justify-between w-full"],
                div![
                    C!["ml-4 flex space-x-2"],
                    img![
                        C!["h-10"],
                        attrs! {At::Src => "/dinxperfm-logo.png", At::Alt =>"DinxperFM" }
                    ],
                    button![
                        C!["bg-blue-500 hover:bg-blue-700 text-white p-2 rounded h-10 w-10"],
                        svg![
                            attrs! {At::Fill => "none", At::ViewBox => "0 0 24 24", At::Stroke => "currentColor"},
                            path![attrs! {
                              At::StrokeLinecap=>"round",
                              At::StrokeLineJoin=>"round",
                              At::StrokeWidth=>"2",
                              At::D => "M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z",
                            }],
                            path![attrs! {
                              At::StrokeLinecap=>"round",
                              At::StrokeLineJoin=>"round",
                              At::StrokeWidth=>"2",
                              At::D => "M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                            }],
                        ]
                    ],
                    button![
                        C!["bg-blue-500 hover:bg-blue-700 text-white p-2 rounded h-10 w-10"],
                        svg![
                            attrs! {At::Fill => "none", At::ViewBox => "0 0 24 24", At::Stroke => "currentColor"},
                            path![attrs! {
                              At::StrokeLinecap=>"round",
                              At::StrokeLineJoin=>"round",
                              At::StrokeWidth=>"2",
                              At::D => "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
                            }]
                        ]
                    ],
                    button![
                        C!["bg-blue-500 hover:bg-blue-700 text-white p-2 rounded h-10 w-10"],
                        svg![
                            attrs! {At::Fill => "none", At::ViewBox => "0 0 24 24", At::Stroke => "currentColor"},
                            path![attrs! {
                              At::StrokeLinecap=>"round",
                              At::StrokeLineJoin=>"round",
                              At::StrokeWidth=>"2",
                              At::D => "M10 9v6m4-6v6m7-3a9 9 0 11-18 0 9 9 0 0118 0z",
                            }]
                        ]
                    ],
                    button![
                        C!["bg-blue-500 hover:bg-blue-700 text-white p-2 rounded h-10 w-10"],
                        svg![
                            attrs! {At::Fill => "none", At::ViewBox => "0 0 24 24", At::Stroke => "currentColor"},
                            path![attrs! {
                              At::StrokeLinecap=>"round",
                              At::StrokeLineJoin=>"round",
                              At::StrokeWidth=>"2",
                              At::D => "M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                            }],
                            path![attrs! {
                              At::StrokeLinecap=>"round",
                              At::StrokeLineJoin=>"round",
                              At::StrokeWidth=>"2",
                              At::D => "M9 10a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 01-1-1v-4z",
                            }]
                        ]
                    ]
                ],
                div![
                    C!["flex flex-col ml-4 text-green-900"],
                    div!["Queen"],
                    div![
                        C!["font-bold"],
                        "Fat bottom girls (make the world go round)"
                    ]
                ],
                div![
                    C!["flex w-96 flex-shrink-0 ml-4"],
                    div![
                        C!["flex flex-col text-red-900 w-1/4"],
                        div!["Intro"],
                        div![C!["font-bold"], "--:--"]
                    ]
                ],
                div![
                    C!["flex flex-col text-orange-900 w-1/4"],
                    div!["Outro"],
                    div![C!["font-bold"], "--:--:--"],
                ],
                div![
                    C!["flex flex-col text-white w-1/4"],
                    div!["Remaining"],
                    div![C!["font-bold"], "00:00:51"],
                ],
                div![
                    C!["flex flex-col text-green-900 w-1/4"],
                    div!["Clock"],
                    div![C!["font-bold"], "22:28:26"],
                ],
            ]
        ]
    ]]
}

fn view_progressbar() -> Node<Msg> {
    div![C!["overflow-hidden h-2 mb-4 text-xs flex rounded bg-gray-200"],
        div![C!["shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-red-500"],
        style!{
            St::Width => "10%",
        }],
        div![C!["shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-green-500"],
        style!{
            St::Width => "15%",
        }],
        div![C!["shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-blue-500"],
        style!{
            St::Width => "25%",
        }],
    ]
}

fn view_queue_item(position: i8, song: String, artist: String, album: String) -> Node<Msg> {
    div![ C!["flex hover:bg-green-900"],
        div![ C!["flex justify-center items-center w-8 min-h-full flex-shrink-0 bg-green-200 text-white"],
            position.to_string()],
        div![ C!["px-2 w-full"],
            div![song],
            div![artist],
            div![album],
            div!["--"]
        ],
        div![ C!["flex flex-shrink-0 flex-row"],
            div![ C!["mx-1 w-16 flex flex-col justify-between items-center"],
                button![ C!["flex h-10 w-full items-center justify-center bg-blue-500 hover:bg-blue-700 rounded text-green-500"],
                    svg![C!["w-8"],
                        attrs! {At::Fill => "none", At::ViewBox => "0 0 24 24", At::Stroke => "currentColor"},
                        path![attrs! {
                            At::StrokeLinecap=>"round",
                            At::StrokeLineJoin=>"round",
                            At::StrokeWidth=>"2",
                            At::D => "M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z",
                        }],
                        path![attrs! {
                            At::StrokeLinecap=>"round",
                            At::StrokeLineJoin=>"round",
                            At::StrokeWidth=>"2",
                            At::D => "M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                        }]
                    ]
                ],
                div![ C!["text-green-900"], "22:29:18"]
            ],
            div![ C!["mx-1 w-16 flex flex-col justify-between items-center"],
                button![ C!["flex h-10 w-full items-center justify-center bg-blue-500 hover:bg-blue-700 rounded text-orange-500"],
                    "V"],
                div![ C!["text-white"],
                "00:00"]
            ],
            div![ C!["mx-1 w-16 flex flex-col justify-between items-center"],
                button![C!["flex h-10 w-full items-center justify-center bg-blue-500 hover:bg-blue-700 rounded text-red-900"],
                    svg![
                        C!["w-8"],
                        attrs! {At::Fill => "none", At::ViewBox => "0 0 24 24", At::Stroke => "currentColor"},
                        path![attrs! {
                            At::StrokeLinecap=>"round",
                            At::StrokeLineJoin=>"round",
                            At::StrokeWidth=>"2",
                            At::D => "M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                        }],
                        path![attrs! {
                            At::StrokeLinecap=>"round",
                            At::StrokeLineJoin=>"round",
                            At::StrokeWidth=>"2",
                            At::D => "M9 10a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 01-1-1v-4z",
                        }],
                    ]
                ],
                div![ C!["text-white"],
                    "00:06:46"]
            ]
        ]
    ]
}

fn view_tabel() -> Node<Msg> {
    div![C!["shadow overflow-hidden border-b border-gray-200 sm:rounded-lg"],
        table![C!["min-w-full divide-y divide-gray-200"],
            thead![C!["bg-gray-50"],
                tr![
                    th![C!["px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"], attrs!{At::Scope=>"col"},
                        "Artist"],
                    th![C!["px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"], attrs!{At::Scope=>"col"},
                        "Track"],
                    th![C!["px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"], attrs!{At::Scope=>"col"},
                        "Length"],
                    th![C!["relative px-6 py-3"], attrs!{At::Scope=>"col"},
                        span![ C!["sr-only"],"Queue"]]
                    ],
            ],
            tbody![
                tr![C!["bg-white"],
                    td![C!["px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900"],
                        "Steppenwolf"],
                    td![C!["px-6 py-4 whitespace-nowrap text-sm text-gray-500"],
                        "Born to be wild"],
                    td![C!["px-6 py-4 whitespace-nowrap text-sm text-gray-500"],
                        "2:34"],
                    td![C!["px-6 py-4 whitespace-nowrap text-right text-sm font-medium"],
                        a![attrs!{At::Href=>"#"}, C!["text-indigo-600 hover:text-indigo-900"],
                            "Queue"]],
                ],
                tr![ C!["bg-gray-50"],
                    td![ C!["px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900"],
                        "Cody Fisher"],
                    td![ C!["px-6 py-4 whitespace-nowrap text-sm text-gray-500"],
                        "Product Directives Officer"],
                    td![ C!["px-6 py-4 whitespace-nowrap text-sm text-gray-500"],
                        "cody.fisher@example.com"],
                    td![ C!["px-6 py-4 whitespace-nowrap text-right text-sm font-medium"],
                        a![C!["text-indigo-600 hover:text-indigo-900"], attrs!{At::Href=>"#" },
                            "Queue"]]
                ]
            ]
        ]
    ]
}

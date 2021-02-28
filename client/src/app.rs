#![allow(unused_braces)]
use mogwai::prelude::*;

// mod api;
// mod components;
// mod page;
// mod store;
// mod widgets;

// use components::{login::Login, nav::Nav, register::Register};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// pub enum Page {
//     // Login(Gizmo<Login>),
// // Register(Gizmo<Register>),
// }

struct MyStruct {
    title: String,
}

impl Component for MyStruct {
    fn update(
        &mut self,
        _msg: &AppModel,
        _tx_view: &Transmitter<AppView>,
        _sub: &Subscriber<AppModel>,
    ) {
    }

    fn view(&self, _: &Transmitter<AppModel>, _rx: &Receiver<AppView>) -> ViewBuilder<HtmlElement> {
        let title = self.title;
        builder! {
            <p>{title}</p>
        }
    }
}

pub struct App {
    // nav: Gizmo<Nav>,
}

impl Default for App {
    fn default() -> App {
        // let nav = Gizmo::from(Nav::default());
        // App { nav }
        App {}
    }
}

#[derive(Clone)]
pub enum AppModel {}

#[derive(Clone)]
pub enum AppView {
    // NewPage { page: View<HtmlElement> },
}

impl Component for App {
    type ModelMsg = AppModel;
    type ViewMsg = AppView;
    type DomNode = HtmlElement;

    // fn bind(&self, sub: &Subscriber<AppModel>) {
    //     // // bind the nav's output view messages to our input model messages
    //     // sub.subscribe_filter_map(&self.nav.recv, |msg| {
    //     //     msg.route()
    //     //         .map(|r| AppModel::HashChange { route: r.clone() })
    //     // });
    // }

    // fn update(&mut self, msg: &AppModel, tx: &Transmitter<AppView>, _sub: &Subscriber<AppModel>) {
    //     // match msg {
    //     //     AppModel::HashChange { route } => {
    //     //         let page = View::from(route);
    //     //         tx.send(&AppView::NewPage {
    //     //             page,
    //     //             route: route.clone(),
    //     //         })
    //     //     }
    //     // }
    // }

    fn bind(&self, _input_sub: &Subscriber<AppModel>, _output_sub: &Subscriber<AppView>) {}

    fn update(
        &mut self,
        _msg: &AppModel,
        _tx_view: &Transmitter<AppView>,
        _sub: &Subscriber<AppModel>,
    ) {
    }

    fn view(&self, _: &Transmitter<AppModel>, _rx: &Receiver<AppView>) -> ViewBuilder<HtmlElement> {
        builder! {
            <slot
            // patch:children=rx.branch_filter_map(|msg| match msg {
                // AppView::NewPage{ page, .. } => Some(Patch::Replace{ index: 1, value: page.clone() }),
            // })
            >
                // {self.nav.view_builder()}

                // This node gets replaced every time we send a patch from the parent node ^
                // {ViewBuilder::from(&self.nav.state_ref().current_route)}

                <footer>
                    <div class="container">
                        <a href="/" class="logo-font">"conduit"</a>
                        <span class="attribution">
                            "An interactive learning project from "
                            <a href="https://thinkster.io">"Thinkster"</a>". "
                            "Code & design licensed under MIT."
                        </span>
                    </div>
                </footer>
            </slot>
        }
    }
}

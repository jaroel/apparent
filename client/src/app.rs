use dominator::{class, clone, events, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use lazy_static::lazy_static;
use std::sync::Arc;

pub struct State {
    counter: Mutable<i32>,
}

impl State {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            counter: Mutable::new(0),
        })
    }

    pub fn render(state: Arc<Self>) -> Dom {
        // Define CSS styles
        lazy_static! {
            static ref ROOT_CLASS: String = class! {
                .style("display", "inline-block")
                // .style("background-color", "black")
                .style("padding", "10px")
            };
            static ref TEXT_CLASS: String = class! {
                .style("color", "white")
                .style("font-weight", "bold")
            };
            static ref BUTTON_CLASS: String = class! {
                .style("display", "block")
                .style("width", "100px")
                .style("margin", "5px")
            };
        }

        let btn = [
            "bg-blue-500",
            "hover:bg-blue-700",
            "text-white",
            "font-bold",
            "py-2",
            "px-4",
            "rounded",
        ];

        // Create the DOM nodes
        html!("div", {
            .class(&*ROOT_CLASS)

            .children(&mut [
                html!("div", {
                    .class(&*TEXT_CLASS)
                    .text_signal(state.counter.signal().map(|x| format!("Counter: {}", x)))
                }),

                html!("div", {
                    .class("inline-flex")
                    .children(&mut [

                        html!("button", {
                            .class(btn)
                            .text("Increase")
                            .event(clone!(state => move |_: events::Click| {
                                // Increment the counter
                                state.counter.replace_with(|x| *x + 1);
                            }))
                        }),

                        html!("button", {
                            .class(btn)
                            .text("Decrease")
                            .event(clone!(state => move |_: events::Click| {
                                // Decrement the counter
                                state.counter.replace_with(|x| *x - 1);
                            }))
                        }),

                        html!("button", {
                            .class(btn)
                            .text("Reset")
                            .event(clone!(state => move |_: events::Click| {
                                // Reset the counter to 0
                                state.counter.set_neq(0);
                            }))
                        }),

                    ])
                }),
            ])
        })
    }
}

use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    let mut isCollapsed = use_signal(|| false);
    let mut currentMode = use_signal(|| "manga");

    let sidebarWidth = if isCollapsed() { "w-20" } else { "w-64" };

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            class: "flex h-screen w-full bg-gray-50 text-gray-800",

            aside {
                class: "{sidebarWidth} bg-white border-r border-gray-200 flex flex-col justify-between transition-all duration-300 ease-in-out p-4 h-full",

                div {
                    class: "flex items-center gap-3 px-2 py-1 overflow-hidden",

                    div {
                        class: "w-8 h-8 rounded-lg bg-blue-600 flex items-center justify-center text-white font-bold flex-shrink-0",

                        "H"
                    }

                    if !isCollapsed() {
                        span {
                            class: "font-bold text-xl tracking-tight text-gray-900 transition-opacity whitespace-nowrap",

                            "Hyoshi"
                        }
                    }
                }

                div {
                    if isCollapsed() {
                       button {
                           onclick: move |_| {
                               if currentMode() == "manga" {
                                   currentMode.set("anime")
                               } else {
                                   currentMode.set("manga")
                               }
                           },

                           "{currentMode().chars().next().unwrap_or('M')}"
                       }
                    } else {
                        button {
                            onclick: move |_| {
                             currentMode.set("manga")
                            },

                            "Manga"
                        },

                        button {
                            onclick: move |_| {
                                currentMode.set("anime")
                            },

                            "Anime"
                        }
                    }
                }

                nav {
                    Link {
                        to: Route::Library {},
                        span {"📚"}
                        if !isCollapsed() {
                            span { "Library" }
                        }
                    }
                    Link {
                        to: Route::Updates {},
                        span {"📰"}
                        if !isCollapsed() {
                            span { "Updates" }
                        }
                    }
                    Link {
                        to: Route::History {},
                        span {"📜"}
                        if !isCollapsed() {
                            span { "History" }
                        }
                    }
                    Link {
                        to: Route::Browse {},
                        span {"🔍"}
                        if !isCollapsed() {
                            span { "Browse" }
                        }
                    }
                }

                div {
                    Link {
                        to: Route::Settings {},
                        "Settings"
                    }

                    button {
                        onclick: move |_| {
                            isCollapsed.toggle();
                        },

                        span { if isCollapsed() { "☰" } else { "✖" } }

                        if !isCollapsed() {
                            span { "Collapse Sidebar"}
                        }
                    }
                }
            }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}

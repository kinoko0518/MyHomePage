use dioxus::prelude::*;

mod view;

fn main() {
    dioxus::launch(App);
}

// My own images
const DASH: Asset = asset!("assets/dash.png");
const PLUS: Asset = asset!("assets/plus.svg");

// CSS
const GENERAL_CSS: Asset = asset!("assets/css/general.css");
const TOP_CSS: Asset = asset!("assets/css/top_page.css");

// My own logos
const K_LOGO: Asset = asset!("assets/icon_k.png");
const MODERN_LOGO: Asset = asset!("assets/logo_modern.png");

// Outer logos
const GITHUB_LOGO: Asset = asset!("assets/logos/github.png");
const NOTE_LOGO: Asset = asset!("assets/logos/note.svg");
const XENON_LOGO: Asset = asset!("assets/logos/xenon.svg");
const XENON_PP_LOGO: Asset = asset!("assets/logos/xenon++.svg");

// Outer icons
const GITHUB_ICON: Asset = asset!("assets/logos/github_icon.svg");
const NOTE_ICON: Asset = asset!("assets/logos/note_icon.svg");
const RUST_ICON: Asset = asset!("assets/logos/rust.svg");
const DIOXUS_ICON: Asset = asset!("assets/logos/dioxus.svg");
const GODOT_ICON: Asset = asset!("assets/logos/godot.svg");

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: GENERAL_CSS }
        document::Stylesheet { href: TOP_CSS }

        Header {}

        // Main Content
        main {
            style: "margin-left: 15%; margin-right: 15%;",

            // Top Section
            img { src: MODERN_LOGO, class: "logo", }

            div {
                style: "
                    display: flex;
                    justify-content: center;
                ",
                Tip { content: NOTE_LOGO, link: "https://note.com/takedabosciola".to_string() }
                Tip { content: XENON_LOGO, link: "https://xe-non.com/".to_string() }
                Tip { content: GITHUB_LOGO, link: "https://github.com/kinoko0518".to_string() }
            }

            Index { content: "Private Projects".to_string() }
            view::PrivateProjects {}
            Index { content: "Team Projects".to_string() }
            view::TeamProjects {}
        }

        Footer {}
    }
}

#[component]
fn Icon(icon: Asset, link: String) -> Element {
    rsx! {
        a {
            href: link,
            img {
                class: "icon",
                src: icon,
            }
        }
    }
}

#[component]
fn Tip(content: Asset, link: String) -> Element {
    rsx! {
        a {
            href: link,
            img {
                class: "tip",
                src: content,
            }
        }
    }
}

#[component]
pub fn Index(content: String) -> Element {
    rsx! {
        div {
            class: "majour_heading_container",
            img { class: "dash", src: DASH }
            h1 { class: "majour_heading", {content} }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        header {
            class: "header",
            img {
                src: K_LOGO,
                height: "40pt",
            },
            a {
                class: "header_text",
                "Киноков Шотаскович"
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer {
            class: "footer",
            div {
                Icon {
                    icon: GITHUB_ICON,
                    link: "https://github.com/kinoko0518",
                },
                Icon {
                    icon: NOTE_ICON,
                    link: "https://note.com/takedabosciola",
                }
            }
        }
    }
}

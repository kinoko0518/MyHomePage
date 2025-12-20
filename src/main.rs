use dioxus::prelude::*;

mod static_site_generation;
mod view;

// My own images
pub const DASH: &str = "assets/dash.png";
pub const PLUS: &str = "assets/plus.svg";

// CSS
pub const GENERAL_CSS: &str = "assets/css/general.css";
pub const TOP_CSS: &str = "assets/css/top_page.css";

// My own logos
pub const K_LOGO: &str = "assets/icon_k.png";
pub const MODERN_LOGO: &str = "assets/logo_modern.png";

// Outer logos
pub const GITHUB_LOGO: &str = "assets/logos/github.png";
pub const NOTE_LOGO: &str = "assets/logos/note.svg";
pub const XENON_LOGO: &str = "assets/logos/xenon.svg";
pub const XENON_PP_LOGO: &str = "assets/logos/xenon++.svg";

// Outer icons
pub const GITHUB_ICON: &str = "assets/logos/github_icon.svg";
pub const NOTE_ICON: &str = "assets/logos/note_icon.svg";
pub const RUST_ICON: &str = "assets/logos/rust.svg";
pub const DIOXUS_ICON: &str = "assets/logos/dioxus.svg";
pub const GODOT_ICON: &str = "assets/logos/godot.svg";

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            ServeConfig::builder()
                .incremental(
                    IncrementalRendererConfig::new()
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
}

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Main {},
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: GENERAL_CSS }
        document::Stylesheet { href: TOP_CSS }

        Header {}
        Router::<Route> {}
        Footer {}
    }
}

#[component]
fn Main() -> Element {
    rsx! {
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
                Tip { content: NOTE_LOGO.to_string(), link: "https://note.com/takedabosciola".to_string() }
                Tip { content: XENON_LOGO.to_string(), link: "https://xe-non.com/".to_string() }
                Tip { content: GITHUB_LOGO.to_string(), link: "https://github.com/kinoko0518".to_string() }
            }

            Index { content: "Private Projects".to_string() }
            view::PrivateProjects {}
            Index { content: "Team Projects".to_string() }
            view::TeamProjects {}
        }
    }
}

#[component]
fn Icon(icon: String, link: String) -> Element {
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
fn Tip(content: String, link: String) -> Element {
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
                    icon: GITHUB_ICON.to_string(),
                    link: "https://github.com/kinoko0518",
                },
                Icon {
                    icon: NOTE_ICON.to_string(),
                    link: "https://note.com/takedabosciola",
                }
            }
        }
    }
}

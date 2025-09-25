use crate::{
    Icon, DIOXUS_ICON, GITHUB_ICON, GODOT_ICON, NOTE_ICON, PLUS, RUST_ICON, XENON_LOGO,
    XENON_PP_LOGO,
};
use dioxus::prelude::*;

// Sample image
const DHARMA_SAMPLE: Asset = asset!("assets/sample_images/dharma.png");
const RUST_3DS_SAMPLE: Asset = asset!("assets/sample_images/rust_3ds_dev.webp");
const ETOILE_SAMPLE: Asset = asset!("assets/sample_images/etoile.png");
const BOXFISH_OVERFLOW_SAMPLE: Asset = asset!("assets/sample_images/bfo.png");

#[component]
pub fn Project(
    project_name: String,
    detail: Element,
    icons: Vec<(Asset, String)>,
    stacks: Vec<(Asset, String)>,
    cover: Asset,
    link: String,
) -> Element {
    rsx! {
        div {
            class: "project",
            img {
                class: "sample",
                src: cover
            }
            a {
                style: "
                    font-size: 15pt;
                    font-weight: 800;
                ",
                href: link,
                class: "link",
                "{project_name}"
            }
            {detail}
            div {
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    gap: 5pt;
                ",
                {
                    icons.iter().map(|(icon, link)| rsx! {
                        Icon { icon: icon.clone(), link: link }
                    })
                }
            }
            hr {}
            div {
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: center;
                ",
                {stacks.iter().enumerate().map(|(index, (icon, link))| rsx! {
                    Icon { icon: icon.clone(), link: link }
                    {
                        if index != (stacks.len() - 1) {
                            rsx! {
                                img {
                                    src: PLUS,
                                    width: "10pt",
                                    height: "10pt",
                                }
                            }
                        } else {
                            rsx! {}
                        }
                    }
                })}
            }
        }
    }
}

#[component]
pub fn TeamLogo(img: Asset, link: String) -> Element {
    rsx! {
        a {
            href: link,
            img {
                src: XENON_LOGO,
                height: "65pt",
                style: "
                    margin-top: 30pt;
                    margin-bottom: 10pt;
                "
            }
        }
    }
}

#[component]
pub fn PrivateProjects() -> Element {
    let rust_stack = (RUST_ICON, "https://www.rust-lang.org/".to_string());
    let dioxus_stack = (DIOXUS_ICON, "https://dioxuslabs.com/".to_string());

    rsx! {
        div {
            style: "
                display: flex;
                align-items: stretch;
                gap: 10pt;
            ",
            Project {
                project_name: "Dharma".to_string(),
                detail: rsx! {
                    p { "テスト勉強用に作った一問一答アプリです。問題形式は選択式/記述式で、TOML形式で問題を作ります。" }
                },
                icons: vec![
                    (GITHUB_ICON, "https://github.com/kinoko0518/DharmaNava".to_string())
                ],
                stacks: vec![rust_stack.clone(), dioxus_stack.clone()],
                cover: DHARMA_SAMPLE,
                link: "https://github.com/kinoko0518/DharmaNava".to_string()
            }
            Project {
                project_name: "Rust 3DS Developing".to_string(),
                detail: rsx! {
                    p { "Rustで3DS向けの開発を行うためのチュートリアルです。もともとC言語向けのライブラリであるdevkitProのRust用ラッパーを使っています。noteで連載中です。" }
                },
                icons: vec![
                    (GITHUB_ICON, "https://github.com/kinoko0518/Rust-3DS-Example".to_string()),
                    (NOTE_ICON, "https://note.com/takedabosciola/m/m24e853602327".to_string())
                ],
                stacks: vec![rust_stack.clone()],
                cover: RUST_3DS_SAMPLE,
                link: "https://note.com/takedabosciola/m/m24e853602327".to_string()
            }
        }
    }
}

#[component]
pub fn TeamProjects() -> Element {
    rsx! {
        TeamLogo {
            img: XENON_LOGO,
            link: "https://xe-non.com/",
        }
        Project {
            project_name: "Etoile".to_string(),
            detail: rsx! {
                p {"ゲームキューブで妖怪や鬼をブン殴るゲームです。現在はプロジェクトを凍結中です。"}
            },
            icons: vec![],
            stacks: vec![(GODOT_ICON, "https://godotengine.org".to_string())],
            cover: ETOILE_SAMPLE,
            link: "https://xe-non.com/".to_string()
        }
        TeamLogo {
            img: XENON_PP_LOGO,
            link: "https://pp.xe-non.com/"
        }
        div {
            style: "
                display: flex;
                align-items: stretch;
                gap: 10pt;
            ",
            Project {
                project_name: "Boxfish Overflow!".to_string(),
                detail: rsx! {
                    p {"4bitのレジスタを持つハコフグくんを動かして論理ゲートをくぐり、特定のビットパターンを作るゲームです。"}
                },
                icons: vec![(GITHUB_ICON, "https://github.com/NIT-Tomakomai-2024J/BoxFishOverflow".to_string())],
                stacks: vec![(GODOT_ICON, "https://godotengine.org".to_string())],
                cover: BOXFISH_OVERFLOW_SAMPLE,
                link: "https://pp.xe-non.com/".to_string()
            }
            Project {
                project_name: "Cooking is Chemistry".to_string(),
                detail: rsx! {
                    p {"冷戦下ソヴィエトの原子力研究所、危険物が溢れるキッチンで料理を作り、理不尽クッキングを生存せよ！"}
                },
                icons: vec![(GITHUB_ICON, "https://github.com/netgame-nittc-2025-2/Cooking-is-Chemistry".to_string())],
                stacks: vec![(GODOT_ICON, "https://godotengine.org".to_string())],
                cover: ETOILE_SAMPLE,
                link: "https://pp.xe-non.com/".to_string()
            }
        }
    }
}

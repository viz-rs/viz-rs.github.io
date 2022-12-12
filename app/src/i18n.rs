use crate::Metadata;

#[cfg(all(feature = "en", not(feature = "zh-cn")))]
pub fn metadata() -> Metadata {
    Metadata {
        title: "Viz",
        description: "Fast, robust, flexible, lightweight web framework for Rust",
        note: "",
        docs: "Docs",
        color_scheme: "Switch to",
        mode: "mode",
        build_with: "Built with",
        deploys_on: "Deploys on",
        get_started: "Get Started",
    }
}

#[cfg(all(feature = "zh-cn", not(feature = "en")))]
pub fn metadata() -> Metadata {
    Metadata {
        title: "Viz",
        description: "快速、轻量、灵活、健壮的 Rust Web 框架",
        note: "",
        docs: "文档",
        color_scheme: "切换到",
        mode: "模式",
        build_with: "构建于",
        deploys_on: "部署在",
        get_started: "快速上手",
    }
}

use std::borrow::Cow;
use viewbuilder::{
    view::{OneOf2, OneOf4},
    web::{
        html::{self, class, html},
        Web,
    },
    ControlFlow, Model, View,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Tab {
    #[default]
    Readme,
    Versions,
    Dependencies,
    Dependents,
}

#[derive(Debug)]
enum CrateMessage {
    Tab(Tab),
    SelectVersion { idx: usize },
}

#[derive(Debug)]
enum Message {
    Crate(CrateMessage),
    Screen(Screen),
}

#[derive(Debug)]
struct Version {
    name: String,
}

#[derive(Debug)]
struct CrateScreen {
    tab: Tab,
    name: String,
    versions: Vec<Version>,
    version_idx: usize,
    license: String,
    repository: String,
}

impl Default for CrateScreen {
    fn default() -> Self {
        Self {
            tab: Default::default(),
            name: String::from("viewbuilder"),
            versions: vec![
                Version {
                    name: String::from("v0.10.0"),
                },
                Version {
                    name: String::from("v0.9.0"),
                },
            ],
            version_idx: 0,
            license: String::from("MIT or Apache-2.0"),
            repository: String::from("https://github.com/matthunz/viewbuilder"),
        }
    }
}

#[derive(Debug, Default)]
enum Screen {
    #[default]
    Home,
    Crate(CrateScreen),
}

#[derive(Default)]
pub struct App {
    screen: Screen,
}

impl Model<Message> for App {
    fn handle(&mut self, msg: Message) -> ControlFlow {
        match msg {
            Message::Crate(crate_msg) => {
                if let Screen::Crate(ref mut krate) = self.screen {
                    match crate_msg {
                        CrateMessage::Tab(tab) => krate.tab = tab,
                        CrateMessage::SelectVersion { idx } => {
                            krate.version_idx = idx;
                            krate.tab = Tab::Readme;
                        }
                    }
                }
            }
            Message::Screen(screen) => self.screen = screen,
        }
        ControlFlow::Rebuild
    }
}

fn view_readme() -> impl View<Web, Message> {
    "Readme"
}

fn view_versions(versions: &[Version]) -> impl View<Web, Message> {
    html::ul(
        (),
        versions
            .iter()
            .enumerate()
            .map(|(idx, version)| {
                (
                    idx,
                    html::li(
                        html::class("version"),
                        (
                            html::a(
                                (
                                    html::class("name"),
                                    html::on_click(move || {
                                        Message::Crate(CrateMessage::SelectVersion { idx })
                                    }),
                                ),
                                version.name.clone(),
                            ),
                            html::div((), "12,000"),
                        ),
                    ),
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn view_dependencies() -> impl View<Web, Message> {
    "Dependencies"
}

fn view_dependents() -> impl View<Web, Message> {
    "Dependents"
}

fn view_crate(model: &CrateScreen) -> impl View<Web, Message> {
    let selected_version = &model.versions[model.version_idx];

    html::div(
        html::class("container"),
        (
            html::h1((), model.name.clone()),
            html::h5(
                class("latest-version"),
                model.versions.last().unwrap().name.clone(),
            ),
            html::ul(
                html::class("tabs"),
                (
                    view_tab("Readme", Tab::Readme, model.tab),
                    view_tab("Versions", Tab::Versions, model.tab),
                    view_tab("Dependencies", Tab::Dependencies, model.tab),
                    view_tab("Dependents", Tab::Dependents, model.tab),
                ),
            ),
            html::div(
                html::class("wrap"),
                (
                    html::div(
                        html::class("content"),
                        match model.tab {
                            Tab::Readme => OneOf4::a(view_readme()),
                            Tab::Versions => OneOf4::b(view_versions(&model.versions)),
                            Tab::Dependencies => OneOf4::c(view_dependencies()),
                            Tab::Dependents => OneOf4::d(view_dependents()),
                        },
                    ),
                    html::ul(
                        html::class("sidebar"),
                        (
                            view_info("Documentation", html::a((), "url")),
                            view_info("Repository", html::a((), "url")),
                            view_info("Homepage", html::a((), "url")),
                            view_info(
                                "Install",
                                (
                                    view_command(format!(
                                        "viewbuilder = {}",
                                        &selected_version.name
                                    )),
                                    view_command("cargo install viewbuilder"),
                                ),
                            ),
                            html::li(
                                (),
                                html::ul(
                                    html::class("row"),
                                    (
                                        view_info(
                                            "Version",
                                            html::a((), selected_version.name.clone()),
                                        ),
                                        view_info("License", html::a((), model.license.clone())),
                                    ),
                                ),
                            ),
                            view_info("Last update", "20 hours ago"),
                        ),
                    ),
                ),
            ),
        ),
    )
}

fn view_info(label: &'static str, content: impl View<Web, Message>) -> impl View<Web, Message> {
    html::li(html::class("info"), (html::label((), label), content))
}

fn view_command(command: impl Into<Cow<'static, str>>) -> impl View<Web, Message> {
    html::li(html::class("command"), command.into())
}

fn view_tab(name: &'static str, tab: Tab, selected: Tab) -> impl View<Web, Message> {
    html::li(
        (
            if tab == selected {
                Some(html::class("selected"))
            } else {
                None
            },
            html::on_click(move || Message::Crate(CrateMessage::Tab(tab))),
        ),
        html::a((), name),
    )
}

fn view(model: &App) -> impl View<Web, Message> {
    (
        html::a(html::on_click(|| Message::Screen(Screen::Home)), "Home"),
        match &model.screen {
            Screen::Home => OneOf2::a(html::a(
                html::on_click(|| Message::Screen(Screen::Crate(CrateScreen::default()))),
                "Go to crate",
            )),
            Screen::Crate(krate) => OneOf2::b(view_crate(&krate)),
        },
    )
}

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::new()
            .set_max_level(tracing::Level::TRACE)
            .build(),
    );

    wasm_rs_async_executor::single_threaded::block_on(async move {
        viewbuilder::web::run(App::default(), view);
    });
}

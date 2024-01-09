use std::borrow::Cow;

use viewbuilder::{
    view::OneOf4,
    web::{html, Web},
    ControlFlow, Model, View,
};

#[derive(Debug)]
enum Message {
    Tab(Tab),
}

struct App {
    tab: Tab,
    name: String,
    version: String,
    license: String,
    repository: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tab: Default::default(),
            name: String::from("viewbuilder"),
            version: String::from("v0.10.0"),
            license: String::from("MIT or Apache-2.0"),
            repository: String::from("https://github.com/matthunz/viewbuilder"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Tab {
    #[default]
    Readme,
    Versions,
    Dependencies,
    Dependents,
}

impl Model<Message> for App {
    fn handle(&mut self, msg: Message) -> ControlFlow {
        match msg {
            Message::Tab(tab) => self.tab = tab,
        }
        ControlFlow::Rebuild
    }
}

fn view_readme() -> impl View<Web, Message> {
    "Readme"
}

fn view_versions() -> impl View<Web, Message> {
    "Versions"
}

fn view_dependencies() -> impl View<Web, Message> {
    "Dependencies"
}

fn view_dependents() -> impl View<Web, Message> {
    "Dependents"
}

fn view(model: &App) -> impl View<Web, Message> {
    html::div(
        html::class("container"),
        (
            html::h1((), model.name.clone()),
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
                            Tab::Versions => OneOf4::b(view_versions()),
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
                                    view_command(format!("viewbuilder = {}", model.version)),
                                    view_command("cargo install viewbuilder"),
                                ),
                            ),
                            html::li(
                                (),
                                html::ul(
                                    html::class("row"),
                                    (
                                        view_info("Version", html::a((), model.version.clone())),
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
            html::on_click(move || Message::Tab(tab)),
        ),
        html::a((), name),
    )
}

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::new()
            .set_max_level(tracing::Level::TRACE)
            .build(),
    );

    viewbuilder::web::run(App::default(), view);
}

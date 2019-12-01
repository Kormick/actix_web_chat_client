#![recursion_limit = "256"]

use yew::format::Nothing;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct Config {
    name: String,
    server_ip: String,
}

pub struct Model {
    config: Config,
    user_id: u32,
    fetch_service: FetchService,
    link: ComponentLink<Model>,
    value: String,
    messages: Vec<String>,
    debug_log: Vec<String>,
    ft: Option<FetchTask>,
}

pub enum Msg {
    GotInput(String),
    ConnectClicked,
    SendClicked,
    RefreshClicked,
    ReceivedUserId(u32),
    Ignore,
}

impl Model {
    fn parse_cmd_from_args() -> Config {
        Config {
            name: "user0".to_string(),
            server_ip: "http://127.0.0.25:8080".to_string(),
        }
    }

    fn show_messages(&self) -> Html<Self> {
        let render = |mes| {
            html! {
                <div>
                    { mes }
                </div>
            }
        };

        html! {
            { for self.messages.iter().map(render) }
        }
    }

    fn show_debug_log(&self) -> Html<Self> {
        let render = |mes| {
            html! {
                <div>
                    { mes }
                </div>
            }
        };

        html! {
            { for self.debug_log.iter().map(render) }
        }
    }

    fn log(&mut self, mes: &str) {
        self.debug_log.push(mes.to_string());
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            config: Model::parse_cmd_from_args(),
            user_id: 0,
            fetch_service: FetchService::new(),
            link,
            value: "".into(),
            messages: Vec::new(),
            debug_log: Vec::new(),
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.value = new_value;
            }
            Msg::ConnectClicked => {
                let callback = self.link.send_back(
                    move |response: Response<Result<String, failure::Error>>| {
                        if response.status().is_success() {
                            if let Ok(body) = response.body() {
                                return Msg::ReceivedUserId(body.parse().unwrap());
                            }
                        }
                        Msg::Ignore
                    },
                );

                let url = format!("{}/connect/{}", self.config.server_ip, self.config.name);

                let request = Request::get(url).body(Nothing).unwrap();
                self.ft = Some(self.fetch_service.fetch(request, callback));
            }
            Msg::SendClicked => {
                let callback = self.link.send_back(move |response: Response<Nothing>| {
                    let (meta, data) = response.into_parts();
                    println!("Response: {:?}, {:?}", meta, data);
                    Msg::Ignore
                });

                let url = format!(
                    "{}/chat/send/{}/{}",
                    self.config.server_ip,
                    self.user_id,
                    self.value.clone()
                );

                let request = Request::get(url).body(Nothing).unwrap();
                self.ft = Some(self.fetch_service.fetch(request, callback));
            }
            Msg::RefreshClicked => {
                let callback = self
                    .link
                    .send_back(move |_response: Response<Nothing>| Msg::Ignore);

                let url = format!("{}/chat.html", self.config.server_ip);

                let request = Request::get(url).body(Nothing).unwrap();
                self.ft = Some(self.fetch_service.fetch(request, callback));
            }
            Msg::ReceivedUserId(id) => {
                self.log(&format!("{}", id));
                self.user_id = id;
            }
            Msg::Ignore => (),
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    { self.user_id }
                </div>
                <div>
                    { self.show_messages() }
                </div>
                <div>
                    <textarea rows=5
                        value=&self.value
                        oninput=|e| Msg::GotInput(e.value)
                        placeholder="placeholder">
                    </textarea>
                </div>
                <div>
                    <button onclick=|_| Msg::ConnectClicked>{ "Connect" }</button>
                </div>
                <div>
                    <button onclick=|_| Msg::SendClicked>{ "Send" }</button>
                </div>
                <div>
                    <button onclick=|_| Msg::RefreshClicked>{ "Refresh" }</button>
                </div>
                <div>
                    { self.show_debug_log() }
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

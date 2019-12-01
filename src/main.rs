#![recursion_limit = "256"]

use yew::format::Nothing;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
    GotMessageInput(String),
    GotUserNameInput(String),

    ConnectClicked,
    SendClicked,
    RefreshClicked,
    CountClicked,

    ReceivedMessages(String),

    Ignore,
}

struct Config {
    server_ip: String,
}

impl Config {
    fn create() -> Config {
        Config {
            server_ip: "http://127.0.0.25:8080".to_string(),
        }
    }
}

pub struct Model {
    console: ConsoleService,
    config: Config,
    fetch_service: FetchService,
    ft: Option<FetchTask>,
    link: ComponentLink<Model>,

    counter: u32,
    user_name: String,
    current_message: String,
    messages_html: String,
}

impl Model {
    fn show_messages(&self) -> Html<Self> {
        let render = |mes| {
            html! {
                <div>
                    { mes }
                </div>
            }
        };

        let messages: Vec<&str> = self.messages_html.as_str().split("<br/>").collect();
        html! {
            { for messages.iter().map(render) }
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            config: Config::create(),
            fetch_service: FetchService::new(),
            ft: None,
            link,
            counter: 0,
            user_name: String::new(),
            current_message: String::new(),
            messages_html: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotUserNameInput(user_name) => {
                self.console.log("GotUserNameInput");
                self.user_name = user_name;
            }
            Msg::GotMessageInput(message) => {
                self.console.log("GotMessageInput");
                self.current_message = message;
            }
            Msg::ConnectClicked => {
                self.console.log("ConnectClicked");
                let callback = self
                    .link
                    .send_back(move |_: Response<Result<String, failure::Error>>| Msg::Ignore);

                let url = format!("{}/connect/{}", self.config.server_ip, self.user_name);
                self.console.log(&url);

                let request = Request::get(url).body(Nothing).unwrap();
                self.ft = Some(self.fetch_service.fetch(request, callback));
            }
            Msg::SendClicked => {
                self.console.log("SendClicked");
                let callback = self.link.send_back(move |_: Response<Nothing>| Msg::Ignore);

                let url = format!(
                    "{}/chat/send/{}/{}",
                    self.config.server_ip,
                    self.user_name,
                    self.current_message.clone()
                );
                self.console.log(&url);

                let request = Request::get(url).body(Nothing).unwrap();
                self.ft = Some(self.fetch_service.fetch(request, callback));
            }
            Msg::RefreshClicked => {
                self.console.log("RefreshClicked");
                let callback = self.link.send_back(
                    move |response: Response<Result<String, failure::Error>>| {
                        if response.status().is_success() {
                            if let Ok(body) = response.body() {
                                return Msg::ReceivedMessages(body.to_string());
                            }
                        }
                        Msg::Ignore
                    },
                );

                let url = format!("{}/chat.html", self.config.server_ip);
                self.console.log(&url);

                let request = Request::get(url).body(Nothing).unwrap();
                self.ft = Some(self.fetch_service.fetch(request, callback));
            }
            Msg::CountClicked => {
                self.console.log("CountClicked");
                self.counter += 1;
            }
            Msg::ReceivedMessages(html) => {
                self.console.log("ReceivedMessages");
                self.messages_html = html;
            }
            Msg::Ignore => (),
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    { self.counter }
                </div>
                <div>
                    { self.config.server_ip.clone() }
                </div>
                <div>
                    { self.show_messages() }
                </div>
                <div>
                    { "User name: " }
                    <textarea rows=1
                        value=&self.user_name
                        oninput=|e| Msg::GotUserNameInput(e.value)
                        placeholder="User name">
                    </textarea>
                </div>
                <div>
                    <textarea rows=5
                        value=&self.current_message
                        oninput=|e| Msg::GotMessageInput(e.value)
                        placeholder="Message">
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
                    <button onclick=|_| Msg::CountClicked>{ "Count" }</button>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

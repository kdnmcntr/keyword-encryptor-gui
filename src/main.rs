mod encryptor;

use std::path::PathBuf;
use iced::{widget::{button, column, text, row, horizontal_rule, text_input}, window, Element, Task, Length::Fill, Border, Shadow, executor, Theme, Settings, Application};
use iced::font::load;
use rfd;
use rfd::FileHandle;

fn main() -> iced::Result {

    iced::application("Keyword Encryptor", App::update, App::view).run()

}

#[derive(Debug)]
struct App {
    input_file: String,
    output_file: String,
    keyword: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input_file: String::new(),
            output_file: String::new(),
            keyword: String::new(),
        }
    }
}

impl App {

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit => window::get_latest().and_then(window::close),
            Message::Encrypt => {
                encryptor::Encryptor::new(self.input_file.clone(), self.output_file.clone(), self.keyword.clone()).encrypt_file().unwrap();
                Task::none()
            },
            Message::Keyword_Changed(keywd) => {
                self.keyword = keywd;
                Task::none()
            },
            Message::InputChanged(file_name) => {
                self.input_file = file_name;
                Task::none()
            },
            Message::OutputChanged(output_file) => {
                self.output_file = output_file;
                Task::none()
            },
            Message::FileCancelled => {
                Task::none()
            },
            Message::OpenFile => {
                open_file()
            },
            Message::SaveFile => {
                save_file()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let go_button = if self.is_ready() {
            button(text("Go")).on_press(Message::Encrypt)
        } else {
            button(text("Go"))
        };

        let selected_input_file: String = if self.input_file.is_empty() {String::from("(No file selected)")} else {self.input_file.clone()};
        let selected_output_file: String = if self.output_file.is_empty() {String::from("(No file selected)")} else {self.output_file.clone()};

        let mut content = column![
        row![
            text("Source file: "),
            text(selected_input_file),
            button("Select File").on_press(Message::OpenFile),
        ],
        row![
            text("Output file: "),
            text(selected_output_file),
            button("Select File").on_press(Message::SaveFile),
        ],
        row![
            text("Password"),
            text_input("Type password here", &self.keyword)
                .on_input(|s| Message::Keyword_Changed(s)),
        ],
        row![
            button(text("Cancel")).on_press(Message::Exit),
            go_button,
        ]
    ];

        content.into()
    }

    fn is_ready(&self) -> bool {
        !self.input_file.is_empty() && !self.output_file.is_empty() && !self.keyword.is_empty()
    }

}

#[derive(Debug, Clone)]
enum Message {
    Exit,
    Encrypt,
    Keyword_Changed(String),
    InputChanged(String),
    OutputChanged(String),
    FileCancelled,
    OpenFile,
    SaveFile,
}

fn open_file() -> Task<Message> {
    Task::future(
        rfd::AsyncFileDialog::new()
            .pick_file(),
    ).then(|handle| {
        match handle {
            Some(file_handle) => Task::perform(get_file_path(file_handle), |result| Message::InputChanged(result)),
            None => Task::done(Message::FileCancelled),
        }
    })

}

fn save_file() -> Task<Message> {
    Task::future(
        rfd::AsyncFileDialog::new()
            .set_file_name("")
            .save_file(),
    ).then(|handle| {
        match handle{
            Some(file_handle) => Task::perform(get_file_path(file_handle), |result| Message::OutputChanged(result)),
            None => Task::done(Message::FileCancelled),
        }
    })
}

async fn get_file_path(file_handle: FileHandle) -> String {
    match Some(file_handle.path()) {
        Some(path) => path.to_string_lossy().to_string(),
        None => String::new(), // fallback if path is unavailable
    }
}
mod encryptor;

use std::path::PathBuf;
use iced::{widget::{button, column, text, row, horizontal_rule, text_input}, window, Element, Task, Length::Fill, Border, Shadow, executor, Theme, Settings, Application, Size};
use iced::font::load;
use iced::widget::{container, Text};
use rfd;
use rfd::FileHandle;

fn main() -> iced::Result {

    let window_settings = window::Settings {
        size: Size::new(600.0, 300.0),
        resizable: true,
        decorations: true,
        ..Default::default()
    };


    iced::application("Keyword Encryptor", App::update, App::view).window(window_settings).run()

}

#[derive(Debug)]
struct App {
    input_file: String,
    output_file: String,
    keyword: String,
    is_running: bool,
    is_completed: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input_file: String::new(),
            output_file: String::new(),
            keyword: String::new(),
            is_running: false,
            is_completed: false,
        }
    }
}

impl App {

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit => window::get_latest().and_then(window::close),
            Message::Encrypt => {
                self.is_running = true;
                //Task::perform(self.run_encryption(), |result| Message::Completed(result));
                let in_file = self.input_file.clone();
                let out_file = self.output_file.clone();
                let kwd = self.keyword.clone();
                Task::future(async {
                    run_encryption(in_file, out_file, kwd).await;
                    Message::Completed
                })
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
            },
            Message::Completed => {
                self.is_running = false;
                self.is_completed = true;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let go_button = if self.is_ready() && !self.is_running && !self.is_completed {
            button(text("Go")).on_press(Message::Encrypt)
        } else {
            button(text("Go"))
        };

        let cancel_button = if !self.is_running && !self.is_completed {
            button(text("Cancel")).on_press(Message::Exit)
        } else {
            button(text("Cancel"))
        };

        let input_file_button= if !self.is_running && !self.is_completed {
            button(text("Select File")).on_press(Message::OpenFile)
        } else {
            button(text("Select File"))
        };

        let output_file_button= if !self.is_running && !self.is_completed {
            button(text("Select File")).on_press(Message::SaveFile)
        } else {
            button(text("Select File"))
        };

        let running_message: Text = if self.is_running {
            text("Processing file...")
        } else if self.is_completed {
            text("Completed.")
        } else {
            text("")
        };

        let ok_button = button(text("Ok")).on_press(Message::Exit);
        let final_row = if self.is_completed {
            row![running_message, ok_button].spacing(10).padding(6)
        } else {
            row![running_message].spacing(10).padding(6)
        };

        let mut selected_input_file: String = if self.input_file.is_empty() {String::from("(No file selected)")} else {self.input_file.clone()};
        let mut selected_output_file: String = if self.output_file.is_empty() {String::from("(No file selected)")} else {self.output_file.clone()};

        if selected_input_file.len() > 45 {
            selected_input_file = format!("{}...",selected_input_file[0..45].to_string());
        }

        if selected_output_file.len() > 45 {
            selected_output_file = format!("{}...",selected_output_file[0..45].to_string());
        }

        let mut content = column![
        row![
            text("Source file: "),
            text(selected_input_file),
            input_file_button,
        ].spacing(4).padding(6),
        row![
            text("Output file: "),
            text(selected_output_file),
            output_file_button,
        ].spacing(4).padding(6),
        row![
            text("Password: "),
            text_input("Type password here", &self.keyword)
                .on_input(|s| Message::Keyword_Changed(s)).width(300),
        ].spacing(4).padding(6),
        row![
            text("").width(Fill),
            cancel_button,
            go_button,
        ].spacing(10).padding(6),
        final_row,
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
    Completed,
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

async fn run_encryption(in_file: String, out_file: String, kwd: String) {
    encryptor::Encryptor::new(in_file, out_file, kwd).encrypt_file().unwrap();
}
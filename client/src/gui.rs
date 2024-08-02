use eframe::egui;
use egui::Separator;

pub struct ChatApp {
    state: AppState,
    username: String,
    messages: Vec<Message>,
    input_text: String,
}

#[derive(PartialEq)]
enum AppState {
    Menu,
    ChatRoom,
}

struct Message {
    sender: String,
    message: String,
}

impl Default for ChatApp {
    fn default() -> Self {
        Self {
            state: AppState::Menu,
            messages: vec![Message {
                sender: "[Chatroom]".to_string(),
                message: "Welcome to the chat room!".to_string(),
            }],
            input_text: String::new(),
            username: "Anonymous".to_string(),
        }
    }
}

impl eframe::App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.state {
            AppState::Menu => self.show_menu(ctx),
            AppState::ChatRoom => self.show_chat_room(ctx),
        }
    }
}

impl ChatApp {
    fn show_menu(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(format!("Welcome to the Chat Room, {}!", self.username));
                ui.add(Separator::default().horizontal().spacing(10.0));
                let input =
                    egui::TextEdit::singleline(&mut self.username).hint_text("Enter a username...");
                ui.add(input);
                if ui.button("Enter Chatroom").clicked() {
                    self.state = AppState::ChatRoom;
                }
            });
        });
    }

    fn show_chat_room(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Chat Room");
                ui.separator();

                // Define the height of the input area and calculate the remaining height for messages
                let input_area_height = 30.0; // Adjust as needed for the input field and buttons
                let available_height = ui.available_height() - input_area_height - 10.0; // Subtract some padding

                // Display messages in a scrollable area
                egui::ScrollArea::vertical()
                    .auto_shrink([false, true])
                    .stick_to_bottom(true)
                    .max_height(available_height)
                    .show(ui, |ui| {
                        for msg in &self.messages {
                            ui.label(format!("{}: {}", msg.sender, msg.message));
                        }
                    });

                ui.separator();

                // Input area for new messages
                ui.horizontal(|ui| {
                    let input = egui::TextEdit::singleline(&mut self.input_text)
                        .hint_text("Type your message here...");
                    let response = ui.add(input);
                    if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        response.request_focus();
                        self.send_message();
                    }
                    if ui.button("Send").clicked() {
                        self.send_message();
                    }
                });
            });
        });
    }

    fn send_message(&mut self) {
        if !self.input_text.trim().is_empty() {
            self.messages.push(Message {
                sender: self.username.clone(),
                message: self.input_text.clone(),
            });
            self.input_text.clear();
        }
    }
}

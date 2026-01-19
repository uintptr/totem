use eframe::egui;
use rand::seq::index::sample;

mod words;

const UI_WIDTH: f32 = 200.0;
const UI_HEIGHT: f32 = 200.0;

const APP_NAME: &str = env!("CARGO_CRATE_NAME");

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([UI_WIDTH, UI_HEIGHT]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
}

struct MyApp {
    words: [&'static str; 5],
    selected: Option<usize>,
    printed: bool,
}

impl MyApp {
    fn new() -> Self {
        let mut rng = rand::rng();
        let indices = sample(&mut rng, words::WORDS.len(), 5);

        Self {
            words: [
                words::WORDS[indices.index(0)],
                words::WORDS[indices.index(1)],
                words::WORDS[indices.index(2)],
                words::WORDS[indices.index(3)],
                words::WORDS[indices.index(4)],
            ],
            selected: None,
            printed: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle key presses
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Num1) {
                self.selected = Some(1);
            } else if i.key_pressed(egui::Key::Num2) {
                self.selected = Some(2);
            } else if i.key_pressed(egui::Key::Num3) {
                self.selected = Some(3);
            } else if i.key_pressed(egui::Key::Num4) {
                self.selected = Some(4);
            } else if i.key_pressed(egui::Key::Num5) {
                self.selected = Some(5);
            }
        });

        // If a selection was made, print it once and exit
        if let Some(n) = self.selected {
            if !self.printed {
                println!("{}", n);
                self.printed = true;
            }
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            for (i, word) in self.words.iter().enumerate() {
                let num = i + 1;
                let text = format!("{}. {}", num, word);
                ui.label(egui::RichText::new(text).size(24.0));
            }
        });
    }
}

use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};

use anyhow::anyhow;
use clap::Parser;
use eframe::egui;
use log::{LevelFilter, info};
use rand::Rng;
use rand::seq::index::sample;
use rstaples::logging::StaplesLogger;

use crate::config::{Mode, TotemConfig};

mod config;
mod words;

const UI_WIDTH: f32 = 200.0;
const UI_HEIGHT: f32 = 200.0;

const APP_NAME: &str = env!("CARGO_CRATE_NAME");

#[derive(Parser)]
#[command(name = "totem", about = "A totem selector for identity verification")]
struct Args {
    /// Path to config file (default: ~/.config/totem/totem.conf)
    #[arg(short, long, default_value_os_t = default_config_file())]
    config_file: PathBuf,

    /// verbose
    #[arg(short, long)]
    verbose: bool,
}

fn get_default_config() -> anyhow::Result<PathBuf> {
    let config_dir = dirs::config_dir().ok_or_else(|| anyhow!("Unable to find config dir"))?;

    let app_config_dir = config_dir.join(APP_NAME);

    if !app_config_dir.exists() {
        fs::create_dir_all(&app_config_dir)?;
    }

    let config_file = app_config_dir.join("totem.json");

    Ok(config_file)
}

fn default_config_file() -> PathBuf {
    let config_file = get_default_config().unwrap();
    PathBuf::from(config_file)
}

struct MyApp {
    items: Vec<String>,
    totem_position: Option<usize>,
    selected: Option<usize>,
    printed: bool,
}

impl MyApp {
    fn new(config: TotemConfig) -> Self {
        let mut rng = rand::rng();

        // First item is the totem, rest are decoys
        let totem = config.totems[0].clone();
        let decoys: Vec<String> = config.totems.iter().skip(1).cloned().collect();

        // Generate 4 random items - from decoys if available, otherwise random
        let mut selected: Vec<String> = if decoys.len() >= 4 {
            // Use decoys from config
            let indices = sample(&mut rng, decoys.len(), 4);
            indices.into_iter().map(|i| decoys[i].clone()).collect()
        } else {
            // Generate random items based on mode
            match config.mode {
                Mode::Numbers => (0..4)
                    .map(|_| format!("{:06}", rng.random_range(0..1_000_000)))
                    .collect(),
                Mode::Words => {
                    let indices = sample(&mut rng, words::WORDS.len(), 4);
                    indices
                        .into_iter()
                        .map(|i| words::WORDS[i].to_string())
                        .collect()
                }
            }
        };

        // Insert totem at a random position
        let totem_pos = rng.random_range(0..5);
        selected.insert(totem_pos, totem);

        Self {
            items: selected,
            totem_position: Some(totem_pos),
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
                self.selected = Some(0);
            } else if i.key_pressed(egui::Key::Num2) {
                self.selected = Some(1);
            } else if i.key_pressed(egui::Key::Num3) {
                self.selected = Some(2);
            } else if i.key_pressed(egui::Key::Num4) {
                self.selected = Some(3);
            } else if i.key_pressed(egui::Key::Num5) {
                self.selected = Some(4);
            }
        });

        // If a selection was made, print result and exit
        if let Some(selected_idx) = self.selected {
            if !self.printed {
                // Output whether the correct totem was selected
                let success = self.totem_position.map_or(true, |pos| selected_idx == pos);

                let ret_str = if success { "1" } else { "0" };

                write!(io::stdout(), "{ret_str}").unwrap();
                io::stdout().flush().unwrap();
                self.printed = true;
            }
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            for (i, item) in self.items.iter().enumerate() {
                let num = i + 1;
                let text = format!("{}. {}", num, item);
                ui.label(egui::RichText::new(text).size(24.0));
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let args = Args::parse();

    let level = if args.verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Error
    };

    StaplesLogger::new()
        .with_stderr()
        .with_colors()
        .with_log_level(level)
        .start();

    info!("config file: {}", args.config_file.display());

    let config = TotemConfig::load(&args.config_file).expect("Failed to load config");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([UI_WIDTH, UI_HEIGHT]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(move |_cc| Ok(Box::new(MyApp::new(config)))),
    )
}

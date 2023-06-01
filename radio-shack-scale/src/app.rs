use hidapi::HidApi;
use std::{sync::mpsc, time::Duration};

pub struct App {
    selected_idx: usize,
    text: String,
    zero: u16,
    initialized: bool,
    receiver: mpsc::Receiver<USBMessage>,
}

const UNITS_TO_GRAMS: f64 = 1./2.674; // Guess based on seeing what other program does
const GRAMS_TO_OUNCES: f64 = 1./28.3495;
const OUNCES_TO_POUNDS: f64 = 1./16.;

#[derive(Debug)]
struct Unit {
    name: &'static str,
    conversion: f64,
    unit_string: &'static str,
    format_places: usize,
}

const UNITS: [Unit; 4] = [
    Unit { name: "Grams", conversion: UNITS_TO_GRAMS, unit_string: "g", format_places: 0 },
    Unit { name: "Kilograms", conversion: UNITS_TO_GRAMS/1000., unit_string: "kg", format_places: 3 },
    Unit { name: "Ounces", conversion: UNITS_TO_GRAMS*GRAMS_TO_OUNCES, unit_string: "oz", format_places: 1 },
    Unit { name: "Pounds", conversion: UNITS_TO_GRAMS*GRAMS_TO_OUNCES*OUNCES_TO_POUNDS, unit_string: "lb", format_places: 2 },
];

impl App {
    fn default(receiver: mpsc::Receiver<USBMessage>) -> App {
        App {
            selected_idx: 0,
            text: "Connecting...".to_string(),
            zero: 0,
            initialized: false,
            receiver: receiver,
        }
    }

    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let (sender, receiver) = mpsc::channel();
        std::thread::spawn(move|| {
            usb_polling(sender);
        });
        App::default(receiver)
    }
}

fn format_value(unit: &Unit, weight: u16, zero: u16) -> String {
    // Doing all math as f64 to avoid overflow issues when result is negative.
    let converted = (weight as f64 - zero as f64) * unit.conversion;
    format!("{converted:.prec$}{}", unit.unit_string, prec=unit.format_places)
}

type USBMessage = Result<u16, hidapi::HidError>;

const SLEEP_TIME: Duration = Duration::from_millis(1000/30);

fn usb_polling(sender: mpsc::Sender<USBMessage>) {
    loop {
        let msg = read_from_scale();
        sender.send(msg).unwrap();
        std::thread::sleep(SLEEP_TIME);
    }
}

fn read_from_scale() -> USBMessage {
    let device = HidApi::new()?.open(8755, 25379)?;
    let mut array: [u8; 8] = [0 ; 8];
    let _ = device.read(&mut array)?;
    let res = ((array[6] as u16) << 8) + (array[7] as u16);
    return Ok(res);
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(4.);
        ctx.request_repaint_after(SLEEP_TIME);

        let Self { selected_idx, text, zero, initialized , ..} = self;
        let selected = &UNITS[*selected_idx];

        // Poll for USB update.
        match self.receiver.try_iter().last() {
            None => { }
            Some(msg) => match msg {
                Err(e) => {
                    *initialized = false;
                    match e {
                        hidapi::HidError::OpenHidDeviceError => {
                            *text = format!("No Device").to_string();
                        }
                        _ => {
                            println!("{:?}", e);
                            *text = format!("Error {}", e).to_string();
                        }
                    }
                }
                Ok(weight) => {
                    // HACK when plugging a device in, the first reading is a 1.
                    // Literally, 0x0001 in last 2 bytes. So, we skip it to get a real reading.
                    if weight != 1 {
                        if !*initialized {
                            println!("Zero: {}", weight);
                            *zero = weight;
                            *initialized = true;
                        }
                        *text = format_value(selected, weight, *zero);
                    }
                }
            }
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            // Code snip for centered layout:
            // ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            let margin = 20.;
            let padding = 5.;
            ui.horizontal(|ui| {
                ui.add_space(margin);
                ui.vertical(|ui| {
                    ui.add_space(padding);
                    ui.heading(text);
                    ui.add_space(padding);
                    if ui.button("Zero").clicked() {
                        *initialized = false;
                    }
                    ui.add_space(padding);
                    egui::ComboBox::from_label("")
                        .selected_text(selected.name)
                        .show_ui(ui, |ui| {
                            for (idx, unit) in UNITS.iter().enumerate() {
                                ui.selectable_value(selected_idx, idx, unit.name);
                            }
                        }
                    );
                });
                ui.add_space(margin);
            });

            egui::warn_if_debug_build(ui);
        });
    }
}

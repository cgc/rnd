#[macro_use] extern crate conrod_core;
extern crate conrod_glium;
extern crate glium;
extern crate hidapi;

use hidapi::HidApi;
use glium::Surface;
use std::sync::mpsc;

mod support;

/// This struct holds all of the variables used to demonstrate application data being passed
/// through the widgets. If some of these seem strange, that's because they are! Most of these
/// simply represent the aesthetic state of different parts of the GUI to offer visual feedback
/// during interaction with the widgets.
struct App {
    /// A vector of strings for drop_down_list demonstration.
    unit_names: Vec<String>,
    /// We also need an Option<idx> to indicate whether or not an
    /// item is selected.
    selected_idx: Option<usize>,
    text: String,
    zero: u16,
    conversion: f64,
    unit_string: &'static str,
    format_places: i8,
    initialized: bool,
}

fn option_vals(option: &str) -> (f64, &'static str, i8) {
    let units_to_grams = 1./2.674; // Guess based on seeing what other program does
    let grams_to_ounces = 1./28.3495;
    let ounces_to_pounds = 1./16.;
    match option {
        "Grams" => (units_to_grams, "g", 0),
        "Kilograms" => (units_to_grams/1000., "kg", 3),
        "Ounces" => (units_to_grams*grams_to_ounces, "oz", 1),
        "Pounds" => (units_to_grams*grams_to_ounces*ounces_to_pounds, "lb", 2),
        _ => panic!("Missing unit"),
    }
}

impl App {
    /// Constructor for the Demonstration Application model.
    fn new() -> App {
        let defaultkey = "Grams";

        let val = option_vals(defaultkey);

        App {
            unit_names: vec![
                "Grams".to_string(),
                "Kilograms".to_string(),
                "Ounces".to_string(),
                "Pounds".to_string()],
            selected_idx: None,
            text: "Connecting...".to_string(),
            zero: 0,
            conversion: val.0,
            unit_string: val.1,
            format_places: val.2,
            initialized: false,
        }
    }

    fn format_value(&self, value: u16) -> String {
        // Doing all math as f64 to avoid overflow issues when result is negative.
        let converted = (value as f64 - self.zero as f64) * self.conversion;
        if self.format_places == 0 {
            format!("{:.0}{}", converted, self.unit_string)
        } else if self.format_places == 1 {
            format!("{:.01}{}", converted, self.unit_string)
        } else if self.format_places == 2 {
            format!("{:.02}{}", converted, self.unit_string)
        } else if self.format_places == 3 {
            format!("{:.03}{}", converted, self.unit_string)
        } else {
            panic!("Unsupported number of places");
        }
    }
}


fn usb_polling(sender: mpsc::Sender<Result<u16, hidapi::HidError>>, termination_receiver: mpsc::Receiver<bool>) {
    let sleep_time = std::time::Duration::from_millis(50);

    loop {
        // Have we received a termination signal?
        if let Ok(value) = termination_receiver.try_recv() {
            if value {
                println!("Received shutdown signal in USB polling.");
                return;
            }
        }
        // If not, we should get the USB data and send.
        let msg = Scale::read();
        sender.send(msg).unwrap();
        std::thread::sleep(sleep_time);
    }
}


struct Scale { }
impl Scale {
    //fn new() -> Scale { Scale { } }

    fn read() -> Result<u16, hidapi::HidError> {
        let device = HidApi::new()?.open(8755, 25379)?;
        let mut array: [u8; 8] = [0 ; 8];
        let _ = device.read(&mut array)?;
        let res = ((array[6] as u16) << 8) + (array[7] as u16);
        return Ok(res);
    }
}


fn main() {
    const WIDTH: u32 = 270;
    const HEIGHT: u32 = 200;

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Radio Shack Scale")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = support::GliumDisplayWinitWrapper(display);

    // construct our `Ui`.
    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Identifiers used for instantiating our widgets.
    let mut ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    ui.fonts.insert_from_file("/Library/Fonts/Arial.ttf").unwrap();

    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod_glium::Renderer::new(&display.0).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

    // Our demonstration app that we'll control with our GUI.
    let mut app = App::new();

    // Spawn the thread that polls the USB device.
    let (sender, receiver) = mpsc::channel();
    let (termination_sender, termination_receiver) = mpsc::channel();
    std::thread::spawn(move|| {
        usb_polling(sender, termination_receiver);
    });

    // Poll events from the window.
    let mut event_loop = support::EventLoop::new();
    'main: loop {
        match receiver.try_iter().last() {
            None => {
                //println!("no message");
            }
            Some(msg) => match msg {
                Err(e) => {
                    app.initialized = false;
                    event_loop.needs_update();
                    match e {
                        hidapi::HidError::OpenHidDeviceError => {
                            app.text = format!("No Device").to_string();
                        }
                        _ => {
                            println!("{:?}", e);
                            app.text = format!("Error {}", e).to_string();
                        }
                    }
                }
                Ok(weight) => {
                    // HACK when plugging a device in, the first reading is a 1.
                    // Literally, 0x0001 in last 2 bytes. So, we skip it to get a real reading.
                    if weight != 1 {
                        if !app.initialized {
                            println!("Zero: {}", weight);
                            app.zero = weight;
                            app.initialized = true;
                        }
                        app.text = app.format_value(weight);
                        event_loop.needs_update();
                    }
                }
            }
        };

        // Handle all events.
        for event in event_loop.next(&mut events_loop, false) {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = support::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::CloseRequested |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => {
                        termination_sender.send(true).unwrap(); // ????????
                        break 'main;
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        // We'll set all our widgets in a single function called `set_widgets`.
        {
            let mut ui = ui.set_widgets();
            set_widgets(&mut ui, &mut app, &mut ids);
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display.0, primitives, &image_map);
            let mut target = display.0.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display.0, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}


// In conrod, each widget must have its own unique identifier so that the `Ui` can keep track of
// its state between updates.
//
// To make this easier, conrod provides the `widget_ids` macro. This macro generates a new type
// with a unique `widget::Id` field for each identifier given in the list. See the `widget_ids!`
// documentation for more details.
widget_ids! {
    struct Ids {
        canvas,
        title,
        button,
        unit_select,
    }
}


/// Set all `Widget`s within the User Interface.
///
/// The first time this gets called, each `Widget`'s `State` will be initialised and cached within
/// the `Ui` at their given indices. Every other time this get called, the `Widget`s will avoid any
/// allocations by updating the pre-existing cached state. A new graphical `Element` is only
/// retrieved from a `Widget` in the case that it's `State` has changed in some way.
fn set_widgets(ui: &mut conrod_core::UiCell, app: &mut App, ids: &mut Ids) {
    use conrod_core::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

    widget::Canvas::new()
        .pad(10.0)
        .color(color::WHITE)
        .scroll_kids()
        .set(ids.canvas, ui);

    widget::Text::new(&app.text)
        .top_left_with_margins_on(ids.canvas, 10.0, 10.0)
        .w(120.*2.)
        .right_justify()
        .font_size(32)
        .color(color::BLACK)
        .set(ids.title, ui);

    let button_color = color::rgb(0.88, 0.88, 0.88);

    if widget::Button::new()
        .w_h(120.0, 50.0)
        .mid_left_of(ids.canvas)
        .down_from(ids.title, 25.0)
        .color(button_color)
        .label("Zero")
        .set(ids.button, ui)
        .was_clicked()
    {
        // Next reading will get copied to be zero.
        app.initialized = false;
    }

    for selected_idx in widget::DropDownList::new(&app.unit_names, app.selected_idx)
        .w_h(120.0, 50.0)
        .right_from(ids.button, 10.0)
        .max_visible_items(4)
        .color(button_color)
        .label(&app.unit_names[0][..])
        .scrollbar_next_to()
        .set(ids.unit_select, ui)
    {
        app.selected_idx = Some(selected_idx);
        let optionval = option_vals(&app.unit_names[selected_idx][..]);
        app.conversion = optionval.0;
        app.unit_string = optionval.1;
        app.format_places = optionval.2;
    }
}

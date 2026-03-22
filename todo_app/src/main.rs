#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast; 
use eframe::egui;

//Data
struct PomodoroApp {
    seconds_left: u32,
    is_running: bool,
    last_tick: std::time::Instant,
}

//logic
impl PomodoroApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            // Starting values
            seconds_left: 1500, // change this for testing
            is_running: false,
            last_tick: std::time::Instant::now(),
        }
    }
}

//drawing
impl eframe::App for PomodoroApp {
    //boilerplate
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
            // This container makes everything big and centered!
            ui.vertical_centered(|ui| {
                ui.add_space(30.0); // Padding at the top
                ui.heading("Pomodoro Timer");//Heading
                //AAAA there are semicolons sometimes and commas sometimes and AAAA
                //this is why i love like python theres none of this

                //setting variables
                // (ik it sounds dumb im adding these comments)
                // pls bear with me i love comments too much
                let mins = self.seconds_left / 60;
                let secs = self.seconds_left % 60; //modulo

                // Print this in the UI!
                let time_text = format!("{}:{:02}", mins, secs);
                
                let mut big_time = egui::RichText::new(time_text).size(80.0).monospace();

                // Make the timer green on 0min!
                if self.seconds_left == 0 {
                    big_time = big_time.color(egui::Color32::GREEN);
                }
                
                // actually draw the time
                ui.label(big_time);

                ui.add_space(20.0);

                if self.seconds_left == 0 {
                    // Uppercase GREEN
                    ui.label(egui::RichText::new("Take a break!").size(30.0).color(egui::Color32::GREEN));
                }

                //Button
                let button_text = if self.is_running {
                    "Stop"
                } else {
                    "Start"
                };

                if ui.add(egui::Button::new(button_text).min_size(egui::vec2(120.0, 40.0))).clicked() {
                    self.is_running = !self.is_running;
                    if self.is_running {
                        self.last_tick = std::time::Instant::now();
                    }
                }

                ui.add_space(10.0);

                //reset button
                if ui.button("Reset").clicked() {
                    self.is_running = false;
                    self.seconds_left = 1500; //set time back to 25min
                }

                // Countdown Logic
                if self.is_running && self.seconds_left > 0 { //if there is tiem left
                    
                    // This is the "Real Time" check
                    if self.last_tick.elapsed().as_secs() >= 1 {
                        self.seconds_left -= 1; //minus 1 second
                        self.last_tick = std::time::Instant::now(); // reset the marker
                    }
                    
                    // This line is CRITICAL. It tells the app: 
                    // "Even if the user doesn't move the mouse, run this loop again ASAP!"
                    ctx.request_repaint(); 
                }
            });
        });
    }
}

// --- START OF THE GATED MAIN FUNCTIONS ---

// This part runs on Windows, Mac, and Linux
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Pomodoro Timer",
        native_options,
        Box::new(|cc| Ok(Box::new(PomodoroApp::new(cc)))),
    )
}

// to run in the browser (this was done by AI)
// This part runs in the Web Browser (WASM)
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect console errors to the browser dev tools
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        // We find the canvas element in your index.html
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id("the_canvas_id")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        eframe::WebRunner::new()
            .start(
                canvas, // Now passing the actual canvas element!
                web_options,
                Box::new(|cc| Ok(Box::new(PomodoroApp::new(cc)))),
            )
            .await
            .expect("failed to start eframe");
    });
}

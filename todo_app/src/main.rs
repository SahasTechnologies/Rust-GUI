use eframe::egui; // Fixed: lowercase 'u'

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

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Pomodoro Timer",
        native_options,
        Box::new(|cc| Ok(Box::new(PomodoroApp::new(cc)))),
    )
}

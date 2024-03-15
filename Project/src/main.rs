use eframe::{egui, run_native};
use hotkeys::HotkeysConfig;
mod draw;
mod gui;
mod gui_base;
mod hotkeys;
mod screenshot;
mod hotkey_handlers;

#[derive(Clone,Debug)]
pub struct HighlighterLine{
    pub line: Vec<egui::Pos2>,//Vec containing points of the highliter
    pub width: u32,
}

impl HighlighterLine {
    fn new() -> Self {
        Self {
            line: Vec::new(),
            width: 20,
        }
    }
}

#[derive(Clone)]
pub struct MyScreen {
    screens: Vec<u8>,
    size: (usize, usize),
    rect: Option<egui::Rect>,
}

impl MyScreen{
    fn new( screen: Option<Vec<u8>>, size: Option<(usize, usize)>) -> Self {
        Self {
            screens: if screen.is_some(){ screen.unwrap()}else{Vec::new()},
            size: if size.is_some(){size.unwrap()}else{(0,0)},
            rect: None
        }
    }
}

#[derive(Clone)]
pub struct MyDraw{
    draw: gui::Paints,
    start: Option<egui::Pos2>,
    end: Option<egui::Pos2>,
    color: Option<egui::Color32>,
    points: Option<HighlighterLine>,
    text: String
}

impl MyDraw {
    fn new(draw: gui::Paints, color: egui::Color32) -> Self {
        Self {
                draw: draw,
                start: None,
                end: None,
                color: Some(color),
                points: if draw==gui::Paints::Highlighter{ Some(HighlighterLine::new())}else if draw==gui::Paints::Eraser{Some(HighlighterLine::new())}else{None},
                text: String::from("")
            }
    
    }

}

pub struct MyApp {
    hotkey_conf: HotkeysConfig,
    output_format: String,
    mode: i32,
    image: Vec<MyScreen>,
    default_name_index: i32,
    area: (Option<egui::Pos2>, Option<egui::Pos2>, i32),
    delay_time: u32,
    n_monitor: usize,
    default_path: String,
    paint: Vec<MyDraw>,
    def_paint: Vec<MyDraw>,
    edit_color:egui::Color32,
    time: f64,
    confirm_hotkey:bool,
    edit_image: MyScreen,  

}

impl MyApp {


    //costructor for MyApp
    fn new() -> MyApp {
        let default_output_format = String::from(".jpg"); //default output format
                                                      //initial static hotkeys list
        MyApp {
            hotkey_conf: HotkeysConfig::new(),
            output_format: default_output_format,
            mode: 0,
            image: Vec::new(),
            area: (None, None, -1),
            default_name_index: 0,
            delay_time: 0,
            n_monitor: 0,
            //use frontslashes to be compatible with different OS
            default_path: String::from("./screenshot_default"), //default screenshot save location, used by save hotkey
            paint: Vec::new(),
            def_paint: Vec::new(),
            edit_color:egui::Color32::BLACK,
            time: 0.0,
            confirm_hotkey:true,
            edit_image: MyScreen::new(None, None),
          
        }
    }
}

//implementing eframe::App trait for MyApp

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    //mandatory function for App trait
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        //custom window frame

        if self.mode == 0 {
            gui_base::custom_window_frame(self,ctx,frame,
                "Screenshot Utility Tool", //the title in this row is used
                |my_app: &mut Self, frame: &mut eframe::Frame, ui| {
                    gui::gui_mode0(my_app, frame, ui);
                },
            );

        }if self.mode == 7 {
            gui_base::custom_window_frame(self,ctx,frame,
                "Screenshot Utility Tool", //the title in this row is used
                |my_app: &mut Self, _frame: &mut eframe::Frame, ui| {
                    gui::gui_mode_setting(my_app, ui);
                },
            );

        }else if self.mode == 1 {
            gui_base::custom_window_frame(self,ctx,frame,
                "Screenshot Utility Tool", //the title in this row is used
                |my_app: &mut Self, _frame: &mut eframe::Frame, ui| {
                    if ui.input(|i|i.time)- my_app.time>0.2{
                        my_app.mode = 2;
                    }
                        ui.ctx().request_repaint();
                }
            );
        } else if self.mode == 2 {
            self.mode = 5; //go to editing mode
            self.image = screenshot::full_screen();
            frame.set_window_size(egui::Vec2 { x: 640.0, y: 480.0 });
            
            if self.image.len() > 1 {
                self.mode = 3;// go to screen selection mode
            }
            else{
                frame.set_fullscreen(true);
            }
        } else if self.mode == 3 {
            //check for multiple monitors
            gui_base::custom_window_frame(self,ctx,frame,
                "Screen Selection", //the title in this row is used
                |my_app: &mut Self, frame: &mut eframe::Frame, ui| {
                    gui::gui_mode3(my_app, ui, frame);
                },
            );
            
        } else if self.mode == 4 {
            gui_base::custom_window_frame(self,ctx,frame,
                "Screenshot Utility Tool", //the title in this row is used
                |my_app: &mut Self, frame: &mut eframe::Frame, ui| {
                    gui::gui_mode4(my_app, frame, ui);
                },
            );
        } else if self.mode == 5 {
            gui_base::custom_window_frame(self,ctx,frame,
                "Cropping Mode", //the title in this row is used
                |my_app: &mut Self, frame: &mut eframe::Frame, ui| {
                    gui::gui_mode5(my_app, frame, ui);
                },
            );
        }
        else if self.mode==6{
            gui_base::custom_window_frame(self,ctx,frame,
                "Editing Mode", //the title in this row is used
                |my_app: &mut Self, frame: &mut eframe::Frame, ui| {
                    gui::gui_mode6(my_app, frame, ui);
                },
            );

        }
    }
    //executed after every update rendering
    fn post_rendering(& mut self , _window_size: [u32; 2], frame: &eframe::Frame) {
  
        if let Some(screenshot) = frame.screenshot() {
            let frame=frame.info().window_info.size;
            let limits =(self.image[self.n_monitor].rect.unwrap().min, self.image[self.n_monitor].rect.unwrap().max); 

            let pixels_per_point = Some((screenshot.pixels.len()/((_window_size[0]*_window_size[1]) as usize) )as f32);
    
                let region = egui::Rect::from_min_max(
                    egui::pos2(limits.0.x*_window_size[0] as f32/frame[0], limits.0.y*_window_size[0] as f32/frame[0]),
                    egui::pos2((limits.1.x*_window_size[1] as f32)/frame[1], (limits.1.y*_window_size[1] as f32)/frame[1])
                );
                let my_screenshot=screenshot.region(&region, pixels_per_point);
                self.edit_image.screens = my_screenshot.as_raw().to_vec();
                self.edit_image.size= ((my_screenshot.size[0]), my_screenshot.size[1]); 
        }
    }
    
}

fn main() {
    //GUI(eframe) setup

    let native_options = eframe::NativeOptions {
        //options for GUI window
        decorated: false,  //roundede corners
        transparent: true, //no OS-specific bar
        follow_system_theme: false,
        default_theme: eframe::Theme::Light,
        resizable: true,
        //initial_window_pos: Some(egui::pos2(0.0,0.0)),
        ..Default::default()
    };
    //let native_options=eframe::NativeOptions::default();

    //app_name,native_options,app_creator, when usign the custom frame window the name in the first field is not used
    run_native("Screenshot Utility Tool",native_options,Box::new(|_cc| Box::new(MyApp::new()))).expect("A probelem has occurred while starting up!");//|_cc| dummy closure, needed to make on the fly function
}

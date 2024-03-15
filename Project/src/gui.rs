use crate::{draw, hotkeys, screenshot,hotkey_handlers, MyApp, MyScreen};
use arboard;
use native_dialog::MessageDialog;
use eframe::egui;
use native_dialog::FileDialog;
use std::borrow::Cow;
use std::thread;
use std::time::Duration;


#[derive(PartialEq,Clone,Copy)]
pub enum Paints {
    Arrow,
    Text,
    Square,
    Circle,
    Highlighter,
    Eraser,
    NoFigure
}

pub fn gui_mode0(my_app: &mut MyApp, frame: &mut eframe::Frame, ui: &mut egui::Ui) {

    if !frame.info().window_info.fullscreen && !frame.info().window_info.maximized{
        frame.set_window_size(egui::Vec2 { x: 640.0, y: 480.0 });
    }
    ui.label(egui::RichText::new(
            "Welcome to the Screenshot Utility Tool, everything is ready to take a screenshot!")
        .font(egui::FontId::proportional(17.5)),
    );
    ui.add_space(20.0);
    ui.horizontal(|ui|{
        ui.add_space((frame.info().window_info.size.x/2.0)-80.0);
    ui.label(egui::RichText::new("Instruction:").font(egui::FontId::proportional(17.0)));
    });
    ui.label(egui::RichText::new("For a quick acquisition with default settings use the Hotkeys").font(egui::FontId::proportional(17.0)));
    ui.label(egui::RichText::new("For an acquisition with customized settings use the 'Take Screenshot' Button").font(egui::FontId::proportional(17.0)));
    ui.add_space(20.0);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Hotkey List:").font(egui::FontId::proportional(17.0)));
    });
    ui.add_space(10.0);
    //Displaying curretnly selected Hotkeys and radio button for format selection
    ui.horizontal(|ui|{
        hotkeys::display_shortcut(my_app, ui);
    });
 

    ui.add_space(40.0); //space between first and second group of widget

    

    ui.horizontal(|ui| {
        //to place widgets on the same row
        ui.add_space((frame.info().window_info.size.x/2.0)-80.0);
        if ui.add_sized([80.0,20.0],egui::Button::new("Take Screenshot!")).clicked()
        {
            if my_app.delay_time != 0 {
                thread::sleep(Duration::new(u64::from(my_app.delay_time), 0)); 
            }
            frame.set_window_size(egui::Vec2 { x: 0.0, y: 0.0 });
            my_app.time = ui.input(|i| i.time);
            my_app.area = (None, None,-1);
            my_app.edit_image=MyScreen::new(None, None);
            my_app.def_paint.clear();
            my_app.paint.clear();
            my_app.mode = 1;
            my_app.n_monitor = 0;
        }
    });
    

    let ev = my_app.hotkey_conf.listen_to_event();
    hotkey_handlers::hotkey_handler_mode0(ev,my_app,ui,frame);
}
pub fn gui_mode_setting(my_app:&mut MyApp,ui:&mut egui::Ui){
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(egui::RichText::new("Set delay:").font(egui::FontId::proportional(17.0)));
            ui.add_space(10.0);
            ui.add(egui::Slider::new(&mut my_app.delay_time, 0..=10).text("Delay in seconds"));
        });
        ui.add_space(200.0);//Horizontal space between slider and radio button

        ui.vertical(|ui|{
        //Radio Button for format selection
        ui.label(egui::RichText::new("Format Selection:").font(egui::FontId::proportional(17.0)));
        ui.add_space(10.0);
        if ui.add(egui::RadioButton::new(my_app.output_format == ".jpg",".jpg")).clicked()
        {my_app.output_format = String::from(".jpg");}
        if ui.add(egui::RadioButton::new(my_app.output_format == ".png",".png")).clicked()
        {my_app.output_format = String::from(".png");}
        if ui.add(egui::RadioButton::new(my_app.output_format == ".gif",".gif")).clicked()
        {my_app.output_format = String::from(".gif");}
        });
    });
        // Vertical space between delay setting and default path setting
        ui.add_space(20.0);

        ui.vertical(|ui| {
            ui.label(egui::RichText::new("Current default path:").font(egui::FontId::proportional(17.0)));

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&my_app.default_path).font(egui::FontId::proportional(15.0)));

                if ui.button("Change Default Path").clicked() {
                    let path = FileDialog::new().show_open_single_dir().unwrap();
                    match path {
                        Some(path_ok) => {
                            my_app.default_path = path_ok.to_string_lossy().to_string()
                        }
                        None => my_app.mode = 7,//go back to setting window
                    }
                }
            });
            ui.add_space(50.0);
            hotkeys::edit_shortcut(my_app,ui);
        });

    ui.add_space(50.0);
    if ui.button("Confirm Settings").clicked(){
        
        if my_app.confirm_hotkey==false{
            MessageDialog::new()
            .set_title("Error")
            .set_text("Save the changing to the hotkeys before going back!")
            .show_alert()
            .unwrap();
        }else{
            //if all the hotkeys are confirmed go back to the main window

            my_app.mode=0;
        }

    }
    let ev = my_app.hotkey_conf.listen_to_event();
    hotkey_handlers::hotkey_handler_setting(ev,my_app,ui);

}

//Multiple screen support
pub fn gui_mode3(my_app: &mut MyApp, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
   
    ui.label(
        egui::RichText::new("Multiple monitors detected\nChoose the monitor to acquire")
            .font(egui::FontId::proportional(17.5)),
    );
    ui.add_space(30.0);
    ui.vertical(|ui|{
        for i in 0..my_app.image.len() {
            ui.horizontal(|ui|{
            //if needed to make the screenshot persistent on screen
            if ui.add(egui::RadioButton::new(my_app.n_monitor == i,(i+1).to_string())).clicked()
            {my_app.n_monitor = i;}
        });
        }
        screenshot::visualize_image(&mut my_app.image[my_app.n_monitor ], ui, egui::Vec2::new(700.0, 220.0), None, true,my_app.mode);
        });
                
        ui.add_space(100.0);
        ui.horizontal(|ui|{
        if ui.button("Back To Menu").clicked() {    
            frame.set_fullscreen(false);
            my_app.mode = 0;
        }
        if ui.button("Confirm").clicked() {
            my_app.mode = 5;//go to image selection mode
            frame.set_fullscreen(true);
        }
    });
    let ev = my_app.hotkey_conf.listen_to_event();

    hotkey_handlers::hotkey_handler_mode3(ev);
}

//Visualization mode
pub fn gui_mode4(my_app: &mut MyApp, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
    if my_app.edit_image.screens.len()>0{
        screenshot::visualize_image(
            &mut my_app.edit_image,
            ui,
            frame.info().window_info.size,
            Some(my_app.image[my_app.n_monitor].size),
            true,
            my_app.mode
        );
    }
    else{
        screenshot::visualize_image(&mut my_app.image[my_app.n_monitor],ui,frame.info().window_info.size,None,true,my_app.mode);
    }
    ui.horizontal(|ui| {
        if ui.button("Back To Menu").clicked() {    
            frame.set_fullscreen(false);
            my_app.mode = 0;
        }
        let window_name =
            String::from(String::from("screenshot") + &(my_app.default_name_index.to_string()));
        if ui.button("Save").clicked() {
            let mut format_for_dialog = "";
            let mut format = "";
            if my_app.output_format == ".png" {
                format_for_dialog = "PNG";
                format = "png";
            } else if my_app.output_format == ".jpg" {
                format_for_dialog = "JPG";
                format = "jpg";
            } else if my_app.output_format == ".gif" {
                format_for_dialog = "GIF";
                format = "gif";
            }
            //leave SOME as path wrapper!!!!!!!!
            //format without the "." in front
            let file_path = FileDialog::new()
                .set_filename(&window_name)
                .add_filter(format_for_dialog, &[format])
                .show_save_single_file()
                .ok()
                .unwrap();
            match file_path {
                Some(file_path) => {
                    let mut image=& my_app.image[my_app.n_monitor];
                if my_app.edit_image.screens.len()>0{
                    image=&my_app.edit_image;
                    }
                    let path_for_thread: String = file_path.to_string_lossy().to_string();
                    let image_for_thread = image.clone();
                    let output_format_for_thread = my_app.output_format.clone();
                    thread::spawn(move || {
                        screenshot::save_image(
                            &path_for_thread,
                            &image_for_thread,
                            &output_format_for_thread,
                            false
                        );
                    });
                    frame.set_fullscreen(false);
                    my_app.default_name_index = my_app.default_name_index + 1;
                    my_app.mode = 0;
                }
                None => my_app.mode = 4, //return to visualize the image
            }
        }

        if ui.button("Copy").clicked() {
            let mut clipboard = arboard::Clipboard::new().unwrap();
            let mut image=& my_app.image[my_app.n_monitor];
            if my_app.edit_image.screens.len()>0{
                image=&my_app.edit_image;
            }
            let image_data = arboard::ImageData {
                width: image.size.0,
                height: image.size.1,
                bytes: Cow::from(&image.screens),
            };
            clipboard.set_image(image_data).expect("Errore nel copy");
        }
        if ui.button("Edit").clicked(){
            my_app.mode=6;

        }
    });
    let ev = my_app.hotkey_conf.listen_to_event();

    hotkey_handlers::hotkey_handler_mode4(ev,my_app,frame);
   
}
//Mode for Cropping 
pub fn gui_mode5(my_app: &mut MyApp, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
    let position = ui.input(|i| i.pointer.hover_pos());
    let info = frame.info().window_info;
    if my_app.image[my_app.n_monitor].rect.is_none(){
        screenshot::visualize_image(&mut my_app.image[my_app.n_monitor], ui, info.size,None, false, my_app.mode);
    }
    
    let my_rect= my_app.image[my_app.n_monitor].rect.unwrap();
    let limits = (my_rect.min, my_rect.max);
    

    let props = (
        (my_app.image[my_app.n_monitor].size.0 as f32) / (limits.1.x - limits.0.x),
        (my_app.image[my_app.n_monitor].size.1 as f32) / (limits.1.y - limits.0.y),
    );

    draw::cut_rect(position, info, my_app, ui,my_rect );

    ui.horizontal(|ui| {
        if ui.button("Back To Menu").clicked(){
            frame.set_fullscreen(false);
            my_app.mode = 0;

        }
        
        if ui.button("Confirm").clicked() {
            if my_app.area.0.is_some(){
            let width = ((my_app.area.1.unwrap().x - my_app.area.0.unwrap().x)* props.0) as u32;
            let height = ((my_app.area.1.unwrap().y - my_app.area.0.unwrap().y) * props.1) as u32;
            my_app.image[my_app.n_monitor] = screenshot::screen_area(
                &mut my_app.image[my_app.n_monitor],
                ((my_app.area.0.unwrap().x- limits.0.x) * props.0) as u32,
                ((my_app.area.0.unwrap().y- limits.0.y) * props.1) as u32,
                width,
                height,
            );
        }
            my_app.mode = 4; //go back to visualization mode but with the cropped image

        }
        if ui.button("Reset Crop").clicked(){
            my_app.area=(None, None,-1);
        }
    });

    let ev = my_app.hotkey_conf.listen_to_event();

    hotkey_handlers::hotkey_handler_mode5(ev);
}

//Annotation Tool 
pub fn gui_mode6(my_app: &mut MyApp, frame: &mut eframe::Frame, ui: &mut egui::Ui){
    screenshot::visualize_image(&mut my_app.image[my_app.n_monitor], ui, frame.info().window_info.size, None, true, my_app.mode);
     let my_rect=my_app.image[my_app.n_monitor].rect.unwrap();

    if my_app.def_paint.len()>0 && my_app.paint.len()==0{
        my_app.paint.append(&mut my_app.def_paint.clone());
    }

    let mut u=my_app.paint.len();
    ui.horizontal(|ui| {
        
        if ui.button("Return").clicked() {
   
            my_app.mode = 4;// go to visualization mode
            my_app.paint.clear();
        }
        
        draw::draw_button(Paints::Square,ui, &mut my_app.paint, my_app.edit_color);
        draw::draw_button(Paints::Circle,ui, &mut my_app.paint, my_app.edit_color);
        draw::draw_button(Paints::Arrow,ui, &mut my_app.paint, my_app.edit_color);
        draw::draw_button(Paints::Text,ui, &mut my_app.paint, my_app.edit_color);
        draw::draw_button(Paints::Highlighter,ui, &mut my_app.paint, my_app.edit_color);
        draw::draw_button(Paints::Eraser,ui, &mut my_app.paint, my_app.edit_color);


        let f=ui.color_edit_button_srgba(&mut my_app.edit_color);
        u=my_app.paint.len();
        if u>0 && my_app.paint[u-1].draw!=Paints::Eraser{
        if f.clicked(){

                match my_app.paint[u-1].color{
                    Some(_) => my_app.paint[u-1].color=None,
                    None => {my_app.paint[u-1].color.replace(my_app.edit_color);},
                }
                
   
       }
       if f.clicked_elsewhere(){
                my_app.paint[u-1].color.replace(my_app.edit_color);
       }
    } 
        if u>0{
            
            if my_app.paint[u-1].color.is_some()  && my_app.paint[u-1].draw!=Paints::Eraser && my_app.paint[u-1].draw!=Paints::Highlighter &&my_app.paint[u-1].draw!=Paints::NoFigure {
                if ui.rect_contains_pointer(my_rect) {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
            draw::draw_shape(ui, my_app, my_rect); 
            }

        }
        if ui.button("Confirm Changes").clicked() {
            u=my_app.paint.len();
            if u>0 {
            
            let last= &my_app.paint[u-1];
            if (last.draw== Paints::Highlighter && last.points.clone().unwrap().line.len()==0) || last.start.is_none() {
                if last.draw== Paints::Eraser || u==1{
                    my_app.paint.pop();
                    if u!=1{
                        my_app.paint.push(crate::MyDraw::new(Paints::NoFigure, my_app.edit_color));
                    }
                    u=my_app.paint.len();
                }
                else if my_app.paint[u-1].draw!=Paints::NoFigure{
                    my_app.paint[u-1].draw=Paints::NoFigure;
                    if my_app.paint[u-1].draw== Paints::Highlighter{
                        my_app.paint[u-1].points=None;
                    }
                }
                
            }
            my_app.def_paint.clear();
            my_app.def_paint.append(&mut my_app.paint.clone());
            if my_app.def_paint.len()>0{
                frame.request_screenshot();
            }
            else{
                my_app.edit_image.screens.clear();
            }
            }
            my_app.mode = 4;// go back to visualization mode, but can't crop anymore
        }
        if u>0{
            if my_app.paint[u-1].draw==Paints::Text{
                if ui.rect_contains_pointer(my_rect) && my_app.paint.last().unwrap().text.trim()!=""{
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                draw::write_text(ui, my_app, my_rect);
            }
        }
     });
     
        if my_app.paint.last().is_some() && my_app.paint.last().unwrap().draw==Paints::Eraser &&
        my_app.paint.last().unwrap().points.is_some() {
            draw::highlight_eraser(&mut my_app.paint,ui, my_rect); 
            if u>0{
            let p=my_app.paint[u-1].points.clone().unwrap();
            draw::eraser(ui, p.line, my_rect, &mut my_app.paint);
            }
            
    }
     
     if my_app.paint.last().is_some() && my_app.paint.last().unwrap().draw==Paints::Highlighter &&
        my_app.paint.last().unwrap().points.is_some() && my_app.paint.last().unwrap().color.is_some(){
                draw::highlight_eraser(&mut my_app.paint,ui, my_rect); 
        }
     
    let painter= ui.painter().with_clip_rect(my_rect);
    for figure in my_app.paint.iter_mut() {
        let mut valid=false;
        if figure.start.is_some() && figure.end.is_some(){
            valid=true;
        }
            
        if figure.draw == Paints::Arrow  && valid{
           
            painter.arrow(
                figure.start.unwrap(),
                figure.end.unwrap() - figure.start.unwrap(),
                egui::Stroke {
                    width: 1.5,
                    color: figure.color.unwrap(),
                },
            );
        } else if figure.draw == Paints::Square && valid {

            painter.rect(
                egui::Rect::from_two_pos(figure.start.unwrap(), figure.end.unwrap()),
                egui::Rounding::none(),
                egui::Color32::TRANSPARENT,
                egui::Stroke {
                    width: 1.5,
                    color: figure.color.unwrap(),
                },
            );

        } else if figure.draw==Paints::Circle && valid{
           
            painter.circle(figure.start.unwrap(), figure.start.unwrap().distance(figure.end.unwrap()), egui::Color32::TRANSPARENT, egui::Stroke {
                    width: 1.5,
                    color: figure.color.unwrap(),//color selected with the color picker
            });
        }
        else if (figure.draw==Paints::Highlighter &&  figure.color.is_some())|| figure.draw==Paints::Eraser{
            let points= figure.points.clone().unwrap();
            let stroke=egui::Stroke::new(points.width as f32, figure.color.unwrap().linear_multiply(0.3));
            let line=egui::Shape::line(points.line, stroke);
            painter.add(line);
        }
        else if figure.draw==Paints::Text && valid && figure.color.is_some(){
            if figure.text.trim() != "" {
                
                let rect = painter.text(
                    figure.start.unwrap(),
                    egui::Align2::LEFT_TOP,
                    figure.text.clone(),
                    egui::FontId::proportional(20.0),
                    figure.color.unwrap(),
                );
                if figure.end.is_none() || figure.end.unwrap() != rect.max {
                    figure.end = Some(rect.max);
                }
            }
        }
    }

    let ev = my_app.hotkey_conf.listen_to_event();

    hotkey_handlers::hotkey_handler_mode6(ev);
}


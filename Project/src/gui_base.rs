use crate::MyApp;

pub fn custom_window_frame(
    my_app: &mut MyApp,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut MyApp, &mut eframe::Frame, &mut egui::Ui),
) {
    let panel_frame = egui::Frame {
        fill: egui::Color32::LIGHT_BLUE, //background color
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };

    //Central Panel Component that implements custom panel_frame
    egui::CentralPanel::default()
        .frame(panel_frame)
        .show(ctx, |ui| {
            let app_rect = ui.max_rect();

            let title_bar_height = 32.0;

            let title_bar_rect = {
                let mut rect = app_rect;
                rect.max.y = rect.min.y + title_bar_height;
                rect
            };

            title_bar_ui(my_app,ui, frame, title_bar_rect, title);

            // Add the contents:
            let content_rect = {
                let mut rect = app_rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
            .shrink(4.0);
            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(my_app, frame, &mut content_ui);
        });
    }

fn title_bar_ui(
    my_app:&mut MyApp,
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    title_bar_rect: eframe::epaint::Rect,
    title: &str,
) {
    

    let title_bar_response = ui.interact(
        title_bar_rect,
        egui::Id::new("title_bar"),
        egui::Sense::click(),
    );
    
    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            if my_app.mode==0{
            let res=ui
            .add(egui::Button::new(
                egui::RichText::new("⚙").size(15.0),
            ))
            .on_hover_text("Settings");
            if res.clicked(){
             my_app.mode=7;
            }}
        });
    });

    
        let painter = ui.painter();
    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        egui::Align2::CENTER_CENTER,
        title,
        egui::FontId::proportional(20.0), //title dimension
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + egui::vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + egui::vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        frame.set_maximized(!frame.info().window_info.maximized);
    } else if title_bar_response.is_pointer_button_down_on() {
        frame.drag_window();
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(my_app,ui, frame);
        });
    });
}

//function to show the close/minimize/expand icon on the frame window
fn close_maximize_minimize(my_app:&MyApp,ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    let button_height = 12.0;

    let close_response = ui
        .add(egui::Button::new(
            egui::RichText::new("❌").size(button_height),
        ))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        frame.close();
    }

    if my_app.mode!=4 && my_app.mode!=5 && my_app.mode!=6{
    if frame.info().window_info.maximized {
        let maximized_response = ui
            .add(egui::Button::new(
                egui::RichText::new("🗗").size(button_height),
            ))
            .on_hover_text("Restore window");
        if maximized_response.clicked() {
            frame.set_maximized(false);
        }
    } else {
        let maximized_response = ui
            .add(egui::Button::new(
                egui::RichText::new("🗗").size(button_height),
            ))
            .on_hover_text("Maximize window");
        if maximized_response.clicked() {
            frame.set_maximized(true);
        }
    }
}
    let minimized_response = ui
        .add(egui::Button::new(
            egui::RichText::new("🗕").size(button_height),
        ))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        frame.set_minimized(true);
    }
  
}
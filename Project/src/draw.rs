use crate::{gui::Paints, screenshot, HighlighterLine, MyApp, MyDraw};
use eframe::egui;
use geo::{Line, line_intersection::line_intersection};
use emath::Rot2;


pub fn cut_rect(
    position: Option<egui::Pos2>,
    info: eframe::WindowInfo,
    my_self: &mut MyApp,
    ui: &mut egui::Ui,
    limits: egui::Rect,
) {
    let valid ;

   
    let mut pos: egui::Pos2;

    match position {
        Some(_) => {
            pos = position.unwrap();
            if limits.contains(pos) {
                valid = true;
                if my_self.area.0.is_none() || my_self.area.1.is_none(){
                ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);

                }
                else if my_self.area.2==-1{
                    let rect1= egui::Rect::from_min_max(my_self.area.0.unwrap(), my_self.area.1.unwrap());
                    
                    if rect1.x_range().contains(&pos.x) {//resizongVertical
                        if (rect1.top()-5.0..rect1.top()+5.0).contains(&pos.y){
                            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
                    
                            my_self.area.2=0;
                            
                        }else if (rect1.bottom()-5.0..rect1.bottom()+5.0).contains(&pos.y){
                            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
              
                            my_self.area.2=1;
                        }

                    }
                    else if rect1.y_range().contains(&pos.y){//ResizingHorizontal
                        if (rect1.left()-5.0..rect1.left()+5.0).contains(&pos.x){
                            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
                            
                            my_self.area.2=2;
                            
                        }
                        else if (rect1.right()-5.0..rect1.right()+5.0).contains(&pos.x){
                            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        
                            my_self.area.2=3;
                            
                        }
                    }
                }
        }
        else{
            valid=false;
        }
    }
        None => {
            pos = egui::Pos2::default();
            valid = false;

        }
    }
        

    if my_self.area.0.is_none() || my_self.area.1.is_none(){

        if valid==true && ui.input(|i|  i.pointer.primary_pressed()) {

            my_self.area = (None, None, -1);
            let start_pos = ui.input(|i| i.pointer.press_origin()).unwrap();
            my_self.area.0.replace(start_pos);
  
            
        }


        if ui.input(|i| i.pointer.primary_released()) && my_self.area.1.is_none() && my_self.area.0.is_some(){

                pos=pos.clamp(limits.min, limits.max);

                let start= my_self.area.0.unwrap().min(pos);
                let end=my_self.area.0.unwrap().max(pos);
                my_self.area.0.replace(start);
                my_self.area.1.replace(end);

        }
    }
    else{
        //TO DO modifica bordi
        if ui.input(|i| i.pointer.primary_released() || !i.pointer.primary_down()){
            my_self.area.2=-1;
        }
   
        if valid && ui.input(|i|  i.pointer.primary_down()){

            
            if my_self.area.2==0{
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
                    my_self.area.0.replace(egui::pos2(my_self.area.0.unwrap().x, pos.y.clamp(limits.top(),my_self.area.1.unwrap().y )));
                
            }
            else if my_self.area.2==1{
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);

                my_self.area.1.replace(egui::pos2(my_self.area.1.unwrap().x,pos.y.clamp(my_self.area.0.unwrap().y , limits.bottom())));
            }
            else if my_self.area.2==2{
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
                my_self.area.0.replace(egui::pos2(pos.x.clamp(limits.left(),my_self.area.1.unwrap().x), my_self.area.0.unwrap().y));

            }
            else if my_self.area.2==3{
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
                my_self.area.1.replace(egui::pos2(pos.x.clamp(my_self.area.0.unwrap().x, limits.right() ), my_self.area.1.unwrap().y));
            }
        } 
    }

    screenshot::visualize_image(&mut my_self.image[my_self.n_monitor] , ui, info.size,None, true, my_self.mode);
    if my_self.area.0.is_some(){
        let mut my_stroke = egui::Stroke::default();
        my_stroke.color = egui::Color32::WHITE;
        my_stroke.width = 2.0;
        let my_rect:egui::Rect;

        if my_self.area.1.is_none(){
                let start= my_self.area.0.unwrap().min(pos.clamp(limits.min, limits.max));
                let end=my_self.area.0.unwrap().max(pos.clamp(limits.min, limits.max));


                my_rect = egui::Rect::from_min_max(start, end);
    
        } else {
            my_rect = egui::Rect::from_min_max(
                my_self.area.0.unwrap(),
                my_self.area.1.unwrap(),
            );
        }
       
            ui.painter().rect(
                my_rect,
                egui::Rounding::none(),
                egui::Color32::from_white_alpha(5),
                my_stroke,
            );
        
    
}
}

pub fn draw_shape(ui: &mut egui::Ui, my_app: &mut MyApp, rect: egui::Rect) {
    ui.input(|i| {

        let u = my_app.paint.len();

        if i.pointer.is_decidedly_dragging() && i.pointer.primary_down() {
            if my_app.paint[u - 1].start.is_none() {
                let pos=i.pointer.press_origin();
                if pos.is_some() && rect.contains(pos.unwrap()){   
                    my_app.paint[u - 1].start = pos;
                }
            } else {
                my_app.paint[u - 1].end = i.pointer.hover_pos();
         }
        } else if i.pointer.primary_released() && my_app.paint[u - 1].start.is_some() {
      
            my_app.paint[u - 1].end = i.pointer.hover_pos();
            my_app
                .paint
                .push(MyDraw::new(my_app.paint[u - 1].draw, my_app.edit_color));
            }

        
    });
}

pub fn write_text(ui: &mut egui::Ui, my_app: &mut MyApp,  rect: egui::Rect) {
    let u = my_app.paint.len() - 1;

    ui.input(|i| {
        
        if i.pointer.primary_down() &&  my_app.paint[u].start.is_none(){
            let pos=i.pointer.hover_pos();
            if pos.is_some() && rect.contains(pos.unwrap()) && my_app.paint[u].color.is_some(){
                my_app.paint[u].start = pos ;
                my_app.paint[u].end = pos ;
                my_app.paint.push(MyDraw::new(Paints::Text, my_app.edit_color))
                
            }
            
        }
    });

    let str_ref: &mut String = &mut my_app.paint[u].text;
    ui.add_space(250.0);
    ui.label(egui::RichText::new("Click in the image to clip text").font(egui::FontId::proportional(15.0)));
    ui.add(egui::TextEdit::singleline(str_ref).text_color(my_app.edit_color).font(egui::FontId::proportional(15.0)));
}

pub fn highlight_eraser(paint: &mut Vec<MyDraw>,ui: &mut egui::Ui,rect: egui::Rect) {
    let u=paint.len()-1;
    // let mut response = ui.allocate_rect(rect, egui::Sense::drag());
    let mut line=paint[u].points.clone().unwrap().line;
    ui.input(|i|{
        let p=i.pointer.hover_pos();
    if  p.is_some() && i.pointer.primary_down() {
        let pointer_pos= p.unwrap();
        
        if line.last() != Some(&pointer_pos)  && rect.contains(pointer_pos){
            if paint[u].draw==Paints::Eraser && line.len()==2{
                line.remove(0);
            }
            line.push( pointer_pos);
        }
    } 

    let len=line.len();
    paint[u].points.replace(HighlighterLine { line, width: 20 });
    
    if i.pointer.primary_released() && len>0{
        if paint[u].draw==Paints::Eraser{
            paint.pop();

            paint.push(MyDraw ::new(Paints::Eraser, egui::Color32::WHITE));
        }
        else if paint[u].draw==Paints::Highlighter{
    
        paint.push(MyDraw ::new(Paints::Highlighter, paint[u].color.unwrap()));//when using eraser color doesn't matter
        }
    }
});
       
}

pub fn draw_button(paint: Paints, ui: &mut egui::Ui, el: &mut Vec<MyDraw>, color: egui::Color32) {
    let mut icon: &str = "";
    if paint == Paints::Square {
        icon = "⬜";
    } else if paint == Paints::Circle {
        icon = "⭕";
    } else if paint == Paints::Arrow {
        icon = "↗";
    } else if paint == Paints::Text {
        icon = "Text";
    } else if paint == Paints::Highlighter {
        icon = "Highlighter";
    }
    else if paint==Paints::Eraser{
        icon ="Eraser";
    }

    let mut button = egui::Button::new(egui::RichText::new(icon));
    let mut u=el.len();
    if u>0 && el[u-1].draw == paint {
        button = egui::Button::new(egui::RichText::new(icon).underline());
    }

    if ui.add(button).clicked() {

        if u>0 && el[u-1].start.is_none() {
            if el[u-1].draw==paint{
                el[u-1].draw=Paints::NoFigure;
            }
            else{
                el.pop();
                u=u-1;
            }
            
        }
        
        if  !(u>0 && el[u-1].draw==Paints::NoFigure){
            if paint==Paints::Eraser{
                el.push(MyDraw::new(paint, egui::Color32::WHITE));
            }
            else{
            el.push(MyDraw::new(paint, color));
            }
        }
        

    }
}

pub fn eraser(ui: &mut egui::Ui,  points: Vec<egui::Pos2>, rect: egui::Rect, paint: &mut Vec<MyDraw>){

    ui.input(|i|{
        let p=i.pointer.hover_pos();
        if p.is_some() && i.pointer.is_decidedly_dragging() && i.pointer.primary_down() {
            let pos=p.unwrap();
            
            if rect.contains(pos){
                
                paint.retain(|x|{
                    let mut my_rect= egui::Rect::NOTHING;
                    if x.start.is_some() && x.end.is_some(){
                        
                        
                            my_rect= egui::Rect::from_min_max(x.start.unwrap(), x.end.unwrap());
                        
                        
                    }
                  
            if x.draw==Paints::Square || x.draw==Paints::Text{
                                let line=Line::new(geo::coord!{x:points[0].x, y: points[0].y},geo::coord!{x:points[1].x, y: points[1].y} );
                                let line_top=Line::new(geo::coord!{x:my_rect.left(), y: my_rect.top()},geo::coord!{x:my_rect.right(), y: my_rect.top()} );
                                let line_bottom=Line::new(geo::coord!{x:my_rect.left(), y: my_rect.bottom()},geo::coord!{x:my_rect.right(), y: my_rect.bottom()} );
                                let line_left=Line::new(geo::coord!{x:my_rect.left(), y: my_rect.top()},geo::coord!{x:my_rect.left(), y: my_rect.bottom()} );
                                let line_right=Line::new(geo::coord!{x:my_rect.right(), y: my_rect.top()},geo::coord!{x:my_rect.right(), y: my_rect.bottom()} );
                                if line_intersection(line, line_top).is_some() || line_intersection(line, line_bottom).is_some() ||
                                line_intersection(line, line_left).is_some() || line_intersection(line, line_right).is_some(){
                                    return false;
                                }
                               
                                //[left, top][right, bottom]
                                //top= min.y (ordinata più in alto- numericamente più piccola), bottom= max.y (ordinata più in basso- numericamente più grande) 
                                // if ( my_rect.x_range().contains(&pos.x) && (my_rect.top()-5.0..my_rect.top()+5.0).contains(&pos.y)) ||//lato in alto
                                // ( my_rect.x_range().contains(&pos.x)  && (my_rect.bottom()-5.0..my_rect.bottom()+5.0).contains(&pos.y) ) //lato in basso
                                // ||(my_rect.y_range().contains(&pos.y) && (my_rect.left()-5.0..my_rect.left()+5.0).contains(&pos.x) ) || //lato a sinistra
                                // (my_rect.y_range().contains(&pos.y) && (my_rect.right()-5.0..my_rect.right()+5.0).contains(&pos.x)) //lato a destra
                                //  {
                                //         return false;
                                //     } 
                                    else{
                                        return true;
                                    }
                        }
  
 
                
                    else if x.draw == Paints::Circle{
                                let c = my_rect.min;
                                let r = c.distance(my_rect.max);
                                let d1 = points[0].distance(c)-r;
                                let d2=points[1].distance(c)-r;
                                if (d1>=0.0 && d2<=0.0) ||(d1<=0.0 && d2>=0.0){
                                    return false;
                                }

                                //if pos.x as usize>=(c.x-r) as usize && pos.x as usize<=(c.x+r) as usize && pos.y as usize>=(c.y-r) as usize && pos.y as usize<=(c.y+r) as usize{
                                // if (r-5.0..r+5.0).contains(&d){
                                //     return false;
                                // }
                                else{
                                    return true;
                                }
                            }
                            else if x.draw==Paints::Arrow{
                                let line=Line::new(geo::coord!{x:points[0].x, y: points[0].y},geo::coord!{x:points[1].x, y: points[1].y} );
                                let p1 = my_rect.min;
                                let p2 = my_rect.max;
                                
                                let line1 = Line::new(geo::coord!{ x:p1.x, y:p1.y}, geo::coord!{ x:p2.x, y:p2.y});

                                let vec = p2 - p1;

                                let rot = Rot2::from_angle(std::f32::consts::TAU / 10.0);
                                let tip_length = vec.length() / 4.0;
                                let tip = p2;
                                let dir = vec.normalized();

                                let p3 = tip - tip_length * (rot * dir);
                                let p4 = tip - tip_length * (rot.inverse() * dir);

                                let line2 = Line::new(geo::coord!{ x:p2.x, y:p2.y}, geo::coord!{ x:p3.x, y:p3.y});
                                let line3 = Line::new(geo::coord!{ x:p2.x, y:p2.y}, geo::coord!{ x:p4.x, y:p4.y});                                
    
                                // let a = p2.y - p1.y;
                                // let b = p1.x - p2.x;
                                // let c = (p2.x - p1.x) * p1.y - (p2.y - p1.y) * p1.x;
    
                                // let dist = (a*pos.x + b*pos.y + c).abs()/(a*a + b*b).sqrt();
                                //if ((pos.x-p1.x)/(p2.x-p1.x)) == ((pos.y-p1.y)/(p2.y-p1.y)) && ((p1.x >= pos.x && p2.x <= pos.x) || (p1.x <= pos.x && p2.x >= pos.x)){  da sistemare cercando di implementare il range
                                if line_intersection(line, line1).is_some() || line_intersection(line, line2).is_some() ||line_intersection(line, line3).is_some(){
                                    return false;
                                } 
                                return true;
                            }
                    
                
                else if x.draw == Paints::Highlighter{
                                if let Some(a) = &x.points{
                                    if a.line.len()>0{
                                        let line=Line::new(geo::coord!{x:points[0].x, y: points[0].y},geo::coord!{x:points[1].x, y: points[1].y} );
                                        for i in 0..a.line.len()-1{
                                            let p1 = a.line[i];
                                            let p2 = a.line[i+1];
                                            let line_hight=Line::new(geo::coord!{x: p1.x, y: p1.y},geo::coord!{x:p2.x, y: p2.y} );
                                            if line_intersection(line,line_hight).is_some(){
                                                return  false;
                                            }
                                            // if pos.x as usize>=cmp::min(p1.x as usize, p2.x as usize)-10 && pos.x as usize<=cmp::max(p1.x as usize, p2.x as usize)+10
                                            // && pos.y as usize>=cmp::min(p1.y as usize, p2.y as usize)-10 && pos.y as usize<=cmp::max(p1.y as usize, p2.y as usize)+10{
                                            //     return false;
                                            // }
                                        }
                                    }

                                    return true;
                                }
                                else{
                                    return true;
                                }

                        }
                            else{
                                return true;
                            }
                            
                           
                });
            }
            
        }
    });

    // }
}
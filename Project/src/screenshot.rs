use eframe::egui;
use egui::{ColorImage, Ui};
// use gif::{Encoder, Frame};  servono per save as gif (forse non necessaria)
//use std::fs::File;
use image::{self, ImageFormat};
use screenshots::Screen;

use crate::MyScreen;

struct MyImage {
    texture: Option<egui::TextureHandle>,
    rect: Option<egui::Rect>
}

impl MyImage {
    fn ui(&mut self, ui: &mut egui::Ui, im: ColorImage, size: egui::Vec2, im_size: (usize, usize), show: bool)->egui::Rect {
        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx().load_texture("my-image", im, Default::default())
        });
        let x=(size.x-im_size.0 as f32)/2.0;
        let y=(size.y-im_size.1 as f32)/2.0;
        let mut start=egui::pos2(if x>10.0{x}else{10.0}, if y>80.0{y}else{80.0});
        let mut max_size=egui::Pos2::default();
     
        
        if im_size.1>im_size.0{
                max_size.y=if im_size.1 as f32+start.y>size.y{size.y-10.0}else{im_size.1 as f32+start.y};
                max_size.x= start.x+ ((max_size.y-start.y)*im_size.0 as f32)/im_size.1 as f32;
            
        }
        else{
            max_size.x=if im_size.0 as f32+start.x>size.x{size.x-10.0}else{im_size.0 as f32+start.x};
            max_size.y= start.y+ ((max_size.x-start.x)*im_size.1 as f32)/im_size.0 as f32;
            if max_size.y>size.y{
                max_size.y=size.y-10.0;
                let width=((max_size.y-start.y)*im_size.0 as f32)/im_size.1 as f32;
                start.x=(size.x-width)/2.0;
                max_size.x= start.x+ ((max_size.y-start.y)*im_size.0 as f32)/im_size.1 as f32;
            }
        }
    
        

        let my_rect = if self.rect.is_some() && self.rect.unwrap().min== start && self.rect.unwrap().max==max_size{self.rect.unwrap()}else{egui::Rect::from_min_max(start, max_size)};
        if show{
        ui.painter().image(
            texture.id(),
            my_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }
        my_rect
    }
}

pub fn full_screen() -> Vec<MyScreen> {
    let screens = Screen::all().unwrap();
    let mut screen_image = Vec::new();
    for screen in screens {
        let img = screen.capture().unwrap();
        let image = MyScreen::new(Some(img.rgba().to_vec()), Some((img.width() as usize, img.height() as usize)));
        screen_image.push(image);
    }
    screen_image
}

pub fn visualize_image(image: &mut MyScreen, ui: &mut Ui, size: egui::Vec2,dim: Option<(usize, usize)>, show: bool, mode: i32) {

        let mut my_image = MyImage { texture: None, rect: image.rect };
        let im =
            egui::ColorImage::from_rgba_unmultiplied([image.size.0, image.size.1], &image.screens);
            match dim {
                Some(_) => {image.rect=Some(my_image.ui(ui, im, size, dim.unwrap(), show));},
                None => {if mode==3{
                    my_image.ui(ui, im, size, image.size, show);
                }
                else {
                    
                    image.rect=Some(my_image.ui(ui, im, size, image.size, show));
                }
                    },
            }
                

        
    
}

pub fn screen_area(image: &mut MyScreen, x0: u32, y0: u32, width: u32, height: u32)->MyScreen {

   
        let rgba_img = image::RgbaImage::from_raw(
            image.size.0 as u32,
            image.size.1 as u32,
            image.screens.to_vec(),
        )
        .expect("Errore nella conversione dell'immagine");
        let cropped_img = image::ImageBuffer::from_fn(width, height, |x, y| {
            rgba_img.get_pixel(x0 + x, y0 + y).clone()
        });

        let mut cropped_bytes = Vec::new();

        for pixel in cropped_img.pixels() {
            cropped_bytes.push(pixel[0]); // Red
            cropped_bytes.push(pixel[1]); // Green
            cropped_bytes.push(pixel[2]); // Blue
            cropped_bytes.push(pixel[3]); // Alpha
        }
        let img = MyScreen::new(Some(cropped_bytes), Some((width as usize, height as usize)));

    
    img
}

pub fn save_image(path: &String, image: &MyScreen, format: &String,use_format:bool) {
    let image_format = if format == ".jpg" {
        ImageFormat::Jpeg
    } else if format == ".png" {
        ImageFormat::Png
    } else {
        ImageFormat::Gif
    };

        let img_buf = image::ImageBuffer::<image::Rgba<u8>, _>::from_vec(
            image.size.0 as u32,
            image.size.1 as u32,
            image.screens.to_vec(),
        )
        .expect("impossibile creare l'immagine");
        if use_format==true{
            img_buf
            .save_with_format(path.to_string()+format, image_format)
            .expect("impossibile salvare l'immagine");
        }else{
        img_buf
            .save_with_format(path.to_string(), image_format)
            .expect("impossibile salvare l'immagine");
        }
    
}

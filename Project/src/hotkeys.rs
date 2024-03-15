use global_hotkey::{GlobalHotKeyManager, hotkey::HotKey, GlobalHotKeyEvent};
use keyboard_types::{Code, Modifiers};
use native_dialog::MessageDialog;
use crate::MyApp;

pub struct HotkeysConfig{
    hotkeys: Vec<HotKey>,
    hotkeys_seq: Vec<(Option<Modifiers>, Code)>,
    hotkeys_string: Vec<(String, String)>,
    commands: Vec<String>,
    new_key: (Code, String), //new key to be decided
    new_mod: (Option<Modifiers>, String), //new modifier to be decided
    enable: bool,  //used to enable the modification of a hotkey
    changed_hotkey: usize, //what hotkey do I want to change
    manager: GlobalHotKeyManager,
}

impl HotkeysConfig{
    pub fn new() -> HotkeysConfig{
        let h:Vec<HotKey> = vec![HotKey::new(Some(Modifiers::CONTROL), Code::KeyA), HotKey::new(Some(Modifiers::CONTROL), Code::KeyS), HotKey::new(Some(Modifiers::CONTROL), Code::KeyC)];
        let j:Vec<(Option<Modifiers>, Code)> = vec![(Some(Modifiers::CONTROL), Code::KeyA), (Some(Modifiers::CONTROL), Code::KeyS), (Some(Modifiers::CONTROL), Code::KeyC)];
        let com:Vec<String> = vec!["Take screenshot".to_string(), "Save".to_string(), "Copy".to_string()];
        let man = GlobalHotKeyManager::new().unwrap();
        man.register_all(&h).unwrap(); //Registering default hotkeys

        HotkeysConfig { hotkeys: h, hotkeys_seq: j, hotkeys_string: vec![("CTRL".to_string(), "A".to_string()), ("CTRL".to_string(), "S".to_string()),("CTRL".to_string(), "C".to_string())], new_key: (Code::KeyA, "A".to_string()), new_mod: (Some(Modifiers::CONTROL), "CTRL".to_string()), enable: true, changed_hotkey:0, commands: com, manager: man}
    }

    pub fn get_new_key(self: &Self) -> (Code, String){
        return self.new_key.clone();
    }

    pub fn get_new_mod(self: &Self) -> (Option<Modifiers>, String){
        return self.new_mod.clone();
    }

    pub fn get_changed_hotkey(self: &Self) -> usize{
        return self.changed_hotkey;
    }

    pub fn get_enable(self: &Self) -> bool{
        return self.enable;  
    }

    pub fn set_new_hotkey(self: &mut Self, new_mod:(Option<Modifiers>, String), new_key: (Code, String)){
        self.new_mod = new_mod;
        self.new_key = new_key;
    }

    pub fn set_enable(self: &mut Self, en: bool){
        self.enable = en;
    }

    pub fn get_hotkeys_len(self: &Self) -> usize{
        return self.hotkeys.len();
    }

    pub fn get_command(self: &Self, i: usize) -> &String{
        return &self.commands[i];
    }

    pub fn get_hotkey_as_string(self: &Self, i:usize) -> String{

        if self.hotkeys_string[i].0.eq(""){
            return self.hotkeys_string[i].1.clone();
        }
        else{
            return self.hotkeys_string[i].0.clone() + "+" + &self.hotkeys_string[i].1;
        }
    }

    pub fn listen_to_event(self:&Self) -> Option<usize>{
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            for i in 0..self.hotkeys.len(){
                if event.id == self.hotkeys[i].id(){
                    return Some(i);
                }
            }
            return None;
        }
        else{
            return None;
        }
    }

    pub fn delete_hotkey(self: &mut Self, i:usize){
        self.manager.unregister(self.hotkeys[i]).unwrap();
        self.new_mod=(self.hotkeys_seq[i].0, self.hotkeys_string[i].0.clone());
        self.new_key=(self.hotkeys_seq[i].1, self.hotkeys_string[i].1.clone());
        self.changed_hotkey = i;
        self.enable = false;
    }

    pub fn change_hotkey(self: &mut Self, i: usize, modifier: (Option<Modifiers>, String), key: (Code, String)) -> bool{
        for c in 0..self.hotkeys_string.len(){
            if self.hotkeys_string[c].0.eq(&modifier.1) && self.hotkeys_string[c].1.eq(&key.1) && c != i {
                return false;
            }
        }
        self.hotkeys_string[i]=(modifier.1, key.1);
        self.hotkeys_seq[i]=(modifier.0, key.0);
        self.hotkeys[i] = HotKey::new(modifier.0, key.0);
        self.manager.register(self.hotkeys[i]).unwrap();
        return true;
    }
}

pub fn display_shortcut(my_app: &mut MyApp,ui:&mut egui::Ui){
    ui.vertical(|ui|{

        for i in 0..my_app.hotkey_conf.get_hotkeys_len() {
            ui.horizontal(|ui| {
    
                let u = my_app.hotkey_conf.get_hotkey_as_string(i);
    
                ui.label(egui::RichText::new(my_app.hotkey_conf.get_command(i).to_owned()+&" :".to_string()).font(egui::FontId::proportional(14.0)));
    
                ui.label(egui::RichText::new(u).font(egui::FontId::proportional(14.0)));
            });
    
        }
    });
}
    
pub fn edit_shortcut(my_app: &mut MyApp, ui: &mut egui::Ui){
    ui.label(egui::RichText::new("Click on the shortcut to edit it").font(egui::FontId::proportional(17.0)),);
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        //hotkeys display
        ui.vertical(|ui| {

            for i in 0..my_app.hotkey_conf.get_hotkeys_len() {
                ui.horizontal(|ui| {
                    let u = my_app.hotkey_conf.get_hotkey_as_string(i);
                    ui.label(
                        egui::RichText::new(my_app.hotkey_conf.get_command(i))
                            .font(egui::FontId::proportional(14.0)),
                    );

                    if my_app.hotkey_conf.get_enable() {
                        if ui
                            .link(egui::RichText::new(u).font(egui::FontId::proportional(14.0)))
                            .clicked()
                        {
                            //If I click on the link, I unregister the hotkey
                            my_app.hotkey_conf.delete_hotkey(i);
                            my_app.confirm_hotkey=false;
                        };
                    } else {
                        let mut new_mod = my_app.hotkey_conf.get_new_mod();
                        let mut new_key = my_app.hotkey_conf.get_new_key();
                        if i != my_app.hotkey_conf.get_changed_hotkey() {
                            ui.label(u);
                        } else {
                            ui.vertical(|ui| {
                                egui::ComboBox::from_label("Set new modifier")
                                    .selected_text(format!("{}", new_mod.1))
                                    .show_ui(ui, |ui| {

                                    ui.selectable_value(&mut new_mod,(Some(Modifiers::SHIFT), "SHIFT".to_string()),"SHIFT");
                                    ui.selectable_value(&mut new_mod,(Some(Modifiers::CONTROL), "CTRL".to_string()),"CTRL");                                    
                                    });
                                egui::ComboBox::from_label("Set new key")
                                    .selected_text(format!("{}", new_key.1))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyA, "A".to_string()),
                                            "A",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyB, "B".to_string()),
                                            "B",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyC, "C".to_string()),
                                            "C",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyD, "D".to_string()),
                                            "D",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyE, "E".to_string()),
                                            "E",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyF, "F".to_string()),
                                            "F",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyG, "G".to_string()),
                                            "G",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyH, "H".to_string()),
                                            "H",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyI, "I".to_string()),
                                            "I",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyJ, "J".to_string()),
                                            "J",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyK, "K".to_string()),
                                            "K",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyL, "L".to_string()),
                                            "L",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyM, "M".to_string()),
                                            "M",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyN, "N".to_string()),
                                            "N",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyO, "O".to_string()),
                                            "O",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyP, "P".to_string()),
                                            "P",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyQ, "Q".to_string()),
                                            "Q",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyR, "R".to_string()),
                                            "R",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyS, "S".to_string()),
                                            "S",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyT, "T".to_string()),
                                            "T",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyU, "U".to_string()),
                                            "U",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyV, "V".to_string()),
                                            "V",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyW, "W".to_string()),
                                            "W",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyX, "X".to_string()),
                                            "X",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyY, "Y".to_string()),
                                            "Y",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyZ, "Z".to_string()),
                                            "Z",
                                        );
                                    });

                                if ui.button("Save").clicked() {
                                    let success = my_app.hotkey_conf.change_hotkey(
                                        i,
                                        new_mod.clone(),
                                        new_key.clone(),
                                    );
                                    if success {
                                        //modification could fail if for example I try to set an already registered hotkey
                                        my_app.hotkey_conf.set_enable(true);
                                        my_app.confirm_hotkey=true;
                                    }else{
                                        MessageDialog::new()
                                        .set_title("Error")
                                        .set_text("Hotkey already used!")
                                        .show_alert()
                                        .unwrap();
                                    }
                                }
                                my_app.hotkey_conf.set_new_hotkey(new_mod, new_key);
                            });
                        }
                    }
                });
            }
        });
    });
}
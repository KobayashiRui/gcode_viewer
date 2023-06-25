#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use core::num;
use std::io::Read;
use std::{num::NonZeroU64, sync::Arc};
use std::fs;
use std::path::Path;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;


use eframe::{
    egui_wgpu::wgpu::util::DeviceExt,
    egui_wgpu::{self, wgpu},
    epaint,
};

use eframe::egui;
use eframe::{egui::*};
use egui::Color32;

use egui_extras::{Size, StripBuilder};

use egui_clip_textedit::ClipTextEdit;

struct ReadFileData{
    file_name: String,
    file_data : String,
}

pub struct GcodeTextEditor{
    pub text_edit: ClipTextEdit,
    //gcode_data: String,
    //line_data:  String,
    //selected_row:  usize,
    picked_path: Option<String>,
    read_file_receiver: Receiver<ReadFileData>,
    read_file_sender: Sender<ReadFileData>,
} 

fn count_lines(s: &String) -> usize {
    s.chars().filter(|&c| c == '\n').count() + 1
}

fn line_size_to_string(s: usize) -> String {

    let src: Vec<usize> = (1..(s+1)).collect();
    let dst: Vec<String> = src.iter().map(|x| x.to_string()).collect();
    
    dst.join("\n")
}

impl GcodeTextEditor {

    pub fn new() -> Self{

        let (tx, rx): (Sender<ReadFileData>, Receiver<ReadFileData>) = mpsc::channel();

        Self{
            text_edit: ClipTextEdit::new("G1 X100 \nG1 X200".to_string()),
            //line_data: "1\n2\n".to_string(),
            //selected_row: 0,
            picked_path: None,
            read_file_receiver: rx,
            read_file_sender: tx,
        }
    }


    async fn read_file(&mut self){
        let file = rfd::AsyncFileDialog::new()
                            .pick_file()
                            .await;
        let data = file.unwrap().read().await;
        let content = String::from_utf8(data).unwrap();
        println!("Read file: {}", content);
        //self.line_data = line_size_to_string(count_lines(&content));
        //self.gcode_data = content;
    }

    pub fn update(&mut self, ui: &mut egui::Ui){
        //let Self {mut gcode_data , mut selected_row, mut picked_path} = self;

        let dark_mode = ui.visuals().dark_mode;
        let faded_color = ui.visuals().window_fill();
        let faded_color = |color: Color32| -> Color32 {
            use egui::Rgba;
            let t = if dark_mode { 0.95 } else { 0.8 };
            egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
        };

        match self.read_file_receiver.try_recv() {
            Ok(rf) =>{
                //println!("Counter: {}", counter);
                println!("Get Message");
                //self.line_data = line_size_to_string(count_lines(&rf.file_data));
                //self.gcode_data = rf.file_data;
                self.text_edit.load_text(rf.file_data);
            },
            Err(e)=>{},
        };

        ui.label("Gcode File");
        if ui.button("Open file…").clicked() {
            let thread_tx = self.read_file_sender.clone();
            execute(async move{
                let file = rfd::AsyncFileDialog::new()
                                    .pick_file()
                                    .await;
                let data = file.unwrap().read().await;
                let content = String::from_utf8(data).unwrap();
                thread_tx.send(ReadFileData{file_name:"".to_string(), file_data:content}).unwrap();
            });
        }
        if let Some(picked_path) = &self.picked_path {
            ui.horizontal(|ui| {
                ui.label("Picked file:");
                ui.monospace(picked_path);

            });

        }

        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

        let mut text_edit_rect = ui.available_rect_before_wrap();
        self.text_edit.show_editor(ui, text_edit_rect);

    }
}

use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    // this is stupid... use any executor of your choice instead
    std::thread::spawn(move || futures::executor::block_on(f));
}

#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
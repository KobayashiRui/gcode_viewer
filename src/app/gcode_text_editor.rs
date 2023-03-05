#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use core::num;
use std::{num::NonZeroU64, sync::Arc};
use std::fs;
use std::path::Path;

use eframe::{
    egui_wgpu::wgpu::util::DeviceExt,
    egui_wgpu::{self, wgpu},
    epaint,
};

use eframe::egui;
use egui::Color32;
use egui_extras::{Size, StripBuilder};

pub struct GcodeTextEditor {
    gcode_data: String,
    line_data: String,
    selected_row: usize,
    text_slider: i32,
    picked_path: Option<String>,
    text_row: Vec<String>,
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
        let mut text_row: Vec<String> = vec![];
        let num_rows = 50000;
        for row in 0..num_rows {
            let mut text = format!("This is row {}/{}", row + 1, num_rows);
            text_row.push(text);
            //egui::TextEdit::singleline(&mut text).show(ui);
        }

        Self{
            gcode_data: "G1 X100 \nG1 X200".to_string(),
            line_data: "1\n2\n".to_string(),
            selected_row: 0,
            text_slider: 0,
            picked_path: None,
            text_row: text_row,
        }
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

        ui.label("Gcode File");
        if ui.button("Open file…").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                self.picked_path = Some(path.display().to_string());

                println!("Update picked path: {:?}", self.picked_path);

                //load file
                if let Some(picked_path) = &self.picked_path {
                    let path_data = Path::new(&picked_path);
                    match fs::read_to_string(path_data) {
                        Ok(content) => {
                            //println!("{}", content);
                            self.line_data = line_size_to_string(count_lines(&content));
                            self.gcode_data = content;
                        }
                        Err(error) => {println!("Please Select Gcode File")},
                    }
                    
                }

                }
        }
        if let Some(picked_path) = &self.picked_path {
            ui.horizontal(|ui| {
                ui.label("Picked file:");
                ui.monospace(picked_path);

            });

        }



        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

        /* 
        egui::ScrollArea::vertical()
            .max_height(500.0)
            .show(ui,  |ui| {

        StripBuilder::new(ui)
            .size(Size::remainder())
            .size(Size::remainder())
            .vertical(|mut strip| {
                strip.strip(|builder| {
                    //builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                    builder
                        .size(Size::exact(100.0))
                        .size(Size::exact(400.0))
                        .size(Size::exact(20.0))
                        .horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.spacing_mut().item_spacing = egui::vec2(5.0, 0.0);  
                                //ui.style_mut().visuals.extreme_bg_color = Color32::from_rgb(53, 140, 100);
                                //ui.style_mut().visuals.override_text_color = Some(Color32::from_rgb(33, 33, 33));
                                //ui.style_mut().visuals.extreme_bg_color = Color32::from_rgb(164, 192, 176);
                                egui::Frame::none()
                                    .fill(Color32::from_rgb(53, 140, 100))
                                    .show(ui, |ui| {

                                        //let mut line_data = &line_size_to_string(count_lines(&self.gcode_data)) as &str;
                                        let mut l = &self.line_data as &str;
                                        let text_edit_output = egui::TextEdit::multiline(&mut l)
                                                                            //.min_size(ui.available_size())
                                                                            //.layouter(&mut layouter)
                                                                            .show(ui);

                                        let rect_data = text_edit_output.response.rect;
                                        let row_num = self.selected_row as f32;
                                        ui.painter().rect_filled(
                                            egui::Rect::from_min_max(egui::Pos2::new(rect_data.left(), rect_data.top()+(row_num*14.0)+1.0 ), egui::Pos2::new(rect_data.right(), rect_data.top()+(row_num*14.0)+1.0+14.0)),
                                            0.0,
                                            Color32::from_rgba_premultiplied(100, 0, 0, 20),
                                            //faded_color(Color32::from_rgb(77, 163, 109)),
                                            //Color32::from_rgb(164, 192, 176),
                                        );

                                    });
                                //ui.label(line_size_to_string(count_lines(gcode_data)));
                                //ui.add(egui::Label::new(line_size_to_string(count_lines(gcode_data))));
                                //ui.painter().rect_filled(
                                //    ui.available_rect_before_wrap(),
                                //    0.0,
                                //    Color32::from_rgb(48, 82, 68),
                                //    //faded_color(Color32::from_rgb(77, 163, 109)),
                                //    //Color32::from_rgb(164, 192, 176),
                                //);
                            });

                            strip.cell(|ui| {
                                ui.spacing_mut().item_spacing = egui::vec2(5.0, 0.0);  
                                ui.style_mut().visuals.extreme_bg_color = Color32::from_rgb(164, 192, 176);
                                ui.style_mut().visuals.override_text_color = Some(Color32::from_rgb(33, 33, 33));
                                let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                                    let mut layout_job = egui::text::LayoutJob::default();

                                    let start_pos = (self.text_slider * 10) as usize;

                                    layout_job.append(
                                        &string[..],
                                        0.0,     
                                        egui::TextFormat {
                                            font_id: egui::FontId::new(12.0, egui::FontFamily::Monospace),
                                            color: Color32::BLACK,
                                            //background: Color32::RED,
                                            ..Default::default()
                                        },);
                                    
                                    
                                    ui.fonts(|f| f.layout_job(layout_job))
                                };

                                
                                let text_edit_output = egui::TextEdit::multiline(&mut self.gcode_data)
                                                                    //.min_size(ui.available_size())
                                                                    .desired_rows(10)
                                                                    .desired_width(f32::INFINITY)
                                                                    .layouter(&mut layouter)
                                                                    .show(ui);
                                //let output = ui.add_sized(ui.available_size(), egui::TextEdit::multiline(gcode_data));

                                let rect_data = text_edit_output.response.rect;
                                let row_num = self.selected_row as f32;
                                ui.painter().rect_filled(
                                    egui::Rect::from_min_max(egui::Pos2::new(rect_data.left(), rect_data.top()+(row_num*14.0)+1.0 ), egui::Pos2::new(rect_data.right(), rect_data.top()+(row_num*14.0)+1.0+14.0)),
                                    0.0,
                                    Color32::from_rgba_premultiplied(100, 0, 0, 20),
                                    //faded_color(Color32::from_rgb(77, 163, 109)),
                                    //Color32::from_rgb(164, 192, 176),
                                );


                                let cursor_pos = text_edit_output.cursor_range;


                                if let Some(cursor_pos) = cursor_pos {

                                    self.selected_row = cursor_pos.primary.rcursor.row;


                                    //if text_edit_output.response.clicked(){
                                    //    println!("click!");

                                    //    let text_edit_id = text_edit_output.response.id;
                                    //    if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
                                    //        println!("{}", cursor_pos.primary.ccursor.index );
                                    //        let ccursor = egui::text::CCursor::new(cursor_pos.primary.ccursor.index + (self.text_slider * 10) as usize);
                                    //        state.set_ccursor_range(Some(egui::text::CCursorRange::one(ccursor)));
                                    //        println!("{}", cursor_pos.primary.ccursor.index );
                                    //        state.store(ui.ctx(), text_edit_id);
                                    //        //ui.ctx().memory().request_focus(text_edit_id); // give focus back to the `TextEdit`.
                                    //    }
                                    //}

                                    //let rect_data = text_edit_output.response.rect;
                                    //let row_num = cursor_pos.primary.rcursor.row as f32;

                                    //let mut now_ccursor = cursor_pos.primary.ccursor;

                                    //text_edit_output.state.set_cursor_range(Some(cursor_pos));
                                    //text_edit_output.state.set_ccursor_range( Some(egui::widgets::text_edit::CCursorRange::one(egui::text::CCursor::new(cursor_pos.primary.ccursor.index + (self.text_slider * 10) as usize))));
                                    //text_edit_output.state.set_ccursor_range( Some(egui::widgets::text_edit::CCursorRange::one(egui::text::CCursor::new(0))));

                                    //ui.painter().rect_filled(
                                    //    egui::Rect::from_min_max(egui::Pos2::new(rect_data.left(), rect_data.top()+(row_num*14.0)+1.0 ), egui::Pos2::new(rect_data.right(), rect_data.top()+(row_num*14.0)+1.0+14.0)),
                                    //    0.0,
                                    //    Color32::from_rgba_premultiplied(100, 0, 0, 20),
                                    //    //faded_color(Color32::from_rgb(77, 163, 109)),
                                    //    //Color32::from_rgb(164, 192, 176),
                                    //);
                                }




                                //let hover_pos = ui.input().pointer.hover_pos();
                                //let hover_pos = ui.input(|i|i.pointer.hover_pos());
                                //if let Some(hover_pos) = hover_pos {
                                //    if text_edit_output.response.rect.contains(hover_pos) {
                                //        let hover_pos = hover_pos - text_edit_output.response.rect.left_top();
                                //        let hover_cursor = text_edit_output.galley.cursor_from_pos(hover_pos).pcursor;
                                //        println!("Line Num: {}", hover_cursor.paragraph);
                                //        if let Some(line) = gcode_data.lines().nth(hover_cursor.paragraph) {
                                //            egui::show_tooltip_at_pointer(ui.ctx(), egui::Id::new("hover tooltip"), |ui| {
                                //                ui.label(line);
                                //            });
                                //        }
                                //    }
                                //}
                            });

                            strip.cell(|ui| {
                                //let mut text_slider = 0;
                                ui.add(
                                    egui::Slider::new(&mut self.text_slider, (100 as i32)..=(0 as i32))
                                        .orientation(egui::SliderOrientation::Vertical)
                                        .text("i32 demo slider")
                                );
                                //println!("output: {}", self.text_slider);
                            })
                    });
                });
            });

        });
        */

        //let text_edit_output = egui::TextEdit::multiline(&mut self.gcode_data)
        //                                    .min_size(ui.available_size())
        //                                    .desired_rows(10)
        //                                    //.desired_width(f32::INFINITY)
        //                                    .show(ui);


        let text_style = egui::TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        let num_rows = 50000;
        egui::ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
            ui,
            row_height,
            num_rows,
            |ui, row_range| {
                for row in row_range {
                    //let mut text = format!("This is row {}/{}", row + 1, num_rows);
                    egui::TextEdit::singleline(&mut self.text_row[row]).show(ui);
                }
            },
        );

        //egui::Grid::new("gcode_data")
        //    .spacing(egui::Vec2::new(0.0, 0.0))
        //    .striped(true)
        //    //.min_col_width(60.0)
        //    //.max_col_width(200.0)
        //    .show(ui, |ui| {
        //        ui.spacing_mut().item_spacing = egui::Vec2::ZERO; 
        //        ui.add(egui::TextEdit::multiline(&mut "1\n2\n".to_string())
        //            .desired_width(f32::INFINITY)
        //        );

        //        ui.add(egui::TextEdit::multiline(gcode_data)
        //            .desired_width(f32::INFINITY)
        //        );
        //        ui.end_row();
        //    });

    }
}

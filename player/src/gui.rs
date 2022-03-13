// heavily draws from fltk-egui's base code

use crate::RunState;
use egui::color::Color32;
use egui::{Label, RichText};
use egui_backend::{
    egui,
    fltk::{enums::*, prelude::*, *},
    gl, DpiScaling,
};
use fltk_egui as egui_backend;
use std::rc::Rc;
use std::sync::mpsc;
use std::{cell::RefCell, time::Instant};

const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 200;

const DARK_LOADING_COLOR: Color32 = Color32::LIGHT_BLUE;
const LIGHT_LOADING_COLOR: Color32 = Color32::BLUE;

const DARK_RUNNING_COLOR: Color32 = Color32::LIGHT_GREEN;
const LIGHT_RUNNING_COLOR: Color32 = Color32::DARK_GREEN;

const DARK_SHUTDOWN_COLOR: Color32 = Color32::LIGHT_RED;
const LIGHT_SHUTDOWN_COLOR: Color32 = Color32::RED;

pub fn run_ui(underlying_state_receiver: mpsc::Receiver<RunState>, title: String) {
    let a = app::App::default();
    let mut win = window::GlWindow::new(100, 100, SCREEN_WIDTH as _, SCREEN_HEIGHT as _, None)
        .with_label(&title);
    win.set_mode(Mode::Opengl3);
    win.end();
    win.make_resizable(true);
    win.show();
    win.make_current();

    let (painter, egui_input_state) = egui_backend::with_fltk(&mut win, DpiScaling::Custom(1.5));
    let mut egui_ctx = egui::CtxRef::default();

    let state = Rc::from(RefCell::from(egui_input_state));
    let painter = Rc::from(RefCell::from(painter));

    // ensure it's only called once by taking it from the option
    let voyager_state = RefCell::new(RunState::Preparing);

    let (state_tx, state_receiver) = app::channel();

    std::thread::spawn(move || {
        for msg in underlying_state_receiver.iter() {
            state_tx.send(msg);
        }
    });

    win.handle({
        let state = state.clone();
        let painter = painter.clone();
        move |win, ev| match ev {
            enums::Event::Push
            | enums::Event::Released
            | enums::Event::KeyDown
            | enums::Event::KeyUp
            | enums::Event::MouseWheel
            | enums::Event::Resize
            | enums::Event::Move
            | enums::Event::Drag => {
                let mut state = state.borrow_mut();
                state.fuse_input(win, ev, &mut painter.borrow_mut());
                true
            }
            _ => false,
        }
    });

    let start_time = Instant::now();
    let mut shutting_down = false;

    while a.wait() {
        if let Some(new_state) = state_receiver.recv() {
            *voyager_state.borrow_mut() = new_state;
        }

        let mut state = state.borrow_mut();
        let mut painter = painter.borrow_mut();
        state.input.time = Some(start_time.elapsed().as_secs_f64());
        let (egui_output, shapes) = egui_ctx.run(state.input.take(), |ctx| {
            unsafe {
                // Clear the screen to black
                gl::ClearColor(0.6, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            egui::CentralPanel::default().show(&ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(&title);
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        egui::widgets::global_dark_light_mode_switch(ui);
                    });
                });

                let loading_color = if ui.visuals().dark_mode {
                    DARK_LOADING_COLOR
                } else {
                    LIGHT_LOADING_COLOR
                };
                let running_color = if ui.visuals().dark_mode {
                    DARK_RUNNING_COLOR
                } else {
                    LIGHT_RUNNING_COLOR
                };
                let shutdown_color = if ui.visuals().dark_mode {
                    DARK_SHUTDOWN_COLOR
                } else {
                    LIGHT_SHUTDOWN_COLOR
                };

                match *voyager_state.borrow() {
                    RunState::Preparing => ui.add(Label::new(
                        RichText::new("Preparing for launch")
                            .color(loading_color)
                            .strong(),
                    )),
                    RunState::ReadingEntities => ui.add(Label::new(
                        RichText::new("Reading primary database..")
                            .color(loading_color)
                            .strong(),
                    )),
                    RunState::ReadingSiteAssets => ui.add(Label::new(
                        RichText::new("Reading assets..")
                            .color(loading_color)
                            .strong(),
                    )),
                    RunState::ReadingFeed => ui.add(Label::new(
                        RichText::new("Reading The Feed..")
                            .color(loading_color)
                            .strong(),
                    )),
                    RunState::ShuttingDown => ui.add(Label::new(
                        RichText::new("Shutting down..")
                            .color(shutdown_color)
                            .strong(),
                    )),
                    RunState::Running(..) => ui.add(Label::new(
                        RichText::new("Running..").color(running_color).strong(),
                    )),
                };

                ui.separator();

                ui.horizontal(|ui| {
                    if let RunState::Running(ref rocket_config, _) = *voyager_state.borrow() {
                        if ui
                            .button("Open in browser")
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            let url = format!(
                                "{}://{}:{}",
                                if rocket_config.tls_enabled() {
                                    "https"
                                } else {
                                    "http"
                                },
                                rocket_config.address,
                                rocket_config.port,
                            );
                            if open::that(&url).is_err() {
                                println!("Couldn't open before in default browser");
                            }
                        }

                        if ui
                            .button("Shut down")
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            shutting_down = true;
                        }
                    }
                });
            });
        });

        state.fuse_output(&mut win, &egui_output);

        let meshes = egui_ctx.tessellate(shapes);

        painter.paint_jobs(None, meshes, &egui_ctx.font_image());

        win.swap_buffers();
        win.flush();

        if shutting_down {
            let mut voyager_state = voyager_state.borrow_mut();
            if let RunState::Running(_, ref mut handle) = *voyager_state {
                if let Some(h) = handle.take() {
                    h.notify();
                    *voyager_state = RunState::ShuttingDown;
                }
            }
        }

        if egui_output.needs_repaint {
            app::awake()
        }
    }
}

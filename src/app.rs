use eframe::egui::{self, Color32, Pos2, RichText, Vec2};

use crate::display::draw_time;
use crate::timer::{Timer, TimerState};

/// Main application state
pub struct TimerApp {
    timer: Timer,
    show_colon: bool,
    colon_timer: f32,
    overlay_mode: bool,
}

impl Default for TimerApp {
    fn default() -> Self {
        Self {
            timer: Timer::default(),
            show_colon: true,
            colon_timer: 0.0,
            overlay_mode: false,
        }
    }
}

impl TimerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for TimerApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // Transparent background when in overlay mode
        if self.overlay_mode {
            [0.0, 0.0, 0.0, 0.0]
        } else {
            [0.86, 0.86, 0.84, 1.0] // Light warm gray
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request continuous repaint for smooth animation
        if self.timer.is_running() || self.overlay_mode {
            ctx.request_repaint();
        }

        // Update timer
        self.timer.tick();

        // Blink colon every 0.5 seconds when running
        if self.timer.is_running() {
            self.colon_timer += ctx.input(|i| i.unstable_dt);
            if self.colon_timer >= 0.5 {
                self.colon_timer = 0.0;
                self.show_colon = !self.show_colon;
            }
        } else {
            self.show_colon = true;
        }



        // Colors
        let bg_color = if self.overlay_mode {
            Color32::from_rgba_unmultiplied(30, 30, 30, 180) // Dark, semi-transparent
        } else {
            Color32::from_rgb(220, 220, 215) // Light warm gray
        };
        let digit_color = if self.overlay_mode {
            Color32::from_rgba_unmultiplied(255, 255, 255, 240) // White, slightly transparent
        } else {
            Color32::from_rgb(80, 80, 75) // Dark gray
        };
        let button_color = Color32::from_rgb(100, 100, 95);
        let button_hover = Color32::from_rgb(70, 70, 65);

        // Set the background with rounded corners for overlay
        let frame = if self.overlay_mode {
            egui::Frame::default()
                .fill(bg_color)
                .rounding(egui::Rounding::same(12.0))
                .inner_margin(egui::Margin::same(8.0))
        } else {
            egui::Frame::default().fill(bg_color)
        };

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            let available_size = ui.available_size();
            
            // In overlay mode, position time at center of small window
            let center = if self.overlay_mode {
                Pos2::new(available_size.x / 2.0, available_size.y / 2.0)
            } else {
                Pos2::new(available_size.x / 2.0, available_size.y / 2.0 - 30.0)
            };

            // Draw the time display
            draw_time(
                ui,
                self.timer.minutes(),
                self.timer.seconds(),
                center,
                digit_color,
                self.show_colon,
                self.overlay_mode,
            );

            if self.overlay_mode {
                let rect = ui.max_rect();
                
                // Background interaction - allocated first so it's behind the button
                let bg_response = ui.allocate_rect(rect, egui::Sense::click_and_drag());
                
                if bg_response.drag_started() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                
                // Exit button - Positioned manually at top-right
                let exit_size = Vec2::new(24.0, 24.0);
                let exit_rect = egui::Rect::from_min_size(
                    rect.right_top() - Vec2::new(exit_size.x, 0.0), 
                    exit_size
                );
                
                let exit_btn = egui::Button::new(
                    RichText::new("✕").size(14.0).color(Color32::from_rgba_unmultiplied(255, 255, 255, 150))
                )
                .fill(Color32::TRANSPARENT);

                let exit_response = ui.put(exit_rect, exit_btn);
                
                // Check exit click FIRST (takes priority)
                if exit_response.on_hover_text("Exit focus mode").clicked() {
                    self.overlay_mode = false;
                    // Request window resize back to normal
                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(400.0, 400.0)));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(true));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Transparent(false));
                } else if bg_response.clicked() {
                    // Only toggle if exit wasn't clicked
                    self.timer.toggle();
                }
            } else {
                // Full UI mode
                
                // Control buttons at the bottom
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(40.0);

                    // Time adjustment controls (only when stopped)
                    if self.timer.state == TimerState::Stopped {
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            ui.add_space((available_size.x - 300.0) / 2.0);
                            
                            // Minutes controls
                            ui.vertical(|ui| {
                                ui.label(RichText::new("Minutes").color(digit_color).size(12.0));
                                ui.horizontal(|ui| {
                                    if ui.button(RichText::new("−").size(18.0).color(button_color)).clicked() {
                                        self.timer.add_minutes(-1);
                                    }
                                    ui.label(RichText::new(format!("{:02}", self.timer.minutes())).color(digit_color).size(16.0));
                                    if ui.button(RichText::new("+").size(18.0).color(button_color)).clicked() {
                                        self.timer.add_minutes(1);
                                    }
                                });
                            });

                            ui.add_space(40.0);

                            // Seconds controls
                            ui.vertical(|ui| {
                                ui.label(RichText::new("Seconds").color(digit_color).size(12.0));
                                ui.horizontal(|ui| {
                                    if ui.button(RichText::new("−").size(18.0).color(button_color)).clicked() {
                                        self.timer.add_seconds(-10);
                                    }
                                    ui.label(RichText::new(format!("{:02}", self.timer.seconds())).color(digit_color).size(16.0));
                                    if ui.button(RichText::new("+").size(18.0).color(button_color)).clicked() {
                                        self.timer.add_seconds(10);
                                    }
                                });
                            });
                        });
                    }

                    ui.add_space(15.0);

                    // Main control buttons
                    ui.horizontal(|ui| {
                        ui.add_space((available_size.x - 280.0) / 2.0);

                        let button_size = Vec2::new(80.0, 35.0);
                        
                        // Start/Pause button
                        let start_text = match self.timer.state {
                            TimerState::Running => "Pause",
                            TimerState::Paused => "Resume",
                            TimerState::Stopped => "Start",
                        };
                        
                        let start_btn = egui::Button::new(
                            RichText::new(start_text).size(16.0).color(Color32::WHITE)
                        )
                        .fill(button_color)
                        .min_size(button_size);
                        
                        if ui.add(start_btn).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                            self.timer.toggle();
                        }

                        ui.add_space(10.0);

                        // Reset button
                        let reset_btn = egui::Button::new(
                            RichText::new("Reset").size(16.0).color(Color32::WHITE)
                        )
                        .fill(button_hover)
                        .min_size(button_size);
                        
                        if ui.add(reset_btn).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                            self.timer.reset();
                        }

                        ui.add_space(10.0);

                        // Overlay mode button
                        let overlay_btn = egui::Button::new(
                            RichText::new("📌 Focus").size(14.0).color(Color32::WHITE)
                        )
                        .fill(Color32::from_rgb(70, 130, 180))
                        .min_size(button_size);
                        
                        if ui.add(overlay_btn).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Always on top - click timer to pause/resume").clicked() {
                            self.overlay_mode = true;
                            // Make window small, always on top, no decorations, transparent
                            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(160.0, 70.0)));
                            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
                            ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(false));
                            ctx.send_viewport_cmd(egui::ViewportCommand::Transparent(true));
                        }
                    });
                });

                // Show "Time's up!" message when finished
                if self.timer.is_finished() {
                    egui::Window::new("Timer Complete")
                        .collapsible(false)
                        .resizable(false)
                        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                        .show(ctx, |ui| {
                            ui.label(RichText::new("⏰ Time's up!").size(24.0).color(digit_color));
                            if ui.button("OK").clicked() {
                                self.timer.reset();
                            }
                        });
                }
            }
        });
    }
}

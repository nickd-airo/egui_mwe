use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {

            let app = MyApp {
                wgpu_render_state: cc.wgpu_render_state.clone().unwrap(),
            };

            Ok(Box::new(app))
        }),
    )
}

struct MyApp {
    wgpu_render_state: eframe::egui_wgpu::RenderState,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            let buffer_memory = self.wgpu_render_state.device.get_internal_counters().hal.buffer_memory.read();
            println!("Buffer Memory: {buffer_memory} bytes");
        });
        ctx.request_repaint();
    }
}

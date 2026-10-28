use crate::diagnostics::Diagnostics;
use crate::input::{self, SettingAction};
use crate::renderer::Renderer;
use crate::settings::ParticleSettings;
use crate::sprite::Sprite;
use crate::state::GameState;
use crate::time::Time;
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::{
 application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
 keyboard::{KeyCode, PhysicalKey}, window::{Fullscreen, Window, WindowId},
};
pub struct App {
 window: Option<Arc<Window>>, renderer: Option<Renderer>, game_state: GameState,
 time: Time, diagnostics: Diagnostics, resizing: bool, last_resize: Option<Instant>,
 settings_panel_open: bool, selected_setting: usize, settings_overlay: crate::text::Text,
}
impl App {
 pub fn new()->Self { Self { window:None, renderer:None, game_state:GameState::new(), time:Time::new(), diagnostics:Diagnostics::new(), resizing:false, last_resize:None, settings_panel_open:false, selected_setting:0, settings_overlay: {
  let mut text = crate::text::Text::new("F1: Open particle settings", 21.0);
  text.set_position(-0.62, 0.60);
  text.set_color(0.72, 0.94, 1.0, 1.0);
  text.set_line_spacing(1.35);
  text.set_max_width(Some(600.0));
  text
 } } }
 fn toggle_fullscreen(&mut self) { let Some(window)=&self.window else{return}; self.resizing=true; self.last_resize=Some(Instant::now()); let fullscreen=if window.fullscreen().is_some(){None}else{Some(Fullscreen::Borderless(window.current_monitor()))}; window.set_fullscreen(fullscreen); }
 fn refresh_settings_overlay(&mut self) {
  let s = &self.game_state.particle_settings;
  let labels = [
   format!("Particle count: {}", s.particle_count),
   format!("Flow speed: {:.2}", s.flow_speed),
   format!("Steering: {:.1}", s.flow_steering),
   format!("Field scale: {:.2}", s.field_scale),
   format!("Field grid: {} x {}", s.field_resolution, s.field_resolution),
   format!("Particle size: {:.4}", s.particle_size),
   format!("Field update: {:.2} s", s.field_update_interval),
  ];

  let mut content = if self.settings_panel_open {
   String::from(
    "+--------------------------------------\\n|  PARTICLE SETTINGS                   |\\n|  TAB select  LEFT/RIGHT adjust       |\\n|                                      |\\n"
   )
  } else {
   String::from("F1: Open particle settings\\n")
  };

  if self.settings_panel_open {
   for (i, label) in labels.iter().enumerate() {
    let marker = if i == self.selected_setting { ">" } else { " " };
    content.push_str(&format!("| {} {}\\n", marker, label));
   }
   content.push_str("|                                      |\\n|  R Reset   F Fullscreen   F1 Close   |\\n+--------------------------------------+");
  }

  self.settings_overlay.set_content(&content);
  self.settings_overlay.set_position(-0.62, 0.60);
  self.settings_overlay.set_color(0.72, 0.94, 1.0, 1.0);
  self.settings_overlay.set_line_spacing(1.35);
  self.settings_overlay.set_max_width(Some(600.0));

  if let Some(renderer) = &mut self.renderer {
   renderer.update_text_object(4, &self.settings_overlay);
  }
 }
 fn handle_settings_action(&mut self, action:SettingAction) {
  match action { SettingAction::TogglePanel=>self.settings_panel_open=!self.settings_panel_open,
   SettingAction::Reset=>{self.game_state.particle_settings=ParticleSettings::default();self.game_state.apply_particle_settings();},
   SettingAction::Decrease|SettingAction::Increase=>{input::adjust(&mut self.game_state.particle_settings,self.selected_setting,action==SettingAction::Increase);self.game_state.apply_particle_settings();}
  }
  if let Some(renderer)=&mut self.renderer { renderer.set_particle_size(self.game_state.particle_settings.particle_size); }
  self.refresh_settings_overlay();
 }
 fn update(&mut self){ self.time.update(); self.diagnostics.update(&self.time); self.game_state.update(self.time.delta_time()); }
 fn render(&mut self){ self.update(); let Some(renderer)=&mut self.renderer else{return}; renderer.render(&self.game_state,&self.diagnostics.fps_text,&self.diagnostics.frame_time_ms_text); }
 fn initialize_scene(&mut self, renderer:&mut Renderer) {
  let sprite=Sprite::from_file(&renderer.device,&renderer.queue,"assets/textures/Ball.png",[0.01,0.01]);
  renderer.initialize_particle_resources(&sprite); renderer.set_particle_size(self.game_state.particle_settings.particle_size);
  for entity in &self.game_state.entities { renderer.create_render_object(entity); }
  renderer.create_text_object(0,self.game_state.text.clone()); renderer.create_text_object(1,self.game_state.score_text.clone());
  renderer.create_text_object(2,self.diagnostics.fps_text.clone()); renderer.create_text_object(3,self.diagnostics.frame_time_ms_text.clone());
  renderer.create_text_object(4,self.settings_overlay.clone());
 }
}
impl ApplicationHandler for App {
 fn resumed(&mut self,event_loop:&ActiveEventLoop) {
  let window=Arc::new(event_loop.create_window(Window::default_attributes().with_title("East Engine")).unwrap());
  let mut renderer=Renderer::new(window.clone()); self.initialize_scene(&mut renderer); self.window=Some(window);self.renderer=Some(renderer);
  if let Some(window)=&self.window {window.request_redraw();}
 }
 fn window_event(&mut self,event_loop:&ActiveEventLoop,_window_id:WindowId,event:WindowEvent){
  match event {
   WindowEvent::CloseRequested=>event_loop.exit(),
   WindowEvent::Resized(size)=>{if size.width==0||size.height==0{self.resizing=true;return;}self.resizing=true;self.last_resize=Some(Instant::now());if let Some(renderer)=&mut self.renderer{renderer.set_size(size.width,size.height);}}
   WindowEvent::KeyboardInput{event,..}=>{
    let pressed=event.state==winit::event::ElementState::Pressed;
    if let PhysicalKey::Code(key)=event.physical_key {
     if key==KeyCode::Escape&&pressed{event_loop.exit();return;}
     if pressed&&!event.repeat {
      if key==KeyCode::KeyF { self.toggle_fullscreen(); }
      else if key==KeyCode::Tab&&self.settings_panel_open { self.selected_setting=(self.selected_setting+1)%7; self.refresh_settings_overlay(); }
      else if key==KeyCode::ArrowLeft||key==KeyCode::ArrowDown||key==KeyCode::ArrowRight||key==KeyCode::ArrowUp||key==KeyCode::KeyR {
       if self.settings_panel_open { if let Some(action)=input::action_for_key(key){self.handle_settings_action(action);} }
      } else if key==KeyCode::F1 { self.handle_settings_action(SettingAction::TogglePanel); }
     }
    }
   }
   WindowEvent::RedrawRequested=>{if self.resizing{return;}self.render();if let Some(window)=&self.window{window.request_redraw();}}
   _=>{}
  }
 }
 fn about_to_wait(&mut self,_event_loop:&ActiveEventLoop){
  if self.resizing&&let Some(last_resize)=self.last_resize&&last_resize.elapsed()>=Duration::from_millis(100){
   self.resizing=false;self.last_resize=None;if let Some(renderer)=&mut self.renderer{renderer.apply_pending_resize();}self.time.reset();if let Some(window)=&self.window{window.request_redraw();}
  }
 }
}

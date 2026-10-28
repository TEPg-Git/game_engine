#[derive(Default)]
pub struct KeyboardState;

use winit::keyboard::KeyCode;
use crate::settings::ParticleSettings;
#[derive(Clone,Copy,Debug,PartialEq,Eq)] pub enum SettingAction { TogglePanel, Decrease, Increase, Reset }
pub fn action_for_key(key:KeyCode)->Option<SettingAction>{match key{KeyCode::F1=>Some(SettingAction::TogglePanel),KeyCode::ArrowLeft|KeyCode::ArrowDown=>Some(SettingAction::Decrease),KeyCode::ArrowRight|KeyCode::ArrowUp=>Some(SettingAction::Increase),KeyCode::KeyR=>Some(SettingAction::Reset),_=>None}}
pub fn adjust(s:&mut ParticleSettings,selected:usize,increase:bool){let sign=if increase{1.0}else{-1.0};match selected{0=>{let step=(s.particle_count/10).max(1_000);s.particle_count=if increase{s.particle_count.saturating_add(step).min(ParticleSettings::MAX_PARTICLES)}else{s.particle_count.saturating_sub(step).max(ParticleSettings::MIN_PARTICLES)};},1=>s.flow_speed+=sign*0.05,2=>s.flow_steering+=sign*0.5,3=>s.field_scale+=sign*0.25,4=>s.field_resolution=if increase{(s.field_resolution+8).min(256)}else{s.field_resolution.saturating_sub(8).max(8)},5=>s.particle_size+=sign*0.00025,6=>s.field_update_interval-=sign*0.05,_=>{}}s.clamp();}

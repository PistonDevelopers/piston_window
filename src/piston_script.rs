//! # Piston script

use crate::*;
use std::sync::Arc;
use std::collections::HashMap;
use current::CurrentGuard;
use dyon::{error, load, Module, Dfn, Runtime, Type};
use dyon_interactive::{FontNames, ImageNames};
use image::RgbaImage;
use wgpu_graphics::{
    Filter,
    GlyphCache,
    Texture,
    TextureContext,
    TextureSettings,
};
use kira::{AudioManager, AudioManagerSettings};
use kira::sound::static_sound::StaticSoundData;
use kira::sound::streaming::StreamingSoundHandle;
use kira::sound::FromFileError;
use kira::track::TrackHandle;

type Sounds = HashMap<Arc<String>, (
    Arc<String>,
    Option<StaticSoundData>,
)>;
type Music = HashMap<Arc<String>, (
    Arc<String>,
    Option<StreamingSoundHandle<FromFileError>>,
)>;

/// Run Dyon script file with Dyon-Interactive API for the Piston game engine.
///
/// Sets up a Piston window automatically.
///
/// You can pass in a Dyon module for extra external functions.
///
/// The `f` parameter is usually `|f| f()` when you do not need to set up
/// customized context for extra external functions.
pub fn run<F>(
    file: &str,
    module: Module,
    f: F,
) -> Result<(), ()>
    where F: FnOnce(&mut dyn FnMut() -> Result<(), ()>) -> Result<(), ()>
{
    let mut window: PistonWindow =
        WindowSettings::new("piston", [512; 2])
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();
    let dyon_module = match load_module(&file, module) {
        None => return Err(()),
        Some(m) => Arc::new(m)
    };

    let mut dyon_runtime = Runtime::new();
    let font_texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let mut glyphs = vec![
        window.load_builtin_font(BuiltInFont::FiraSansRegular, font_texture_settings).unwrap(),
        window.load_builtin_font(BuiltInFont::HackRegular, font_texture_settings).unwrap(),
    ];
    let mut font_names = FontNames(vec![
        Arc::new("FiraSans-Regular".to_owned()),
        Arc::new("Hack-Regular".to_owned()),
    ]);
    let mut images = vec![];
    let mut image_names = ImageNames(vec![]);
    let mut texture_context = window.create_texture_context();
    let mut textures = vec![];
    let mut gl = G2d::new(window.device.clone(), &window.surface_config);
    let mut events = Events::new(EventSettings::new());
    let mut audio_manager = AudioManager::new(AudioManagerSettings::default()).unwrap();
    let mut music_track = audio_manager.add_sub_track(Default::default()).unwrap();
    let mut sounds: Sounds = HashMap::new();
    let mut music: Music = HashMap::new();

    let mut e: Option<Event> = None;
    let window_guard = CurrentGuard::new(&mut window);
    let event_guard: CurrentGuard<Option<Event>> = CurrentGuard::new(&mut e);
    let glyphs_guard: CurrentGuard<Vec<GlyphCache>> = CurrentGuard::new(&mut glyphs);
    let font_names_guard: CurrentGuard<FontNames> = CurrentGuard::new(&mut font_names);
    let images_guard: CurrentGuard<Vec<RgbaImage>> = CurrentGuard::new(&mut images);
    let image_names_guard: CurrentGuard<ImageNames> = CurrentGuard::new(&mut image_names);
    let texture_context_guard: CurrentGuard<TextureContext> =
        CurrentGuard::new(&mut texture_context);
    let textures_guard: CurrentGuard<Vec<Texture>> = CurrentGuard::new(&mut textures);
    let gl_guard: CurrentGuard<G2d> = CurrentGuard::new(&mut gl);
    let events_guard: CurrentGuard<Events> = CurrentGuard::new(&mut events);
    let audio_manager_guard: CurrentGuard<AudioManager> = CurrentGuard::new(&mut audio_manager);
    let music_track_guard: CurrentGuard<TrackHandle> = CurrentGuard::new(&mut music_track);
    let sounds_guard: CurrentGuard<Sounds> = CurrentGuard::new(&mut sounds);
    let music_guard: CurrentGuard<Music> = CurrentGuard::new(&mut music);

    f(&mut || {
        if error(dyon_runtime.run(&dyon_module)) {
            return Err(());
        } else {Ok(())}
    })?;

    drop(music_guard);
    drop(sounds_guard);
    drop(music_track_guard);
    drop(audio_manager_guard);
    drop(events_guard);
    drop(gl_guard);
    drop(textures_guard);
    drop(texture_context_guard);
    drop(image_names_guard);
    drop(images_guard);
    drop(font_names_guard);
    drop(glyphs_guard);
    drop(event_guard);
    drop(window_guard);

    Ok(())
}

fn load_module(file: &str, mut module: Module) -> Option<Module> {
    use dyon_functions::*;
    use dyon_interactive::add_functions;

    add_functions::<PistonWindow, TextureContext, GlyphCache>(&mut module);
    module.add(Arc::new("draw".into()), draw, Dfn::nl(vec![Type::array()], Type::Void));
    module.add(Arc::new("next_event".into()),
        next_event, Dfn::nl(vec![], Type::Bool));
    module.add(Arc::new("bind_sound__name_file".into()),
        bind_sound__name_file, Dfn::nl(vec![Type::Str; 2], Type::Void));
    module.add(Arc::new("bind_music__name_file".into()),
        bind_music__name_file, Dfn::nl(vec![Type::Str; 2], Type::Void));
    module.add(Arc::new("play_sound__name_repeat_volume".into()),
        play_sound__name_repeat_volume, Dfn::nl(vec![Type::Str, Type::F64, Type::F64], Type::Void));
    module.add(Arc::new("play_sound_forever__name_volume".into()),
        play_sound_forever__name_volume, Dfn::nl(vec![Type::Str, Type::F64], Type::Void));
    module.add(Arc::new("play_music__name_repeat".into()),
        play_music__name_repeat, Dfn::nl(vec![Type::Str, Type::F64], Type::Void));
    module.add(Arc::new("play_music_forever__name".into()),
        play_music_forever__name, Dfn::nl(vec![Type::Str], Type::Void));
    module.add(Arc::new("set_music_volume".into()),
        set_music_volume, Dfn::nl(vec![Type::F64], Type::Void));
    module.add(Arc::new("create_texture".into()),
        create_texture, Dfn {
            lts: vec![dyon::Lt::Default],
            tys: vec![Type::Any],
            ret: Type::Any,
            ext: vec![(vec![], vec![Type::F64, Type::F64], Type::F64)],
            lazy: dyon::LAZY_NO,
        });
    module.add(Arc::new("update__texture_image".into()),
        update__texture_image, Dfn::nl(vec![Type::F64, Type::F64], Type::Void)
    );
    module.add(Arc::new("load_font".into()),
        load_font, Dfn::nl(vec![Type::Str], Type::Result(Box::new(Type::F64)))
    );
    module.add(Arc::new("load_font_obj".into()),
        load_font_obj, Dfn::nl(vec![Type::Str], Type::Result(Box::new(Type::Any)))
    );

    if error(dyon::load_str(
        "graphics2d.dyon",
        Arc::new(include_str!("../src/graphics2d.dyon").into()),
        &mut module
    )) {
        return None;
    }
    module.make_transitive();

    if error(load(file, &mut module)) {
        None
    } else {
        Some(module)
    }
}

mod dyon_functions {
    use crate::*;
    use dyon::{dyon_fn, dyon_macro_items, dyon_fn_pop};
    use piston::input::{Event, RenderEvent};
    use wgpu_graphics::{GlyphCache, Texture, TextureContext, TextureSettings, UpdateTexture};
    use dyon::{Runtime, Variable};
    use dyon_interactive::{draw_2d, FontNames, NO_EVENT};
    use current::Current;
    use std::sync::Arc;
    use image::RgbaImage;

    pub fn load_font(rt: &mut Runtime) -> Result<Variable, String> {
        use dyon::embed::PushVariable;
        use wgpu_graphics::{Filter, TextureSettings};
        use graphics::glyph_cache::rusttype::GlyphCache;

        let texture_context = unsafe {&*Current::<TextureContext>::new()};
        let glyphs = unsafe { &mut *Current::<Vec<GlyphCache<'static, TextureContext, Texture>>>::new() };
        let font_names = unsafe { &mut *Current::<FontNames>::new() };
        let file: Arc<String> = rt.pop()?;
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        Ok(match GlyphCache::<'static, TextureContext, Texture>::new(
            &**file, texture_context.clone(), texture_settings)
        {
            Ok(x) => {
                let id = glyphs.len();
                glyphs.push(x);
                font_names.0.push(file.clone());
                Ok::<usize, Arc<String>>(id).push_var()
            }
            Err(err) => {
                Err::<usize, Arc<String>>(Arc::new(format!("{}", err))).push_var()
            }
        })
    }

    pub fn load_font_obj(rt: &mut Runtime) -> Result<Variable, String> {
        use dyon::embed::{to_rust_object, PushVariable};
        use dyon::RustObject;
        use wgpu_graphics::{Filter, TextureSettings};
        use graphics::glyph_cache::rusttype::GlyphCache;

        let file: Arc<String> = rt.pop()?;
        let texture_context = unsafe { &*Current::<TextureContext>::new() };
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        Ok(match GlyphCache::<'static, TextureContext, Texture>::new(
            &**file, texture_context.clone(), texture_settings
        ) {
            Ok(x) => Ok::<RustObject, Arc<String>>(to_rust_object(x)).push_var(),
            Err(err) => Err::<RustObject, Arc<String>>(Arc::new(format!("{}", err))).push_var(),
        })
    }

    pub fn draw(rt: &mut Runtime) -> Result<(), String> {
        let e = unsafe { &*Current::<Option<Event>>::new() };
        if let &Some(ref e) = e {
            if e.render_args().is_some() {
                let glyphs = unsafe { &mut *Current::<Vec<GlyphCache>>::new() };
                let textures = unsafe { &mut *Current::<Vec<Texture>>::new() };
                let window = unsafe { &mut *Current::<PistonWindow>::new() };

                if let Some(x) = window.draw_2d(e, |c, g, _| draw_2d(rt, glyphs, textures, c, g)) {x}
                else {Err("Drawing failed".to_string())}
            } else {
                Ok(())
            }
        } else {
            Err(NO_EVENT.into())
        }
    }

    pub fn create_texture(rt: &mut Runtime) -> Result<Variable, String> {
        use dyon::embed::{to_rust_object, PushVariable};

        let image: Variable = rt.pop()?;
        match image {
            Variable::F64(id, _) => {
                let texture_context = unsafe {&mut *Current::<TextureContext>::new()};
                let images = unsafe { &*Current::<Vec<RgbaImage>>::new() };
                let textures = unsafe { &mut *Current::<Vec<Texture>>::new() };
                let image: &RgbaImage = if let Some(x) = images.get(id as usize) {x}
                            else {return Err("Image id is out of bounds".into())};

                let new_id = textures.len();
                textures.push(Texture::from_image(texture_context, image, &TextureSettings::new())
                    .map_err(|_| "Could not create texture".to_string())?);
                Ok(new_id.push_var())
            }
            Variable::RustObject(obj) => {
                let texture_context = unsafe {&mut *Current::<TextureContext>::new()};
                let mut guard = obj.lock().map_err(|_| "Could not obtain lock on Mutex".to_string())?;
                let image: &mut RgbaImage = guard.downcast_mut()
                    .ok_or_else(|| "Expected RgbaImage".to_string())?;

                Ok(Variable::RustObject(
                    to_rust_object(Texture::from_image(texture_context, image, &TextureSettings::new()))))
            }
            _ => Err("Expected `f64` (image id) or `any` (rust object)".to_string()),
        }
    }

    #[allow(non_snake_case)]
    pub fn update__texture_image(rt: &mut Runtime) -> Result<(), String> {
        use texture::Format;
        let image: Variable = rt.pop()?;
        let texture: Variable = rt.pop()?;
        match image {
            Variable::F64(id, _) => {
                let texture_context = unsafe {&mut *Current::<TextureContext>::new()};
                let images = unsafe { &*Current::<Vec<RgbaImage>>::new() };
                let image: &RgbaImage = if let Some(x) = images.get(id as usize) {x}
                            else {return Err("Image id is out of bounds".into())};
                match texture {
                    Variable::F64(id, _) => {
                        let textures = unsafe { &mut *Current::<Vec<Texture>>::new() };
                        let texture: &mut Texture = if let Some(x) = textures.get_mut(id as usize) {x}
                                    else {return Err("Texture id is out of bounds".into())};
                        let (w, h) = image.dimensions();
                        texture.update(texture_context, Format::Rgba8, &image, [0; 2], [w, h])
                            .map_err(|_| "Could not update texture".to_string())
                    }
                    Variable::RustObject(obj) => {
                        let mut guard = obj.lock()
                            .map_err(|_| "Could not obtain lock on Mutex".to_string())?;
                        let texture: &mut Texture = guard.downcast_mut()
                            .ok_or_else(|| "Expected Texture".to_string())?;
                        let (w, h) = image.dimensions();
                        texture.update(texture_context, Format::Rgba8, &image, [0; 2], [w, h])
                            .map_err(|_| "Could not update texture".to_string())
                    }
                    _ => Err("Expected `f64` (texture id) or `any` (rust object)".to_string()),
                }
            }
            Variable::RustObject(obj) => {
                let texture_context = unsafe {&mut *Current::<TextureContext>::new()};
                let mut guard = obj.lock().map_err(|_| "Could not obtain lock on Mutex".to_string())?;
                let image: &mut RgbaImage = guard.downcast_mut()
                    .ok_or_else(|| "Expected RgbaImage".to_string())?;
                match texture {
                    Variable::F64(id, _) => {
                        let textures = unsafe { &mut *Current::<Vec<Texture>>::new() };
                        let texture: &mut Texture = if let Some(x) = textures.get_mut(id as usize) {x}
                                    else {return Err("Texture id is out of bounds".into())};
                        let (w, h) = image.dimensions();
                        texture.update(texture_context, Format::Rgba8, &image, [0; 2], [w, h])
                            .map_err(|_| "Could not update texture".to_string())
                    }
                    Variable::RustObject(obj) => {
                        let mut guard = obj.lock()
                            .map_err(|_| "Could not obtain lock on Mutex".to_string())?;
                        let texture: &mut Texture = guard.downcast_mut()
                            .ok_or_else(|| "Expected Texture".to_string())?;
                        let (w, h) = image.dimensions();
                        texture.update(texture_context, Format::Rgba8, &image, [0; 2], [w, h])
                            .map_err(|_| "Could not update texture".to_string())
                    }
                    _ => Err("Expected `f64` (texture id) or `any` (rust object)".to_string()),
                }
            }
            _ => Err("Expected `f64` (image id) or `any` (rust object)".to_string()),
        }
    }

    dyon_fn!{fn next_event() -> bool {
        let window = unsafe { &mut *Current::<PistonWindow>::new() };
        let events = unsafe { &mut *Current::<Events>::new() };
        let e = unsafe { &mut *Current::<Option<Event>>::new() };
        if let Some(new_e) = events.next(window) {
            *e = Some(new_e);
            true
        } else {
            *e = None;
            false
        }
    }}

    dyon_fn!{fn bind_sound__name_file(name: Arc<String>, file: Arc<String>) {
        use crate::piston_script::Sounds;
        let sounds = unsafe { &mut *Current::<Sounds>::new() };
        sounds.insert(name, (file, None));
    }}

    dyon_fn!{fn bind_music__name_file(name: Arc<String>, file: Arc<String>) {
        use crate::piston_script::Music;
        let music = unsafe { &mut *Current::<Music>::new() };
        music.insert(name, (file, None));
    }}

    dyon_fn!{fn play_sound__name_repeat_volume(name: Arc<String>, repeat: f64, volume: f64) {
        use crate::piston_script::Sounds;
        use crate::piston_script::AudioManager;
        use kira::sound::static_sound::StaticSoundData;
        use kira::StartTime;
        use std::path::Path;
        use std::time::Duration;

        let f = |x| amplitude_to_decibels(x);

        let audio_manager = unsafe { &mut *Current::<AudioManager>::new() };
        let sounds = unsafe { &mut *Current::<Sounds>::new() };
        let sound_track = audio_manager.main_track();
        if let Some((file, sound_data)) = sounds.get_mut(&name) {
            if repeat == -1.0 {
                if let Some(sound_data) = sound_data {
                    let _ = sound_track.play(sound_data.clone().loop_region(..).volume(f(volume as f32)));
                } else {
                    let data = StaticSoundData::from_file(Path::new(&**file)).unwrap();
                    let _ = sound_track.play(data.clone().loop_region(..).volume(f(volume as f32)));
                    *sound_data = Some(data);
                }
            } else if repeat != 0.0 {
                if let Some(sound_data) = sound_data {
                    let _ = sound_track.play(sound_data.clone().volume(f(volume as f32)));
                } else {
                    let data = StaticSoundData::from_file(Path::new(&**file)).unwrap();
                    let _ = sound_track.play(data.clone().volume(f(volume as f32)));
                    *sound_data = Some(data);
                }
                let duration = if let Some(sound_data) = sound_data {
                    sound_data.duration()
                } else {Duration::new(0, 0)};

                let mut start = duration;
                if let Some(data) = sound_data {
                    for _ in 1..repeat as u32 {
                        let mut handle = sound_track.play(data.clone().volume(f(volume as f32))).unwrap();
                        handle.pause(Default::default());
                        handle.resume_at(StartTime::Delayed(start), Default::default());

                        start += duration;
                    }
                }
            }
        }
    }}

    fn amplitude_to_decibels(amplitude: f32) -> kira::Decibels {
        if amplitude <= 0.0 {return kira::Decibels::SILENCE};
        (20.0 * amplitude.log10()).into()
    }

    dyon_fn!{fn play_sound_forever__name_volume(name: Arc<String>, volume: f64) {
        use crate::piston_script::Sounds;
        use crate::piston_script::AudioManager;
        use kira::sound::static_sound::StaticSoundData;
        use std::path::Path;

        let f = |x| amplitude_to_decibels(x);

        let audio_manager = unsafe { &mut *Current::<AudioManager>::new() };
        let sounds = unsafe { &mut *Current::<Sounds>::new() };
        let sound_track = audio_manager.main_track();
        if let Some((file, sound_data)) = sounds.get_mut(&name) {
            if let Some(sound_data) = sound_data {
                let _ = sound_track.play(sound_data.loop_region(..).volume(f(volume as f32)));
            } else {
                let data = StaticSoundData::from_file(Path::new(&**file)).unwrap();
                let _ = sound_track.play(data.clone().loop_region(..).volume(f(volume as f32)));
                *sound_data = Some(data);
            }
        }
    }}

    dyon_fn!{fn play_music__name_repeat(name: Arc<String>, repeat: f64) {
        use crate::piston_script::Music;
        use crate::piston_script::TrackHandle;
        use kira::sound::streaming::StreamingSoundData;
        use kira::StartTime;
        use std::path::Path;

        let music = unsafe { &mut *Current::<Music>::new() };
        let music_track = unsafe { &mut *Current::<TrackHandle>::new() };
        if let Some((file, sound_handle)) = music.get_mut(&name) {
            if repeat == -1.0 {
                if let Some(sound_handle) = sound_handle {
                    sound_handle.set_loop_region(..);
                    sound_handle.resume(Default::default());
                } else {
                    let sound_data = StreamingSoundData::from_file(Path::new(&**file)).unwrap();
                    let sound = music_track.play(
                        sound_data.loop_region(..)
                    ).unwrap();
                    *sound_handle = Some(sound);
                }
            } else if repeat != 0.0 {
                let sound_data = StreamingSoundData::from_file(Path::new(&**file)).unwrap();
                let duration = sound_data.duration();
                let sound = music_track.play(
                    sound_data
                ).unwrap();
                *sound_handle = Some(sound);

                let mut start = duration;
                for _ in 1..repeat as u32 {
                    let sound_data = StreamingSoundData::from_file(Path::new(&**file)).unwrap();
                    let mut sound = music_track.play(sound_data).unwrap();
                    sound.pause(Default::default());
                    sound.resume_at(StartTime::Delayed(start), Default::default());

                    start += duration;
                }
            }
        }
    }}

    dyon_fn!{fn play_music_forever__name(name: Arc<String>) {
        use crate::piston_script::Music;
        use crate::piston_script::TrackHandle;
        use kira::sound::streaming::StreamingSoundData;
        use std::path::Path;

        let music = unsafe { &mut *Current::<Music>::new() };
        let music_track = unsafe { &mut *Current::<TrackHandle>::new() };
        if let Some((file, sound_handle)) = music.get_mut(&name) {
            if let Some(sound_handle) = sound_handle {
                sound_handle.set_loop_region(..);
                sound_handle.resume(Default::default());
            } else {
                let sound_data = StreamingSoundData::from_file(Path::new(&**file)).unwrap();
                let sound = music_track.play(
                    sound_data.loop_region(..)
                ).unwrap();
                *sound_handle = Some(sound);
            }
        }
    }}

    dyon_fn!{fn set_music_volume(volume: f64) {
        use crate::piston_script::TrackHandle;

        let music_track = unsafe { &mut *Current::<TrackHandle>::new() };
        music_track.set_volume(amplitude_to_decibels(volume as f32), Default::default());
    }}
}

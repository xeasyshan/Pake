use crate::app::config::PakeConfig;
use std::{path::{ PathBuf}, env};
use tauri::{App, Window, WindowBuilder, WindowUrl};
use std::fs;

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;

pub fn get_window(app: &mut App, config: PakeConfig, _data_dir: PathBuf) -> Window {
    let window_config = config
        .windows
        .first()
        .expect("At least one window configuration is required");

    let user_agent = config.user_agent.get();

    let url = match window_config.url_type.as_str() {
        "web" => WindowUrl::App(window_config.url.parse().unwrap()),
        "local" => WindowUrl::App(PathBuf::from(&window_config.url)),
        _ => panic!("url type can only be web or local"),
    };

    let mut window_builder = WindowBuilder::new(app, "pake", url)
        .title("")
        .user_agent(user_agent)
        .visible(false) // Prevent initial shaking
        .resizable(window_config.resizable)
        .fullscreen(window_config.fullscreen)
        .inner_size(window_config.width, window_config.height)
        .disable_file_drop_handler() //Very annoying, otherwise dragging files to the window will not work.
        .initialization_script(include_str!("../inject/style.js"))
        .initialization_script(include_str!("../inject/event.js"))
        .initialization_script(include_str!("../inject/component.js"));

    #[cfg(target_os = "macos")]
    {
        let title_bar_style = if window_config.transparent {
            TitleBarStyle::Overlay
        } else {
            TitleBarStyle::Visible
        };
        window_builder = window_builder.title_bar_style(title_bar_style)
    }

    let current_dir = env::current_dir().unwrap();
    let dynamic_path = PathBuf::from("src/inject/dynamic.js");
    let absolute_path = current_dir.join(dynamic_path);
    match fs::metadata(&absolute_path) {
        Ok(metadata) => {
            if metadata.is_file() {
                window_builder = window_builder.initialization_script(include_str!("../inject/dynamic.js"));
            }
        }
        Err(_) => {}
    }

    #[cfg(not(target_os = "macos"))]
    {
        window_builder = window_builder.data_directory(_data_dir);
    }

    window_builder.build().unwrap()
}

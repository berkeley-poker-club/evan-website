use leptos::prelude::*;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "dark" => Theme::Dark,
            _ => Theme::Light,
        }
    }
}

#[derive(Clone)]
pub struct ThemeContext {
    pub theme: ReadSignal<Theme>,
    pub toggle: Arc<dyn Fn() + Send + Sync>,
}

fn get_system_theme() -> Theme {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(media_query)) = window.match_media("(prefers-color-scheme: dark)") {
            if media_query.matches() {
                return Theme::Dark;
            }
        }
    }
    Theme::Light
}

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let initial_theme = {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(theme_str)) = storage.get_item("theme") {
                    Theme::from_str(&theme_str)
                } else {
                    get_system_theme()
                }
            } else {
                get_system_theme()
            }
        } else {
            Theme::Light
        }
    };

    let (theme, set_theme) = signal(initial_theme);

    Effect::new(move |_| {
        let current_theme = theme.get();

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let class_list = html.class_list();
                    match current_theme {
                        Theme::Dark => {
                            let _ = class_list.add_1("dark");
                        }
                        Theme::Light => {
                            let _ = class_list.remove_1("dark");
                        }
                    }
                }
            }

            // Save to localStorage
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("theme", current_theme.as_str());
            }
        }
    });

    let toggle_theme = Arc::new(move || {
        set_theme.update(|t| {
            *t = match *t {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            }
        });
    });

    let ctx = ThemeContext {
        theme,
        toggle: toggle_theme,
    };

    provide_context(ctx);

    children()
}

pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().expect("ThemeProvider must be present in component tree")
}

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

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    // Initialize theme from localStorage or default to light
    let initial_theme = {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(theme_str)) = storage.get_item("theme") {
                    Theme::from_str(&theme_str)
                } else {
                    Theme::Light
                }
            } else {
                Theme::Light
            }
        } else {
            Theme::Light
        }
    };

    let (theme, set_theme) = signal(initial_theme);

    // Apply theme to document on initialization and changes
    Effect::new(move |_| {
        let current_theme = theme.get();

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    // Add or remove 'dark' class on <html> element
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

// Hook to use theme in components
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>()
        .expect("ThemeProvider must be present in component tree")
}

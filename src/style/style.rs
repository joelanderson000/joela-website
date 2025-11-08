use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct Theme {
    pub colours: &'static str,
    pub font: &'static str,
    accent: &'static str,
    pub classes: &'static str,
}

pub static LIGHT: Lazy<Theme> = Lazy::new(|| Theme {
    colours: "bg-gray-900 text-white",
    font: "font-sans",
    accent: "text-blue-500",
    classes: "",
});

pub static DARK: Lazy<Theme> = Lazy::new(|| Theme {
    colours: "bg-slate-900 text-white",
    font: "font-sans",
    accent: "text-blue-400",
    classes: "",
});
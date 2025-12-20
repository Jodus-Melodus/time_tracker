use tray_icon;

pub struct AppState {
    pub _tray_icon: tray_icon::TrayIcon,
}

#[derive(PartialEq)]
pub enum UserState {
    Idle,
    Active,
}

use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::Mutex;
use tauri::{path::BaseDirectory, AppHandle, Manager, Wry};
use tauri_plugin_shell::ShellExt;

use crate::seelenweg::SeelenWeg;

lazy_static! {
    pub static ref SEELEN: Arc<Mutex<Seelen>> = Arc::new(Mutex::new(Seelen::default()));
}

pub struct Seelen {
    handle: Option<AppHandle<Wry>>,
    weg: Option<SeelenWeg>,
}

impl Default for Seelen {
    fn default() -> Self {
        Self {
            handle: None,
            weg: None,
        }
    }
}

impl Seelen {
    pub fn handle(&self) -> &AppHandle<Wry> {
        self.handle.as_ref().unwrap()
    }

    pub fn weg(&self) -> &SeelenWeg {
        self.weg.as_ref().unwrap()
    }

    pub fn set_handle(&mut self, app: AppHandle<Wry>) {
        self.handle = Some(app.clone());
        self.weg = Some(SeelenWeg::new(app.clone()));
    }

    pub fn start_ahk_shortcuts(&self) {
        tauri::async_runtime::spawn(async move {
            let app = SEELEN.lock().handle().clone();

            let ahk_path = app
                .path()
                .resolve("static/seelen.ahk", BaseDirectory::Resource)
                .expect("Failed to resolve path")
                .to_str()
                .expect("Failed to convert path to string")
                .to_owned()
                .trim_start_matches("\\\\?\\")
                .to_owned();

            app.shell()
                .command("cmd")
                .args(["/C", &ahk_path])
                .spawn()
                .expect("Failed to spawn shortcuts");
        });
    }

    pub fn kill_ahk_shortcuts(&self) {
        self.handle()
            .shell()
            .command("powershell")
            .args([
                "-ExecutionPolicy",
                "Bypass",
                "-NoProfile",
                "-Command",
                "Get-WmiObject Win32_Process | Where-Object { $_.CommandLine -like '*seelen.ahk*' } | ForEach-Object { Stop-Process -Id $_.ProcessId -Force }",
            ])
            .spawn()
            .expect("Failed to close ahk");
    }

    pub fn start_komorebi_manager(&self) {
        tauri::async_runtime::spawn(async move {
            let app = SEELEN.lock().handle().clone();

            let config_route = app
                .path()
                .resolve(".config/komorebi-ui/settings.json", BaseDirectory::Home)
                .expect("Failed to resolve path")
                .to_str()
                .unwrap_or("")
                .to_string();

            app.shell()
                .command("komorebi-wm.exe")
                .args(["-c", &config_route])
                .spawn()
                .expect("Failed to spawn komorebi");
        });
    }
}
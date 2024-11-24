use std::path::PathBuf;

use seelen_core::{handlers::SeelenEvent, state::Widget};
use tauri::Emitter;

use crate::{error_handler::Result, seelen::get_app_handle};

use super::FullState;

impl FullState {
    pub(super) fn emit_widgets(&self) -> Result<()> {
        get_app_handle().emit(SeelenEvent::StateWidgetsChanged, &self.plugins)?;
        Ok(())
    }

    fn load_widget_from_file(path: PathBuf) -> Result<Widget> {
        Ok(serde_yaml::from_str(&std::fs::read_to_string(&path)?)?)
    }

    pub(super) fn load_widgets(&mut self) -> Result<()> {
        let user_path = self.data_dir.join("widgets");
        let bundled_path = self.resources_dir.join("static/widgets");

        let entries = std::fs::read_dir(&bundled_path)?.chain(std::fs::read_dir(&user_path)?);
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                continue;
            }
            match Self::load_widget_from_file(path) {
                Ok(widget) => {
                    self.widgets.insert(widget.id.clone(), widget);
                }
                Err(e) => {
                    log::error!("Failed to load widget: {}", e);
                }
            }
        }
        Ok(())
    }
}

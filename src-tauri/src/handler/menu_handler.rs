use crate::handler::image_handler::create_image;
use crate::models::NewMenuDetail;
use crate::{get_conn, DbPool};
use base64::decode;
use tauri::{command, State};

#[command]
pub fn add_new_menu(state: State<DbPool>, menu: NewMenuDetail) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;
    let image_data = decode(&menu.image_data).map_err(|_| "Invalid Base encoding".to_string())?;

    let id = create_image(conn, image_data, menu.mime_type, menu.image_name)?;

    Ok(())
}

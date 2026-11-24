use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct FolderItem {
    pub path: String,
    pub is_folder: bool,
}

#[tauri::command]
pub fn list_folder(working_directory: &str, show_folders_only: bool) -> Result<Vec<FolderItem>, String> {
    /*
     * This function lists all the files and folders inside the
     * given working directory. When show_folders_only is true,
     * it only lists the folders inside.
     *
     * Arguments:
     *    working_directory: string -> the path to the working directory
     *    show_folders_only: bool -> true lists folders only,
     *                               false lists files and folders
     *
     * Returns:
     *    Result<Vec<FolderItem>, String> -> a list of paths with a folder/file flag,
     *                                       or the reason the folder can't be read
     */
    let mut contents = Vec::new();

    fs::create_dir_all(working_directory)
        .map_err(|e| format!("Cannot open {working_directory}: {e}"))?;

    let entries = fs::read_dir(working_directory)
        .map_err(|e| format!("Cannot read {working_directory}: {e}"))?;

    for entry in entries.flatten() {
        let path = entry.path();
        let is_folder = path.is_dir();

        if !show_folders_only || is_folder {
            contents.push(FolderItem {
                path: path.to_str().unwrap_or_default().to_string(),
                is_folder,
            });
        }
    }

    Ok(contents)
}

#[tauri::command]
pub fn toggle_mod(
    mod_folder: String,
    working_directory: String,
    mods_folder: String,
    enable: bool,
    type_of: String,
) -> Result<String, String> {
    /*
     * This function toggles a mod between enabled and disabled.
     * When disabling, the mod moves into the disabled-mods folder.
     * When enabling, the mod moves back into the engine's mods folder.
     * The disabled-mods folder is created if it does not exist.
     *
     * Arguments:
     *    mod_folder: string -> the full path to the mod folder to move
     *    working_directory: string -> the path to the engine's folder
     *    mods_folder: string -> the name of the mods folder for this engine
     *    enable: bool -> true moves the mod into mods, false moves it to disabled-mods
     *    type_of: string -> the type of mod to toggle (e.g. "mod", "addons", "content")
     *
     * Returns:
     *    Result<String, String> -> a message or an error message
     */
    let disabled_folder = format!("{working_directory}/disabled-{type_of}");
    fs::create_dir_all(&disabled_folder)
        .map_err(|e| format!("Failed to create disabled-mods folder: {e}"))?;

    let folder_name = mod_folder.rsplit(['/', '\\']).next().unwrap_or(&mod_folder);
    let mods_path = format!("{working_directory}/{mods_folder}");

    let destination = if enable {
        format!("{mods_path}/{folder_name}")
    } else {
        format!("{disabled_folder}/{folder_name}")
    };

    fs::rename(&mod_folder, &destination)
        .map_err(|e| format!("Failed to move mod: {e}"))?;

    if enable {
        Ok(format!("Enabled mod: {folder_name}"))
    } else {
        Ok(format!("Disabled mod: {folder_name}"))
    }
}

#[tauri::command]
pub fn trash_folder(mod_folder: String) -> Result<String, String> {
    /*
     * This functions deletes the mod given from the
     * arguemnt.
     * 
     * Arguments:
     *     mod_folder: String -> The mod to trash.
     */

    fs::remove_dir_all(&mod_folder)
        .map_err(|e| format!("Failed to delete mod: {e}"))?;

    Ok(format!("Trashed mod: {mod_folder}"))
}

#[tauri::command]
pub fn paste_to_dir(to_copy: &str, dest_to_paste: &str) -> Result<String, String> {
    /*
     * This function copies a folder from one place to another.
     * Folders are copied recursively, so all their contents come along.
     *
     * Arguments:
     *    to_copy: string -> the path of the folder to copy
     *    dest_to_paste: string -> the destination folder path
     *
     * Returns:
     *    Result<String, String> -> a success message or an error message
     */
    let source = fs::metadata(to_copy)
        .map_err(|e| format!("Failed to read source path: {e}"))?;

    if !source.is_dir() {
        return Err(format!("The source path is not a folder: {to_copy}"));
    }

    let folder_name = std::path::Path::new(to_copy)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("mod");

    let final_destination = std::path::Path::new(dest_to_paste).join(folder_name);
    let final_destination = final_destination.to_str().unwrap_or_default();

    copy_folder_recursive(to_copy, final_destination)?;

    Ok(format!("Sucessfully copied {to_copy} to {final_destination}"))
}

fn copy_folder_recursive(source: &str, destination: &str) -> Result<(), String> {
    /*
     * This function copies a folder and everything inside it.
     *
     * Arguments:
     *    source: string -> the folder to copy
     *    destination: string -> where to copy the folder to
     *
     * Returns:
     *    Result<(), String> -> nothing, or an error message
     */
    fs::create_dir_all(destination)
        .map_err(|e| format!("Failed to create destination folder: {e}"))?;

    let entries = fs::read_dir(source)
        .map_err(|e| format!("Failed to read source folder: {e}"))?;

    for entry in entries.flatten() {
        let entry_path = entry.path();
        let new_path = std::path::Path::new(destination).join(entry.file_name());

        if entry_path.is_dir() {
            copy_folder_recursive(
                &entry_path.to_str().unwrap_or_default(),
                &new_path.to_str().unwrap_or_default(),
            )?;
        } else {
            fs::copy(&entry_path, &new_path)
                .map_err(|e| format!("Failed to copy file: {e}"))?;
        }
    }

    Ok(())
}
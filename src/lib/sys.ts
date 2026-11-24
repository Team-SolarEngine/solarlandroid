import { open } from "@tauri-apps/plugin-dialog";

export async function pickFile(
    extensions: string[] = ["*"],
    name: string = "File",
    directory: boolean = false,
): Promise<string> {
    /*
     * This function opens a file picker dialog and returns
     * the path of the selected file, or an empty string if cancelled.
     *
     * When extensions is ["*"] (the default), no filter is applied
     * so any file, including .exe and extensionless files, can be picked.
     */
    const file = await open({
        directory,
        multiple: false,
        ...(extensions.length === 1 && extensions[0] === "*" ? {} : {
            filters: [{
                name,
                extensions,
            }],
        }),
    });
    return file ?? "";
}
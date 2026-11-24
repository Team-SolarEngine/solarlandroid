export function formatBytes(bytes: number) {
    /*
        * This function turns a byte count into a readable size string.
        *
        * Arguments:
        *    bytes: number -> the size in bytes
        *
        * Returns:
        *    string -> the size in a readable format
        */
    if (!bytes) return "";
    const units = ["B", "KB", "MB", "GB"];
    let i = 0;
    let value = bytes;
    while (value >= 1024 && i < units.length - 1) {
        value /= 1024;
        i++;
    }
    return `${value.toFixed(1)} ${units[i]}`;
}

function truncateString(str: string, max_length: number) {
    return str.length > max_length ? str.slice(0, max_length).trimEnd() + '...' : str;
}
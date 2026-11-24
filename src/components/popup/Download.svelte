<script lang="ts">
    import { formatBytes } from "$lib/format";
    import { invoke } from "@tauri-apps/api/core";
    import { downloadDir, homeDir } from "@tauri-apps/api/path";
    import { error } from "@tauri-apps/plugin-log";
    import { download } from "@tauri-apps/plugin-upload";

    type DownloadVars = {
        open: boolean;
        url: string;
        targetted_path: string;
        download_name: string;
        file_name?: string;
    };

    let { dlpvar = $bindable<DownloadVars>() } = $props()

    let download_bytes = $state(0);
    let total_bytes = $state(0);
    let percent = $state(0);
    let bytes_per_second = $state(0);
    let status = $state("");
    let download_error = $state("");
    let running = $state(false);

    // Plain guard: reactive state here would retrigger the $effect below.
    let started = false;

    function fileName() {
        const raw = dlpvar.file_name || dlpvar.download_name || "mod.zip";
        const clean = raw.replace(/[^a-zA-Z0-9._-]+/g, "_").replace(/^_+|_+$/g, "");
        return clean || "mod.zip";
    }

    async function start_download() {
        if (started) return;
        started = true;
        running = true;

        download_error = "";
        status = "Starting...";
        percent = 0;
        download_bytes = 0;
        total_bytes = 0;
        bytes_per_second = 0;

        try {
            if (!dlpvar.url) throw new Error("No download URL was selected.");
            if (!dlpvar.targetted_path) throw new Error("No instance was selected.");

            const name = fileName();
            const temp_path = `${await downloadDir()}/solarlandroid-${Date.now()}-${name}`;
            const relative = dlpvar.targetted_path.split("/").filter(Boolean).join("/");
            const final_dir = `${await homeDir()}/${relative}/mods`;

            status = "Downloading...";
            await download(dlpvar.url, temp_path, ({ progressTotal, total, transferSpeed }) => {
                percent = total > 0
                    ? Math.min(100, Math.round((progressTotal / total) * 100))
                    : 0;

                download_bytes = progressTotal;
                total_bytes = total;
                bytes_per_second = transferSpeed;
            });

            status = "Unzipping...";
            await invoke("shizuku_make_dirs", { path: final_dir });
            await invoke("shizuku_unzip", { zip: temp_path, dest: `${final_dir}/${name}` });
            await invoke("shizuku_delete", { path: temp_path });

            percent = 100;
            status = "Done.";
        } catch(e) {
            download_error = `${e}`;
            status = "Failed.";
            error(`${e}`)
        } finally {
            running = false;
        }
    }

    $effect(() => {
        if (!dlpvar.open) {
            started = false;
            return;
        }

        start_download();
    })
</script>

<div class="overlay" class:active={dlpvar.open}></div>
<dialog class="top" style="padding-top: 4rem;" class:active={dlpvar.open}>
    <h4>{dlpvar.download_name || "Download"}</h4>
    <span>{status || "Sit back, relax, and watch it download. I wouldn't reccomend exiting the app, though!"}</span>

    {#if download_error}
        <article class="border red">
            <span>{download_error}</span>
        </article>
    {/if}

    <div style="display: flex; flex-direction: column;">
        <progress class="wavy" value={percent} max="100"></progress>
        <div style="display: flex; flex-direction: row; justify-content: flex-end;">
            {#if bytes_per_second}
            <span>{formatBytes(download_bytes)}/{formatBytes(total_bytes)} ({formatBytes(bytes_per_second)})</span>
            {:else}
            <span>Fetching HTTP request...</span>
            {/if}
        </div>
    </div>

    <div class="right-align no-space" style="margin-top: 16px;">
        <button class="transparent" disabled={running} onclick={() => dlpvar.open = false}>
            <i>close</i> Close
        </button>
    </div>
</dialog>

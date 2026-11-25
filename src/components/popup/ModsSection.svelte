<script lang="ts">
    import type { Mods } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";
    import { homeDir } from "@tauri-apps/api/path";
    import { error } from "@tauri-apps/plugin-log";
    import { getContext } from "svelte";

    let { msvar = $bindable() } = $props()
    type StorageItem = { path: string; isFolder: boolean };
    const errorPopup: any = getContext('error')

    let mods = $state<Array<Mods>>([]);
    let test_path = $state("")
    let list_error = $state("")

    let shizuku = $state({
        binder: false,
        permissionGranted: false,
        serviceReady: false,
        error: "",
    })

    async function refreshShizukuStatus() {
        try {
            const status: any = await invoke("shizuku_status");
            shizuku.binder = !!status.binder;
            shizuku.permissionGranted = !!status.permissionGranted;
            shizuku.serviceReady = !!status.serviceReady;
            shizuku.error = status.error ?? "";
        } catch (e) {
            shizuku.error = `${e}`;
            errorPopup({
                active: true,
                action: "Refresh Shizuku Status",
                what_happened: "Probably wasn't allowed or user doesn't have Shizuku enabled.",
                what_should_happen: "Refresh Shizuku to check if it's enabled.",
                error_log: e
            })
        }
    }

    async function requestShizuku() {
        try {
            list_error = "";
            await invoke("shizuku_request_permission");

            for (let i = 0; i < 10; i++) {
                await refreshShizukuStatus();
                if (shizuku.serviceReady) break;
                await new Promise((resolve) => setTimeout(resolve, 200));
            }

            if (shizuku.serviceReady) await listMods();
        } catch (e) {
            list_error = `${e}`;
            errorPopup({
                active: true,
                action: "Request Shizuku Permission",
                what_happened: "Probably wasn't allowed or user doesn't have Shizuku enabled.",
                what_should_happen: "Request for Shizuku's permission.",
                error_log: e
            })
        }
    }

    async function listShizukuFolder(path: string): Promise<StorageItem[]> {
        const result: any = await invoke("shizuku_list_folder", { path });
        return (result.items ?? []).filter((item: StorageItem) => item.isFolder);
    }

    async function listMods() {
        mods = [];
        list_error = "";

        if (!msvar.working_dir) return;

        try {
            test_path = `${await homeDir()}/${msvar.working_dir}`;

            await invoke("shizuku_make_dirs", { path: `${test_path}/mods` });
            await invoke("shizuku_make_dirs", { path: `${test_path}/disabled-mods` });

            const enabledFolders = await listShizukuFolder(`${test_path}/mods`);
            for (const modFolder of enabledFolders) {
                mods.push({ folder: modFolder.path, name: modFolder.path.split(/[/\\]/).pop() ?? modFolder.path, enabled: true });
            }

            const disabledFolders = await listShizukuFolder(`${test_path}/disabled-mods`);
            for (const modFolder of disabledFolders) {
                mods.push({ folder: modFolder.path, name: modFolder.path.split(/[/\\]/).pop() ?? modFolder.path, enabled: false });
            }
        } catch (e) {
            list_error = `${e}`;
            errorPopup({
                active: true,
                action: "List Mods",
                what_happened: "Shizuku might've not been enabled, or something went wrong.",
                what_should_happen: "List mods content",
                error_log: e
            })
        }
    }

    async function toggleMod(mod: { folder: string }, enable: boolean) {
        try {
            const base = `${await homeDir()}/${msvar.working_dir}`;
            const folderName = mod.folder.split(/[/\\]/).pop() ?? mod.folder;
            const destination = `${base}/${enable ? "mods" : "disabled-mods"}/${folderName}`;

            await invoke("shizuku_rename", { from: mod.folder, to: destination });
            await listMods();
        } catch (e) {
            list_error = `${e}`;
            errorPopup({
                active: true,
                action: "Toggle mod",
                what_happened: "Shizuku might've not been enabled, or something went wrong.",
                what_should_happen: `Toggle ${mod.folder} to ${enable ? "enabled" : "disabled"}`,
                error_log: e
            })
        }
    }

    async function trashMod(modFolder: string) {
        try {
            await invoke("shizuku_delete", { path: modFolder });
            await listMods();
        } catch (e) {
            list_error = `${e}`;
            errorPopup({
                active: true,
                action: "Delete mod",
                what_happened: "Shizuku might've not been enabled, or something went wrong.",
                what_should_happen: `Delete mod ${modFolder}`,
                error_log: e
            })
        }
    }

    async function openFolder(path: string) {
        try {
            await invoke("shizuku_open_folder", { path });
        } catch(e) {
            list_error = `${e}`;
            errorPopup({
                active: true,
                action: "Open Folder",
                what_happened: "Shizuku might've not been enabled, or something went wrong.",
                what_should_happen: `Open folder at ${path}`,
                error_log: e
            })
        }
    }

    $effect(() => {
        msvar.open;
        msvar.working_dir;
        refreshShizukuStatus();
        listMods()
    })
</script>

<div class="overlay" class:active={msvar.open} onclick={() => msvar.open = false}></div>
<dialog class="right" class:active={msvar.open}>
    <h3>Mod Manager</h3>
    <span>Manage your mods here! You can open the folder, delete content, or enable/disable them!</span>

    {#if !shizuku.serviceReady}
        <article class="border orange">
            <span>
                {!shizuku.binder
                    ? "Start Shizuku, then open the mod manager."
                    : shizuku.permissionGranted
                        ? "Connecting to Shizuku..."
                        : "Grant Shizuku access to manage Android/obb folders."}
            </span>
            {#if shizuku.binder && !shizuku.permissionGranted}
                <button class="border" onclick={requestShizuku}><i>key</i> Grant Shizuku</button>
            {/if}
        </article>
    {/if}

    {#if list_error}
        <article class="border red">
            <span>{list_error}</span>
        </article>
    {/if}

    <hr class="medium" />

    {#if mods.length > 0}
        {#each mods as mod}
            <article style={mod.enabled ? "" : "opacity: 0.5;"}>
                <div style="display: flex; flex-direction: row; align-items: center;">
                    <div style="flex: 1; min-width: 0; overflow-wrap: anywhere;">
                        <h6>{mod.name}</h6>
                    </div>

                    <div class="no-space" style="margin-left: 12px; display: flex; flex-direction: row; align-items: center;">
                        <button class="transparent circle" onclick={() => openFolder(mod.folder)}><i>folder</i></button>
                        <button class="transparent circle" onclick={() => trashMod(mod.folder)}><i>delete</i></button>
                        <label class="checkbox large">
                            <input type="checkbox" checked={mod.enabled} onchange={() => toggleMod(mod, !mod.enabled)} />
                            <span></span>
                        </label>
                    </div>
                </div>
            </article>
        {/each}
    {:else if !list_error}
        <span>Mods is empty. Try adding mods! <code>{test_path}</code></span>
    {/if}

    <div class="right-align no-space" style="margin-top: 16px;">
        <button class="transparent" onclick={() => openFolder(test_path)}>
            <i>folder</i> Open Folder
        </button>
        <button class="transparent" onclick={() => msvar.open = false}>
            <i>close</i> Close
        </button>
    </div>
</dialog>

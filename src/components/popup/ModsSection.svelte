<script lang="ts">
    import type { Mods } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";
    import { error } from "@tauri-apps/plugin-log";

    let {
        working_dir = "",
        open = false,
    } = $props()

    let mods = $state<Array<Mods>>([]);

    async function listMods() {
        /*
         * This function lists all the mods from both the mods folder
         * and the disabled-mods folder, combined into a single list.
         * Each mod keeps an enabled flag so it can be toggled.
         *
         * Returns:
         *     mods: Array -> An array of mod objects.
         */

        try {
            mods = [];

            const enabledFolderFinal = `${working_dir}/mods`
            const enabledFolders: any[] = await invoke("list_folder", { workingDirectory: enabledFolderFinal, showFoldersOnly: true });
            for (const modFolder of enabledFolders) {
                mods.push({ folder: modFolder, name: modFolder.split(/[/\\]/).pop() ?? modFolder, enabled: true });
            }

            let disabledFolderFinal = `${working_dir}/disabled-mods`;
            const disabledFolders: any[] = await invoke("list_folder", { workingDirectory: disabledFolderFinal, showFoldersOnly: true });
            for (const modFolder of disabledFolders) {
                mods.push({ folder: modFolder, name: modFolder.split(/[/\\]/).pop() ?? modFolder, enabled: false });
            }
        } catch (e) {
            error(`${e}`)
        }
    }

    async function toggleMod(mod: { folder: string }, enable: boolean) {
        /*
         * This function moves a mod between the mods folder and the
         * disabled-mods folder, then refreshes both lists.
         */
        try {
            await invoke("toggle_mod", {
                modFolder: mod.folder,
                workingDirectory: working_dir,
                modsFolder: "mods",
                enable,
                typeOf: "mods",
            });
            await listMods();
        } catch (e) {
            error(`${e}`)
        }
    }

    async function trashMod(modFolder: string) {
        /*
         * This functions calls the Tauri backend to
         * trash the folder given from the arguments.
         * 
         * Arguments:
         *    modFolder: string -> The folder you want to delete
         */

        try {
            await invoke("trash_folder", { modFolder });
            await listMods();
        } catch (e) {
            error(`${e}`)
        }
    }

    $effect(() => {
        if (open) {
            listMods()
        }
    })
</script>

<div class="overlay" class:active={open} onclick={() => open = false}></div>
<dialog class="bottom" class:active={open}>
    {#if mods.length > 1}
        {#each mods as mod}
            <article style={mod.enabled ? "" : "opacity: 0.5;"}>
                <div style="display; flex; flex-direction: row;">
                    <div style="flex: 1; min-width: 0;">
                        <h6>{mod.name}</h6>
                    </div>

                    <div class="no-space row" style="margin-left: 12px;">
                        <button class="transparent circle" onclick={() => trashMod(mod.folder)}><i>delete</i></button>
                        <label class="checkbox large">
                            <input type="checkbox" checked={mod.enabled} onchange={() => toggleMod(mod, !mod.enabled)} />
                            <span></span>
                        </label>
                    </div>
                </div>
            </article>
        {/each}
    {:else}
        <span>Mods is empty. Try adding mods!</span>
    {/if}
</dialog>
<script lang="ts">
    import { formatBytes } from "$lib/format";
    import { error } from "@tauri-apps/plugin-log";
    import CardApp from "../CardApp.svelte";
    import type { Instance } from "$lib/types";
    import { getContext, onMount } from "svelte";
    import { store } from "$lib/store";

    let {
        modalGameBanana = $bindable(),
        modId = 0,
    } = $props()

    const download: any = getContext('download')

    let gb_loading = $state(false);
    let gb_name = $state("");
    let gb_desc = $state("");
    let gb_banner_url = $state("");
    let gb_downloads = $state<Array<{ name: string; url: string; size: string; file_name: string }>>([]);
    let gb_valid_fnf = $state(true);

    let selected_to_download = $state("")
    let selected_file_name = $state("")
    let instances: Instance[] = $state([]);
    async function loadInstances() { instances = (await store.get<Instance[]>("instances")) ?? []; }

    $effect(() => {
        if (modalGameBanana) {
            load_mod()
        }
    })

    async function load_mod() {
        gb_loading = true
        gb_name = ""
        gb_desc = ""
        gb_banner_url = ""
        gb_downloads = []
        selected_to_download = ""
        selected_file_name = ""

        try {
            const response = await fetch(`https://gamebanana.com/apiv11/Mod/${modId}/ProfilePage`);
            const mod = await response.json();

            gb_name = mod._sName || "Unknown mod";
            gb_desc = mod._sText || "";
            gb_valid_fnf = mod._aGame._idRow == 8694;

            const media = mod._aPreviewMedia?._aImages || [];
            const urls = media
                .map((img: any) => img._sBaseUrl + "/" + img._sFile)
                .filter(Boolean);
            gb_banner_url = urls[0] || "";

            gb_downloads = (mod._aFiles || []).map((file: any) => ({
                name: file._sFile,
                url: file._sDownloadUrl,
                size: formatBytes(file._nFilesize),
                file_name: file._sFile || file._sName || "mod.zip",
            }));
        } catch (e) {
            error(`${e}`)
        }

        gb_loading = false;
    }

    function startDownload(targetted_path: string) {
        modalGameBanana = false;
        download({
            open: true,
            url: selected_to_download,
            targetted_path,
            download_name: gb_name,
            file_name: selected_file_name,
        })
    }

    onMount(() => {
        loadInstances();
        const interval = setInterval(loadInstances, 1000);
        return () => clearInterval(interval);
    });
</script>

<div class="overlay" class:active={modalGameBanana} onclick={() => modalGameBanana = false}></div>
<dialog class="bottom" class:active={modalGameBanana}>
    {#if gb_loading}
        <span>Loading...</span>
    {:else}
        {#if !gb_valid_fnf}
            <article class="tertiary">
                <div class="row">
                    <i>warning</i>
                    <h4>Warning</h4>
                </div>
                <span>This mod submission does not look like it supports FNF! If you think we got this wrong, please contact us through GitHub issue or Discord!</span>
            </article>
            <hr class="medium">
        {/if}

        {#if gb_downloads.length > 0}
            <h6 style="margin-bottom: 10px;">Downloading for <b>{gb_name}</b>...</h6>
            <img src={gb_banner_url} alt={gb_name} style="width: 100%; height: 200px; object-fit: cover;" class="round" />
            <div style="display: flex; flex-direction: column; gap: 0.5rem; margin-top: 10px;">
                {#each gb_downloads as download}
                    <article onclick={() => {
                        selected_to_download = download.url;
                        selected_file_name = download.file_name;
                    }} style="cursor: pointer; display: flex; justify-content: space-between; align-items: center; gap: 0.5rem;">
                        <div style="min-width: 0; overflow-wrap: break-word;">
                            <i>download</i>
                            <span><b>{download.name}</b></span>
                        </div>
                        <span>{download.size}</span>
                    </article>
                {/each}
            </div>
        {:else}
            <span>No download assets were found.</span>
        {/if}

        {#if selected_to_download}
            <hr class="medium" />
            <div style="display: flex; flex-direction: column; gap: 0.4rem;">
                {#each instances as i}
                    <div onclick={() => startDownload(i.wd)}>
                        <CardApp
                            name={i.name}
                            image={i.icon}
                            description={i.desc}
                            working_dir={i.wd}
                            preview={true}
                        />
                    </div>
                {:else}
                    <span>No instances found...</span>
                {/each}
            </div>
        {/if}
    {/if}
</dialog>
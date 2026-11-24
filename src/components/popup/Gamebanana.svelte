<script lang="ts">
    import { formatBytes } from "$lib/format";
    import { debug, error } from "@tauri-apps/plugin-log";

    let {
        modalGameBanana = $bindable(),
        modId = 0,
    } = $props()

    let gb_loading = $state(false);
    let gb_name = $state("");
    let gb_desc = $state("");
    let gb_banner_url = $state("");
    let gb_downloads = $state<Array<{ name: string; url: string; size: string }>>([]);
    let gb_valid_fnf = $state(true);

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
            }));
        } catch (e) {
            error(`${e}`)
        }

        gb_loading = false;
    }
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
                    <article onclick={() => debug(download.url)} style="cursor: pointer; display: flex; justify-content: space-between; align-items: center; gap: 0.5rem;">
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
    {/if}
</dialog>
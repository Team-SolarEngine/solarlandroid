<script lang="ts">
    import { getVersion } from "@tauri-apps/api/app";
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { onMount } from "svelte";

    let ver_onl = $state("")
    let already_fetched = $state("")
    let ver_loc = $state("")

    class VersionFunc {
        async latest_online() {
            const res = await fetch('https://api.github.com/repos/team-solarengine/solarlandroid/tags')
                .catch((_) => { already_fetched = "failed" })
            const dat = await res.json()
            ver_onl = dat[0].name
        }

        async local() {
            ver_loc = await getVersion();
        }
    }

    onMount(() => {
        const versions = new VersionFunc()
        void versions.latest_online()
        void versions.local()
    })
</script>

<div>
    <article>
        <h4>Information</h4>
        <span>SolarLanDroid is the Solar Launcher mobile version for managing FNF mods! Made with Tauri, Svelte, and BeerCSS.</span>
        <nav class="group connected" style="justify-content: center;">
            <button class="border left-round" onclick={() => openUrl("https://github.com/team-solarengine/solarlandroid")}>
                <i>commit</i>
                <span>Github</span>
            </button>
            <button class="border right-round" onclick={() => openUrl("https://discord.gg/RaHmP5fgyA")}>
                <i>chat</i>
                <span>Discord</span>
            </button>
        </nav>
    </article>

    <article>
        <h4>App Version</h4>
        <div style="margin: 8px 0; display: flex; flex-direction: row; justify-content: space-between; align-items: center;">
            <div style="display: flex; flex-direction: column; justify-content: center; align-items: center; transform: translateX(4rem);">
                {#if ver_onl == ""}
                    <div class="shape loading-indicator small"></div>
                {:else if ver_onl != ""}
                    <i>check</i>
                {:else if already_fetched == "failed" && ver_onl == ""}
                    <i>close</i>
                {/if}

                {#if ver_onl == ""}
                    <span>Loading...</span>
                {:else if ver_onl != ""}
                    <span>Online: {ver_onl}</span>
                {:else if already_fetched == "failed" && ver_onl == ""}
                    <span>Failed to fetch!</span>
                {/if}
            </div>

            <div style="display: flex; flex-direction: column; justify-content: center; align-items: center; transform: translateX(-4rem);">
                {#if ver_loc == ""}
                    <div class="shape loading-indicator small"></div>
                {:else if ver_loc != ""}
                    <i>check</i>
                {/if}

                {#if ver_loc == ""}
                    <span>Loading...</span>
                {:else if ver_loc != ""}
                    <span>Local: {ver_loc}</span>
                {/if}
            </div>
        </div>

        <div class="center-align">
            {#if ver_loc == "" || ver_onl == ""}
                <span>Fetching...</span>
            {:else if ver_loc != "" && already_fetched == "failed"}
                <span>Failed to fetch for online!</span>
            {:else if ver_loc == ver_onl}
                <span>You're on the latest version!</span>
            {:else if ver_loc != ver_onl}
                <span>Your version is outdated!</span>
            {/if}
        </div>

        <div class="center-align">
            <button onclick={() => openUrl("https://github.com/team-solarengine/solarlandroid/releases")}>
                Open Releases
            </button>
        </div>
    </article>

    <hr class="medium" />

    <article>
        <h4>No settings yet!</h4>
        <span>Settings don't exist yet. We don't have one that's currently useful to us, so this is what you're <b>temporarily</b> getting!</span>
    </article>
</div>
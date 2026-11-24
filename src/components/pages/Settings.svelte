<script lang="ts">
    import { openUrl } from "@tauri-apps/plugin-opener";

    
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

    <hr class="medium" />

    <span>settings go here I guess</span>
</div>
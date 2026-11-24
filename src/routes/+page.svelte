<script lang="ts">
    import "beercss";
    import "material-dynamic-colors";
    import { cubicOut } from "svelte/easing";
    import { fly } from 'svelte/transition';
    import Instances from "../components/pages/Instances.svelte";
    import Settings from "../components/pages/Settings.svelte";
    import { setContext, getContext, onMount } from "svelte";
    import Local from "../components/popup/Local.svelte";
    import { impactFeedback } from "@tauri-apps/plugin-haptics";
    import { getCurrent, onOpenUrl } from "@tauri-apps/plugin-deep-link";
    import { error } from "@tauri-apps/plugin-log";
    import Gamebanana from "../components/popup/Gamebanana.svelte";

    let current_page = $state("instances");

    const fly_in = { x: -40, y: -40, opacity: 0, duration: 250, easing: cubicOut };
    const fly_out = { y: 40, opacity: 0, duration: 200, easing: cubicOut };

    let local_instnace = $state({
        open: false,
        editing: false,
        name: "",
        desc: "",
        image: "",
        working_dir: ""
    })
    setContext('addInstance', (dat: any) => { local_instnace = dat })
    const addInstance: any = getContext('addInstance')

    async function switchPage(page: string) {
        current_page = page;
        await impactFeedback('light')
    }

    let modalGameBanana = $state(false);
    let gameBananaModId = $state(0);
    async function handleDeepLink(url: string) {
        /*
            * This function handles a solar-launch:// deep link.
            * It expects links like solar-launch://mods/<id>,
            * then opens the GameBanana mod popup with that id.
            *
            * Arguments:
            *    url: string -> the full deep link url
            */
        try {
            const parsed = new URL(url);
            const path = parsed.pathname.split("/").filter(Boolean);
            if (parsed.protocol === "solar-launch:" && parsed.host === "gb-mods") {
                gameBananaModId = Number(path[0]);
                modalGameBanana = true;
            }
        } catch (e) {
            error(`${e}`);
        }
    }

    onMount(() => {

        // deeplink support - taken from SLDesktop!
        onOpenUrl((urls) => { for (const url of urls) handleDeepLink(url); });
        getCurrent().then((urls) => { if (urls) for (const url of urls) handleDeepLink(url); });
    })
</script>

<div style="height: 100vh; display: flex; flex-direction: column;">
    <div style="flex: 1; margin-top: 2rem;">
        {#if current_page == "instances"}
            <div in:fly={fly_in} out:fly={fly_out} style="
                position: fixed;
                left: 0;
                right: 0;
                width: 100%;
                box-sizing: border-box;
                padding: 0 1rem;
                overflow-x: hidden;
            ">
                <h2>Instances</h2>
                <span>This is all where your instances go! Manage them! Or don't. We needed a heading here, since it's looks empty.</span>
                <hr class="medium" />

                <Instances/>
            </div>
        {:else}
            <div in:fly={fly_in} out:fly={fly_out} style="
                position: fixed;
                left: 0;
                right: 0;
                width: 100%;
                box-sizing: border-box;
                padding: 0 1rem;
                overflow-x: hidden;
            ">
                <h2>Settings</h2>
                <span>Change your settings? Go ahead! Go crazy or something. Nothing is stopping you! Maybe?</span>
                <hr class="medium" />

                <Settings/>
            </div>
        {/if}
    </div>

    {#if current_page == "instances"}
        <div in:fly={fly_in} out:fly={fly_out} style="position: fixed; right: 0; bottom: 0; padding: 24px; margin-bottom: 6rem;">
            <button class="circle extra" onclick={() => addInstance({open: true})}><i>add</i></button>
        </div>
    {/if}

    <nav class="bottom">
        <a style="padding-top: 2.5rem;" class="vertical" onclick={() => switchPage("instances")} class:active={current_page == "instances"}><i>list</i> Instances</a>
        <a style="padding-top: 2.5rem;" class="vertical" onclick={() => switchPage("settings")} class:active={current_page == "settings"}><i>settings</i> Settings</a>
    </nav>
</div>

<Local bind:vars={local_instnace} />
<Gamebanana bind:modalGameBanana modId={gameBananaModId} />
<script lang="ts">
    import "beercss";
    import "material-dynamic-colors";
    import { cubicOut } from "svelte/easing";
    import { fly } from 'svelte/transition';
    import Instances from "../components/pages/Instances.svelte";
    import Settings from "../components/pages/Settings.svelte";
    import { setContext, getContext } from "svelte";
    import Local from "../components/popup/Local.svelte";

    let current_page = $state("instances");

    const fly_in = { x: -40, y: -40, opacity: 1, duration: 250, easing: cubicOut };
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
        <a style="padding-top: 2.5rem;" class="vertical" onclick={() => current_page = "instances"} class:active={current_page == "instances"}><i>list</i> Instances</a>
        <a style="padding-top: 2.5rem;" class="vertical" onclick={() => current_page = "settings"} class:active={current_page == "settings"}><i>settings</i> Settings</a>
    </nav>
</div>

<Local bind:vars={local_instnace} />
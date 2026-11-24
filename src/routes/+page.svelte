<script lang="ts">
    import "beercss";
    import "material-dynamic-colors";
    import { cubicOut } from "svelte/easing";
    import { fade, fly } from 'svelte/transition';
    import Instances from "../components/pages/Instances.svelte";
    import Settings from "../components/pages/Settings.svelte";
    import { setContext, getContext } from "svelte";
    import Local from "../components/popup/Local.svelte";

    let current_page = $state("instances");

    const fly_in = { y: -40, opacity: 1, duration: 250, easing: cubicOut };
    const fly_out = { y: 40, opacity: 0, duration: 250, easing: cubicOut };

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
    <div class="tabs">
        <a style="padding-top: 2.5rem;" class="vertical" onclick={() => current_page = "instances"} class:active={current_page == "instances"}><i>list</i> Instances</a>
        <a style="padding-top: 2.5rem;" class="vertical" onclick={() => current_page = "settings"} class:active={current_page == "settings"}><i>settings</i> Settings</a>
    </div>

    <div style="flex: 1; padding: 16px;">
        {#if current_page == "instances"}
            <div in:fly={fly_in} out:fly={fly_out} style="position: fixed;">
                <Instances/>
            </div>
        {:else}
            <div in:fly={fly_in} out:fly={fly_out} style="position: fixed;">
                <Settings/>
            </div>
        {/if}
    </div>

    {#if current_page == "instances"}
        <div in:fly={fly_in} out:fly={fly_out} style="position: fixed; right: 0; bottom: 0; padding: 24px;">
            <button class="circle extra" onclick={() => addInstance({open: true})}><i>add</i></button>
        </div>
    {/if}
</div>

<Local bind:vars={local_instnace} />
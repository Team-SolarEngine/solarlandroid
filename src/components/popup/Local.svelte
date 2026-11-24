<script lang="ts">
    import CardApp from "../CardApp.svelte";
    import { getContext } from "svelte";
    import type { Instance } from "$lib/types";
    import { store } from "$lib/store";

    let { vars = $bindable() } = $props()
    const snackbar: any = getContext('snackbar')

    let instance_name = $state("")
    let instance_icon = $state("")
    let instance_description = $state("")
    let instance_working_dir = $state("")

    function emptyVars() {
        instance_name = ""
        instance_icon = ""
        instance_description = ""
        instance_working_dir = ""
    }

    async function save() {
        if (!instance_name || !instance_working_dir) return snackbar({
            active: true,
            top: true,
            type: "error",
            text: "Either Name or Working Directory is missing. Please fill it in."
        });

        const instances = (await store.get<Instance[]>("instances")) ?? [];
        instances.push({
            name: instance_name,
            icon: instance_icon,
            desc: instance_description,
            wd: instance_working_dir,
        });

        await store.set("instances", instances);
        await store.save();
        vars.open = false;
        emptyVars();
    }
</script>

<div class="overlay" class:active={vars.open} onclick={() => vars.open = false}></div>
<dialog class="bottom" class:active={vars.open}>
    {#if !vars.editing}
        <h4>Adding a new instnace</h4>
    {:else}
        <h4>Editing a instnace</h4>
    {/if}

    <div class="field label border">
        <input type="text" bind:value={instance_name}>
        <label>Name</label>
        <output>The name of your instance!</output>
    </div>

    <div class="field label border">
        <input type="text" bind:value={instance_icon}>
        <label>Icon (URL)</label>
        <output>The URL path to your instance! We reccomend URL.</output>
    </div>

    <div class="field label border">
        <input type="text" bind:value={instance_description}>
        <label>Description</label>
        <output>What do you wanna describe your instance? Something cool?</output>
    </div>

    <div class="field label border">
        <input type="text" bind:value={instance_working_dir}>
        <label>Working Directory</label>
        <output>Where's your instance located? It should start at the home folder of your android! <b>HINT: Use Zarchiver.</b></output>
    </div>

    <CardApp
        name={instance_name || "Placeholder"}
        image={instance_icon || "/preview.jpg"}
        working_dir={instance_working_dir}
        description={instance_description || "This instance is a mod manager."}
        preview={true}
    />

    <div style="margin-top: 14px;" class="right-align" onclick={() => save()}>
        <button><i>add</i> Add</button>
    </div>
</dialog>
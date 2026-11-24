<script lang="ts">
    import CardApp from "../CardApp.svelte";
    import { getContext } from "svelte";
    import type { Instance } from "$lib/types";
    import { store } from "$lib/store";
    import { pickFile } from "$lib/sys";

    let { vars = $bindable() } = $props()
    const snackbar: any = getContext('snackbar')

    let instance_name = $state("")
    let instance_icon = $state("")
    let instance_description = $state("")
    let instance_working_dir = $state("")
    let original_working_dir = $state("");

    function emptyVars() {
        instance_name = "";
        instance_icon = "";
        instance_description = "";
        instance_working_dir = "";
        original_working_dir = "";
    }

    async function save() {
        if (!instance_name || !instance_working_dir) return snackbar({
            active: true,
            top: true,
            type: "error",
            text: "Either Name or Working Directory is missing. Please fill it in."
        });

        const instances = (await store.get<Instance[]>("instances")) ?? [];
        const updated: Instance = {
            name: instance_name,
            icon: instance_icon,
            desc: instance_description,
            wd: instance_working_dir,
        };

        const next = vars.editing
            ? instances.map((instance) =>
                instance.wd === original_working_dir ? updated : instance
            )
            : [...instances, updated];

        await store.set("instances", next);
        await store.save();

        vars.open = false;
        vars.editing = false;
        emptyVars();
    }

    $effect(() => {
        if (!vars.open) return;
    
        if (vars.editing) {
            instance_name = vars.name ?? "";
            instance_icon = vars.image ?? "";
            instance_description = vars.desc ?? "";
            instance_working_dir = vars.working_dir ?? "";
            original_working_dir = vars.working_dir ?? "";
        } else {
            emptyVars();
        }
    });
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

    <div class="field label prefix border">
        <a onclick={async () => instance_working_dir = await pickFile([], "Folder", true)}> <i>attach_file</i> </a>
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
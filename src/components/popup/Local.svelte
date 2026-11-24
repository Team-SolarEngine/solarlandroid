<script lang="ts">
    import CardApp from "../CardApp.svelte";

    let { vars = $bindable() } = $props()

    let instance_name = $state("")
    let instance_icon = $state("")
    let instance_description = $state("")
    let instance_working_dir = $state("")
</script>

<div class="overlay" class:active={vars.open} onclick={() => vars.open = false}></div>
<dialog class="bottom" class:active={vars.open}>
    {#if !vars.editing}
        <h4>Adding a new instnace</h4>
    {:else}
        <h4>Editing a instnace</h4>
    {/if}

    <div class="field label border">
        <input type="text">
        <label>Name</label>
        <output>The name of your instance!</output>
    </div>

    <div class="field label border">
        <input type="text">
        <label>Icon (URL)</label>
        <output>The URL path to your instance! We reccomend URL.</output>
    </div>

    <div class="field label border">
        <input type="text">
        <label>Description</label>
        <output>What do you wanna describe your instance? Something cool?</output>
    </div>

    <div class="field label border">
        <input type="text">
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

    <div style="margin-top: 14px;" class="right-align">
        <button><i>add</i> Add</button>
    </div>
</dialog>
<script lang="ts">
    import { store } from "$lib/store";
    import type { Instance } from "$lib/types";
    import { getContext } from "svelte";
    import ModsSection from "./popup/ModsSection.svelte";

    let {
        name = "Funkin'",
        image = "https://duckduckgo.com/i/e39c0e602fe42691.png",
        working_dir = "",
        description = "The funkin' game!",
        preview = false,
    } = $props()

    const openInstancePopup: any = getContext("addInstance");
    let open_mod_contents = $state(false);

    // muahaha I'm evil
    //   for context, since the function to refresh the
    //   instances take about a second, I made a fake 'lil
    //   animation so it looks like it's actually deleting/editing
    //   it instead of just taking a second and then it's gone
    let fake_anim_start = $state(false)
    let fake_anim_text = $state("...")

    async function deleteKey() {
        const instances = (await store.get<Instance[]>("instances")) ?? [];
        await store.set(
            "instances",
            instances.filter((instance) => instance.wd !== working_dir)
        );
        await store.save();
        fake_anim_start = true
        fake_anim_text = "Deleting..."
    }

    function editInstance() {
        openInstancePopup({
            open: true,
            editing: true,
            name,
            image,
            desc: description,
            working_dir,
        });
    }
</script>

{#if !fake_anim_start}
<article style="display: flex; flex-direction: column;">
    <div style="display: flex; gap: 8px; width: 85vw;">
        <img style="width: 80px;" src={image || "/preview.jpg"} alt={name}/>
        <div style="display: flex; flex-direction: column; justify-content: center; gap: 8px;">
            <span style="font-size: 2rem;">{name}</span>
            {#if description}
                <span style="opacity: 0.5;">{description}</span>
            {/if}
        </div>
    </div>

    {#if !preview}
    <nav class="group connected" style="justify-content: center;">
        <button class="border left-round" onclick={() => open_mod_contents = true}>
            <i>folder</i>
            <span>Mods</span>
        </button>
        <button class="border" onclick={editInstance}>
            <i>edit</i>
            <span>Edit</span>
        </button>
        <button class="border right-round tertiary" onclick={deleteKey}>
            <i>delete</i>
            <span>Delete</span>
        </button>
    </nav>
    {/if}
</article>
{:else}
<article style="display: flex; flex-direction: column; justify-content: center; align-items: center;">
    <div class="shape loading-indicator medium"></div>
    <span>{fake_anim_text}</span>
</article>
{/if}

{#if !preview}
<ModsSection
    working_dir={working_dir}
    open={open_mod_contents}
/>
{/if}
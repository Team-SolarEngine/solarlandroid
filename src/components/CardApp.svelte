<script>
    let {
        name = "Funkin'",
        image = "https://duckduckgo.com/i/e39c0e602fe42691.png",
        working_dir = "",
        description = "The funkin' game!",
        preview = false,
    } = $props()
</script>

<article style="display: flex; flex-direction: column;">
    <div style="display: flex; gap: 8px; width: 85vw;">
        <img style="width: 80px;" src={image} alt={name}/>
        <div style="display: flex; flex-direction: column; gap: 8px;">
            <span style="font-size: 2rem;">{name}</span>
            <span style="opacity: 0.5;">{description}</span>
        </div>
    </div>

    {#if !preview}
    <nav class="group connected" style="justify-content: center;">
        <button class="border left-round">
            <i>folder</i>
            <span>Mods</span>
        </button>
        <button class="border">
            <i>edit</i>
            <span>Edit</span>
        </button>
        <button class="border right-round tertiary">
            <i>delete</i>
            <span>Delete</span>
        </button>
    </nav>
    {/if}
</article>
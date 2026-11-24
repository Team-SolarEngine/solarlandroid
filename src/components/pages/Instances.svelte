<script lang="ts">
    import { onMount } from "svelte";
    import CardApp from "../CardApp.svelte";
    import type { Instance } from "$lib/types";
    import { store } from "$lib/store";

    let instances: Instance[] = $state([]);

    async function loadInstances() {
        instances = (await store.get<Instance[]>("instances")) ?? [];
    }

    onMount(() => {
        loadInstances();
        const interval = setInterval(loadInstances, 1000);
        return () => clearInterval(interval);
    });
</script>

<div>
    <div style="overflow-y: auto; height: 100dvh; padding-bottom: 18rem;">
        {#each instances as i}
            <CardApp
                name={i.name}
                image={i.icon}
                description={i.desc}
                working_dir={i.wd}
            />
        {:else}
            <span>No instances found...</span>
        {/each}
    </div>
</div>
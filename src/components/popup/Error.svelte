<script lang="ts">
    // active: boolean
    // action: string
    // what_happened: string
    // what_should_happen: string
    // error_log: string
    let { evar = $bindable() } = $props()

    function copy_text() {
        navigator.clipboard.writeText(`# SL ERROR - <@&1354116717990903890>
**Action**: ${evar.action}
**What happened**: ${evar.what_happened}
**What should've happened:**: ${evar.what_should_happen}

## Logs
\`\`\`
${evar.error_log}
\`\`\`
`)
    }
</script>

<div class="overlay" class:active={evar.active}></div>
<dialog class:active={evar.active}>
    <h3>An error has accoured!</h3>
    <span>Something went wrong, and we're here to tell you what went wrong! Here's a full detail:</span>

    <h4>Interaction</h4>
    <div style="display: flex; flex-direction: column;">
        <span><b>Action</b>: {evar.action}</span>
        <span><b>What happened?</b>: {evar.what_happened}</span>
        <span><b>What should've happened?</b>: {evar.what_should_happen}</span>
    </div>

    <h4>Logs</h4>
    <code><pre>{evar.error_log}</pre></code>

    <div class="right-align no-space" style="margin-top: 12px;">
        <button onclick={() => copy_text()} class="transparent">
            <i>content_copy</i>
            Copy
        </button>
        <button onclick={() => evar.active = false}>
            <i>close</i>
            Close
        </button>
    </div>
</dialog>
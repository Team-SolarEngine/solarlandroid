<script>
    let {
        sbvar = $bindable()
    } = $props()

    $effect(() => {
        if (sbvar.active) {
            setTimeout(() => {
                sbvar.active = false
            }, 5000)
        }
    })
</script>

<div class="snackbar {sbvar.type}"
    class:top={sbvar.top}
    class:active={sbvar.active}
>{sbvar.text}</div>
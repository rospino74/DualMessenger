<script lang="ts">
    import Portal from "svelte-portal";

    export let isOpen: boolean;
    export let closeable: boolean;

    // Generate a random portal ID
    const portalId = Math.random().toString(36).substring(2, 15);
</script>

<div>
    <Portal>
        <input
            type="checkbox"
            id={`modal-${portalId}`}
            class="modal-toggle"
            bind:checked={isOpen}
        />
        {#if closeable}
            <label for={`modal-${portalId}`} class="modal modal-bottom sm:modal-middle cursor-pointer">
                <label class="modal-box relative" for="">
                    <slot />
                </label>
            </label>
        {:else}
            <div class="modal modal-bottom sm:modal-middle">
                <div class="modal-box">
                    <slot />
                </div>
            </div>
        {/if}
    </Portal>
</div>

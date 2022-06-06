<script lang="ts">
    import { _ } from "svelte-i18n";
    import { selectAPK } from "../logic/selectFile";
    export let file: string  = '';

    let filePromise: Promise<string> | undefined;
    let rejected: boolean = false;

    function openFilePickingDialog() {
        rejected = false;
        filePromise = selectAPK();
        filePromise.then(
            (filePath) => {
                file = filePath.split("/").pop();
            },
            () => {
                rejected = true;
            }
        );
    }
</script>

<div class="w-[80%] flex flex-row gap-2">
    <div
        class="input input-bordered w-full p-2 flex justify-start items-center hover:bg-base-200"
        class:text-error={rejected}
        class:border-error={rejected}
        on:click={openFilePickingDialog}
    >
        <span class="block">
            {#if filePromise}
                {#await filePromise}
                    {$_("waiting_for_file")}
                {:then ignored}
                    {file}
                {:catch ignored}
                    {$_("error_while_selecting")}
                {/await}
            {:else}
                {$_("press_to_select")}
            {/if}
        </span>
    </div>
</div>

<script lang="ts">
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
                    Waiting for file selection...
                {:then _}
                    {file}
                {:catch _}
                    Error while selecting file
                {/await}
            {:else}
                Press here to select an APK
            {/if}
        </span>
    </div>
</div>

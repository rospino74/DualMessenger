<script lang="ts">
    import { selectAPK } from "../logic/selectFile";

    let filePromise: Promise<string> | undefined;
</script>

<div class="w-[80%] flex flex-row gap-2">
    <div
        class="input input-bordered w-full p-2 flex justify-start items-center hover:bg-base-200"
        on:click={() => {
            filePromise = selectAPK();
        }}
    >
        <span class="block">
            {#if filePromise}
                {#await filePromise}
                    Waiting for file selection...
                {:then filePath}
                    {filePath.split("/").pop()}
                {:catch _}
                    Error while selecting file
                {/await}
            {:else}
                Press here to select an APK
            {/if}
        </span>
    </div>
</div>

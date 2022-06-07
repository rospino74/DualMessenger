<script>
    import { invoke } from "@tauri-apps/api/tauri";

    const promises = [
        [invoke("get_sys_version"), "OS Version"],
        [invoke("get_locale"), "System locale"],
        [invoke("is_adb_installed"), "ADB installed"],
    ];
</script>

<div class="prose">
    <h1>System info</h1>
    <ul>
        {#each promises as promise}
            <li class="item">
                {promise[1]}
                {#await promise[0]}
                    <span class="waiting">Waiting...</span>
                {:then result}
                    <span class="value">{result}</span>
                {/await}
            </li>
        {/each}
    </ul>
    <h1>ADB Info</h1>
    <ul>
        {#await invoke("get_adb_devices")}
            <li class="item">
                Devices <span class="waiting">Waiting...</span>
            </li>
        {:then result}
            {#if result.length === 0}
                <li class="item">
                    <span class="value error">No devices found</span>
                </li>
            {:else}
                <li class="item">
                    Devices <code>{JSON.stringify(result, null, 2)}</code>
                </li>
                <li class="item">
                    Users
                    {#await invoke("get_adb_users", { device: result[0] })}
                        <span class="waiting">Waiting...</span>
                    {:then result2}
                        <code>{JSON.stringify(result2, null, 2)}</code>
                    {:catch error}
                        <span class="value error">{error}</span>
                    {/await}
                </li>
            {/if}
        {:catch error}
            <li class="item">
                Devices <span class="value error">{error}</span>
            </li>
        {/await}
    </ul>
</div>

<style lang="scss">
    .item {
        @apply list-none;

        .waiting {
            @apply text-neutral italic;
        }

        .value {
            @apply text-primary font-bold;

            &.error {
                @apply text-error;
            }
        }
    }
</style>

<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import Error from "../components/Error.svelte";
    import LoadingIcon from "../components/LoadingIcon.svelte";
    import { onDestroy } from "svelte";

    // Pool for devices every 5 seconds
    let devicePromise = invoke("get_adb_devices");
    let timer;
    devicePromise.then(
        () => {
            timer = setTimeout(() => {
                devicePromise = invoke("get_adb_devices");
            }, 5000);
        },
        () => {
            clearTimeout(timer);
        }
    );
    onDestroy(() => {
        clearTimeout(timer);
    });
</script>

{#await devicePromise}
    <div>
        Carico i dispositivi...
        <LoadingIcon />
    </div>
{:then devices}
    <div class="picker">
        <h1>Seleziona il dispositivo</h1>
        <ul>
            {#each devices as device}
                <li class="device">
                    <a href={device.url}>{device.name}</a>
                </li>
            {/each}
        </ul>
    </div>
{:catch error}
    <Error>
        {error}
    </Error>
{/await}

<style lang="scss">
    .picker {
        @apply prose;

        .device {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0.5rem;
            border-bottom: 1px solid #ccc;
        }
    }
</style>

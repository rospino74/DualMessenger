<script>
import { invoke } from '@tauri-apps/api/tauri'

const promises = [
    [invoke("get_sys_version"), "OS Version"],
    [invoke("get_locale"), "System locale"],
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
</div>

<style lang="scss">
    .item {
        @apply list-none;

        .waiting {
            @apply text-neutral italic;
        }

        .value {
            @apply text-primary font-bold;
        }
    }
</style>
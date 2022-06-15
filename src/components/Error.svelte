<script>
    import { _ } from "svelte-i18n";
    import { LifeBuoyIcon } from "svelte-feather-icons";
</script>

<div class="box">
    <LifeBuoyIcon class="error-icon" />
    <h1>{$_("error_funny")}</h1>
    <p>
        <slot />
    </p>
</div>

<style lang="scss">
    .box {
        @apply flex flex-col justify-center items-center w-full h-full;

        :global(.error-icon) {
            @apply block stroke-error w-32 h-32 mb-6;
        }

        h1 {
            @apply font-bold text-4xl text-error-content mb-4;
        }

        p {
            @apply text-2xl italic text-error-content opacity-80;
        }
    }
</style>

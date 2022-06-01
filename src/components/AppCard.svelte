<script>
    import { Trash2Icon } from "svelte-feather-icons";
    import { createEventDispatcher } from "svelte";

    // Proprietà
    export let name, javaPackage, version, icon;

    // Bottone e  correlati
    let deleteButton;
    let cardWidth, deleteButtonOffsetLeft;

    // Calcolo il margine del bottone
    // $: deleteButtonMargin = (cardWidth - deleteButtonOffsetLeft) / cardWidth * 100;

    // onMount(() => {
    //     // Calcolo l'offset del bottone
    //     deleteButtonOffsetLeft = deleteButton.offsetLeft;
    //     hideButton();
    // });

    // Event Dispatcher
    const dispatch = createEventDispatcher();

    // Event Handlers per animazioni del bottone
    const showButton = () => {
        // deleteButton.style.marginRight = "0";
    };
    const hideButton = () => {
        // const { width: buttonWidth } = deleteButton.getBoundingClientRect();
        // deleteButton.style.marginRight = `-${buttonWidth / 16 + 0.8}rem`;
        // deleteButton.style.marginRight = `-${deleteButtonMargin}%`;
    };
</script>

<div class="card w-full bg-base-100 shadow-xl" bind:offsetWidth={cardWidth}>
    <div
        class="content card-body text-base-content"
        on:mouseenter={showButton}
        on:mouseleave={hideButton}
    >
        <!-- Con ultimo evento mostro il bottone appena entro con il mouse -->
        <img src={icon} alt="{name}'s icon" class="h-8 w-8 md:h-16 md:w-16" />
        <div class="identifier">
            <span class="font-bold text-lg md:text-3xl mr-1">{name}</span>
            <span class="font-light text-base md:text-lg">{javaPackage}</span>
        </div>
        <div class="font-light text-lg">
            v<span class="font-bold text-primary">{version}</span>
        </div>
        <div class="action" bind:this={deleteButton}>
            <button
                class="btn btn-outline btn-primary gap-2"
                on:click={() => dispatch("uninstall")}
            >
                <Trash2Icon size="20" class="h-6 w-6" />
                Delete
            </button>
        </div>
    </div>
</div>

<style lang="scss">
    .content {
        @apply flex flex-row items-center justify-start;
        box-sizing: border-box;
        position: relative;

        & > * {
            @apply block m-0 mr-1 md:mr-5 last:mr-0;
        }

        .identifier {
            @apply mr-auto md:flex-col md:flex md:items-start;
        }

        .action {
            transition: margin-right 250ms ease-in-out, display 0s;
        }
    }
</style>

<div class="bar">
    <div class="thumb"></div>
</div>

<style lang="scss">
    @keyframes moving {
        0% {
            transform: translateX(-100%);
        }

        100% {
            transform: translateX(100%);
        }
    }

    @keyframes sizing {
        0% {
            transform: scaleX(0%);
        }

        100% {
            transform: scaleX(100%);
        }
    }

    $speed: 2s;
    .bar {
        @apply w-full h-full bg-base-200 rounded overflow-hidden;

        .thumb {
            animation: moving ease-in-out $speed infinite;
            @apply h-full w-full;

            
            &::before {
                animation: sizing calc($speed / 2) infinite alternate;
                content: '';
                @apply rounded bg-primary block h-full w-full;
            }
        }
    }
</style>

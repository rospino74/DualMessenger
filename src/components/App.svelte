<script lang="ts">
    // @ts-ignore
    import logo from "../resources/logo.png";

    export let name: string;
</script>

<main>
    <img class="logo" src={logo} alt="Svelte Logo" />
    <h1>Hello {name}!</h1>
</main>

<style lang="scss">
    :root {
        --color: hsl(171, 100%, 91%);
    }

    main {
        text-align: center;
        padding: 1em;
        margin: 0 auto;

        .logo {
            height: 16rem;
            width: 16rem;
            filter: grayscale(100%) sepia(100%) hue-rotate(90deg);
        }

        h1 {
            color: var(--color);
            text-transform: uppercase;
            font-size: 4rem;
            font-weight: 100;
            line-height: 1.1;
            margin: 2rem auto;
            max-width: 14rem;
        
            @media (min-width: 480px) {
                max-width: none;
            }
        }
    }

</style>

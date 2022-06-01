<script>
  import { DownloadIcon, PlusIcon, XIcon } from "svelte-feather-icons";
  import AppCard from "../components/AppCard.svelte";
  import LoadingIcon from "../components/LoadingIcon.svelte";
  import Modal from "../components/Modal.svelte";
  import APKPicker from "../components/APKPicker.svelte";

  const apps = [
    {
      name: "Attractions",
      javaPackage: "com.attractions",
      version: "1.0.0",
      icon: "https://illright.github.io/attractions/logo.svg",
    },
    {
      name: "Electron",
      javaPackage: "io.electron",
      version: "3.0.0",
      icon: "https://upload.wikimedia.org/wikipedia/commons/9/91/Electron_Software_Framework_Logo.svg",
    },
    {
      name: "Tauri",
      javaPackage: "studio.tauri",
      version: "1.7.0",
      icon: "https://github.com/tauri-apps/tauri/raw/dev/app-icon.png",
    },
  ];

  // Modal
  let isLoadingModalOpen = false;
  let isUploadModalOpen = true;
  let isUninstalling = false;

  function uninstall(e) {
    isLoadingModalOpen = true;
    isUninstalling = true;
  }
</script>

<h1 class="prose-h1">List of installed apps</h1>
<main>
  {#each apps as app (app.name)}
    <div>
      <AppCard {...app} on:uninstall={uninstall} />
    </div>
  {/each}
</main>

<Modal isOpen={isLoadingModalOpen}>
  <h3 class="font-bold text-3xl text-center">
    {#if isUninstalling}
      Disinstallazione
    {:else}
      Installazione
    {/if}
    in corso
  </h3>
  <p class="py-4 text-center">
    Attendi mentre l'applicazione viene {#if isUninstalling}disinstallata{:else}installata{/if}&hellip;
    <br />
    Puoi iniziare già a pregustarti il risultato.
  </p>
  <div class="m-auto mt-4 h-2 w-full">
    <LoadingIcon />
  </div>
</Modal>

<Modal bind:isOpen={isUploadModalOpen} closeable>
  <h3 class="font-bold text-3xl text-center">Pick an APK</h3>
  <p class="py-4 text-center flex justify-center items-center flex-col">
    <span class="block mb-2">Please select the APK of the application you want to install on the device.</span>
    <APKPicker />
  </p>
  <div class="modal-action">
    <button class="btn btn-primary btn-outline gap-2">
      <DownloadIcon size="20" />
      Installa
    </button>
    <button
      class="btn btn-error btn-outline gap-2"
      on:click={() => (isUploadModalOpen = false)}
    >
      <XIcon size="20" />
      Chiudi
    </button>
  </div>
</Modal>

<button
  class="fixed right-0 bottom-0 m-4 btn btn-square"
  on:click={() => (isUploadModalOpen = true)}
>
  <PlusIcon size="20" />
</button>

<style lang="scss">
  main {
    padding: 1rem;

    & > div {
      margin-bottom: 1rem;
    }
  }
</style>

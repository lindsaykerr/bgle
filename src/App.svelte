<script lang="ts">
  
  import OpenDir from './lib/OpenDir.svelte';

  import EmulatorList from './lib/EmulatorList.svelte';

</script>

<main class="container">
  <h1>BGLE</h1>
  <OpenDir />
  <EmulatorList />

</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
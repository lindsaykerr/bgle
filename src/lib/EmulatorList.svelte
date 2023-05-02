<script lang="ts">
    import "../styles/components/emulator-list.css";

    import EmulatorListItem from "./EmulatorListItem.svelte";
    import { emulatorListStore } from "./store";
    import type { Emulator } from "./types";

    let emulatorList:Emulator[] | undefined;
    let state_change = false; 

    emulatorListStore.subscribe((value) => {
        if (state_change === false && JSON.stringify(value) === "[]") {
            state_change = false;
        }
        else {
            state_change = true;
        }
        emulatorList = value;
        
    });
    let selectionId: string | undefined;


</script>
<div class="emulators container">
    {#if state_change === true}
        <div class="spacer"></div> 

        {#if emulatorList.length === 0}
            <p class="alert">No emulators found</p>
        {:else}
            <div class="list">
            {#each emulatorList as emulator, i (emulator.name)}
                <EmulatorListItem id={`em-${i}`} bind:activeId={selectionId} {emulator} />    
            {/each}
            </div>
        {/if}
    {/if}
</div>
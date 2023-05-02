<script lang='ts'>
    import "../styles/components/open-directory.css"
    import { open } from '@tauri-apps/api/dialog';
    import { invoke } from "@tauri-apps/api/tauri";
    import { emulatorListStore } from './store';
    import type { Emulators } from './types';

    let directoryPath: string | string[] | null = "";

    async function openDirectory() {
        directoryPath = await open({
            title: 'Open a directory',
            directory: true,
        });
       

        if (typeof directoryPath === 'string') {
            // Call backend to get emulator list and update store
            await invoke("get_emulator_list",{path: directoryPath})
            .then((list: Emulators) => {
                emulatorListStore.set(list.emulators);
            }).catch((err:any) => {
                emulatorListStore.set([]);
            });
        }
    }
</script>

<div class="open-directory container">
    <div>
        <button on:click={openDirectory}>Open</button> path to Roms directory
    </div>
    
    {#if directoryPath}
        <p id="path-listing">Opened at: {directoryPath}</p>
    {:else}
        <p id="path-listing">No directory opened</p>
    {/if}
    
</div>

<style>



</style>
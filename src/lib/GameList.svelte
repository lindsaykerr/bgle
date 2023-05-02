<script lang="ts">

    import "../styles/components/game-list.css";
    import GameListItem from "./GameListItem.svelte";

    import {invoke} from "@tauri-apps/api/tauri";
    import type { Games } from "./types";
    import { gameListStore } from "./store";
 
    export let directoryPath: string | undefined;

    let activeId = "";


    
 

    async function retrieve_game_list(): Promise<Games> {
        const promise = new Promise((resolve, reject) =>{
            invoke("get_game_list", {valid_dir: directoryPath}).then((value: Games) => {
                resolve(value);
            }).catch((e) => {
                reject(e);
            });
        });
        return promise as Promise<Games>;

    }

    let promise = retrieve_game_list();
    promise.then((value) => {
        console.log("Gamelist:", value)
        gameListStore.set(value);
    });




</script>
<div class="gamelist">
    {#await promise}
        <p class="alert">Loading...</p>
    {:then gameList}
        {#if gameList.games.length === 0}
            <p class="alert">No games found</p>
        {:else}
            <div class="container .gamelist-items">
            {#each gameList.games as game, i (game.id)}
                <GameListItem directory={gameList.directory} index={i} game={game} bind:selectedId={activeId} />
            {/each}
            </div>
        {/if}
    {/await}
</div>
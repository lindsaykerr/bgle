<script lang="ts">

    import "../styles/components/game-form.css";
    import type { Game }  from "./types";
    import { gameListStore } from "./store";
    import { invoke } from "@tauri-apps/api/tauri";
    import {confirm} from "@tauri-apps/api/dialog";
    import Field from "./Field.svelte";

    export let gameItem: Game;
    export let index: number;
    
    let fields = gameItem.fields;

    async function save() {
        const gameList = $gameListStore;
        gameList.games[index].fields = fields;
        let confirmation = await confirm(
            "Are you sure you want to save this game?", 
            {title: "Saving to gamefile.xml", type: "warning"}
            );
            
        if (confirmation) {  
            invoke("save_game_list", {game_list: gameList}).then((value) => {
                console.log("Save was a success:", value);
            }).catch((e) => {
                console.log(e);
            });
        }

    }
    function setFieldType(field: string): string {
        switch (field) {
            case "File":
                return "file";
            case "Integer":
            case "Float":
                return "number";
            case "Boolean":
                return "checkbox";
            case "LineText":
                return "text";
            case "MultilineText":
                return "textarea";
            case "Date":
                return "date";
            case "Range":
                return "range";
            default:
                return "text";
        }
    }
</script>

<form class="game-form">
    {#each fields as field, i (field.name)}
        <Field type={setFieldType(field.field_type)} gameIndex={index} fieldIndex={i} />
    {/each}
    <div class="form-button">
        <button type="submit" on:click|preventDefault={save}>Save</button>
    </div>
</form>
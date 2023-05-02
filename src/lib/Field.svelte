<script lang=ts>
    import "../styles/components/field.css";
    import { gameListStore } from "./store";
    import {open} from "@tauri-apps/api/dialog";
    export let fieldIndex: number;
    export let gameIndex: number;
    export let type: string;
    let gameList = $gameListStore;
    
    let value = gameList.games[gameIndex].fields[fieldIndex].value;
    let name = gameList.games[gameIndex].fields[fieldIndex].name;
    let disabled = !gameList.games[gameIndex].fields[fieldIndex].editable;
    let field_type = gameList.games[gameIndex].fields[fieldIndex].field_type;
    let label = name;
    
    label = label[0].toUpperCase() + label.slice(1);
    switch(label) {
        case "Path":
            label = "Rom Path";
        break;
        case "Md5":
            label = "MD5 Checksum";
        default:
            break;
    }
    
    let reletivePath = value;
    if (type === "file") {
        value = gameList.directory + value.slice(1)
    }

    let classes = {
        "error" : {
            state: false,
            message: "",
        },

    }


    function assignInputValue() {
        value = String(value);
        gameList.games[gameIndex].fields[fieldIndex].value = value;
        gameListStore.set(gameList);
    }

    async function assignInputValuefilePath() {
        let chosenEmulatorPath: string | string[] | null = "";

        chosenEmulatorPath = await open({
            defaultPath: gameList.directory,
            title: 'Open file',
        });
        if (chosenEmulatorPath === null || Array.isArray(chosenEmulatorPath)) {
            
            return;
        }
    
        // test that the emulator path is the same as the chosen path
      
      
        if (chosenEmulatorPath.slice(0, gameList.directory.length) !== gameList.directory) {
            classes.error.state = true;
            classes.error.message = "Error: File must be in the same directory as the emulator";
            
            return;
        } else {
            classes.error.state = false;
            classes.error.message = "";
        }

        let filePath = chosenEmulatorPath.slice(gameList.directory.length+1);
        
        reletivePath = './' + filePath;
        gameList.games[gameIndex].fields[fieldIndex].value = reletivePath;
        gameListStore.set(gameList);
    }

    function buildClasses() {
        let classString = "";
        for (const [key, value] of Object.entries(classes)) {
            if (value.state) {
                classString += key + " ";
            }
        }
        return classString;
    }
    function assignWidthClass(field_name: string): string {
        switch (field_name) {
            case "path":
                return "half";
            case "name":
                return "two-thirds";
            case "description":
                return "full";
            case "rating":
                return "eighth";
            case "releaseDate":
                return "quarter";
            case "developer":
                return "full";
            case "Publisher":
                return "full";
            case "genre":
                return "one-third";
            case "players":
                return "one-third";
            case "md5":
                return "two-thirds";
            case "region":
                return "eighth";
            case "language":
                return "eighth";
            case "date":
                return "one-third";
            case "lang":
                return "eighth";
        }
    }


    
</script>
<div class="field">
{#if type === "text"} 
    <label for={name}>{label}</label>:<br/><input class="{assignWidthClass(name)} {field_type.toLocaleLowerCase()}" {name} type="text" bind:value={value} {disabled} on:input={assignInputValue} />
{:else if type === "file"} 
    <div><label for={name}>{label}</label>:<br/><input class="{field_type.toLocaleLowerCase()}" {name} type="button" value="Open" {disabled} on:click={assignInputValuefilePath} /> <span class="small-text">{reletivePath}</span></div>
    <div class="error-message small-text">{classes.error.message}</div>
{:else if type === "range"}
    <label for={name}>{label}</label>:<br><input class="{assignWidthClass(name)} {field_type.toLocaleLowerCase()}" {name} type="range" bind:value={value} min="1" max="4" step="1" on:input={assignInputValue} /><span>{value}</span>
{:else if type === "number"}
    {#if name === "rating"}

    <label for={name}>{label}</label>: <input class="{assignWidthClass(name)} {field_type.toLocaleLowerCase()}" {name} type="number" min="0" max="1" step=".1" bind:value={value} {disabled} on:input={assignInputValue} />
    {:else}
    <label for={name}>{label}</label>:<br/><input class="{assignWidthClass(name)} {field_type.toLocaleLowerCase()}" {name} type="number" bind:value={value} {disabled} on:input={assignInputValue} />
    {/if}
{:else if type === "date"}
    <label for={name}>{label}</label>: <input class="{assignWidthClass(name)} {field_type.toLocaleLowerCase()}" {name} type="date" bind:value={value} {disabled} on:input={assignInputValue} />
{:else if type === "textarea"}
    <label for={name}>{label}</label>:<br/><textarea class="full {field_type.toLocaleLowerCase()}" {name} bind:value={value} {disabled} on:input={assignInputValue} />
{:else if type === "float"}

{/if}
</div>  



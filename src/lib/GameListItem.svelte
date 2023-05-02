
<script lang="ts">
    import "../styles/components/game-list-item.css";
    import GameForm from "./GameForm.svelte";
    import type { Game } from './types';
    export let game: Game | undefined;
    export let selectedId: string | "" = "";
    export let index: number;
    export let directory: string | undefined;
    
    interface Color {
        r: number;
        g: number;
        b: number;
    }

    /*
    const startColor:   Color = { r: 223, g: 80, b: 80 };
    const endColor:     Color = { r: 76, g: 255, b: 22 };

    let color: Color = startColor

    
    function colorGradientFromPercentage(percent: number) {
        const calc = (color: string, percent: number) => {
            return Math.round(((endColor[color] - startColor[color]) * (percent / 100)) + startColor[color]);
        }
        const r = calc('r', percent);
        const g = calc('g', percent);
        const b = calc('b', percent);
        color = { r, g, b };
    }
    $: colorGradientFromPercentage(game.completePercent);
 */

    let name: string;

    const nameResult = game.fields.filter(field => field.name === "name");
    if (nameResult.length > 0) {
        name = nameResult[0].value;
    }
    else {
        name = game.fields.filter(field => field.name === "path")[0].value;
    }

    let active: String = "";

    function onSelect() {
        selectedId = game.id;
        checkState();
    }
    function checkState() {
        const toggleActiveClass = () => active = (active === "active") ? "" : "active"
        toggleActiveClass();
    }

    $: active = (selectedId !== game.id) ? "": "active";
   



 
    
</script>
<div class="game-list-item" >
<button class="btn" on:click={onSelect}>
    <div class="container">
    <div class="name">
        {name}
    </div>
    </div>
</button>
{#if active === "active"}

    <GameForm {index} gameItem={game}  />
{/if}
</div>



<script lang="ts">
    import "../styles/components/emulator-list-item.css";
    import type { Emulator } from './types';
    import GameList from "./GameList.svelte";

    export let emulator: Emulator | undefined;
    export let activeId: string | "" = "";
    export let id = "";
    let toggle: boolean = false;
    let active: String = "";


 
    interface Color {
        r: number;
        g: number;
        b: number;
    }

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
   

    function onSelect() {
		activeId = emulator.name;
		checkState();
        
	}


	
	function checkState() {
		const toggleActiveClass = () => active = (active === "active") ? "" : "active"
		const changeToggle = () => toggle = (toggle) ? false : true;

		toggleActiveClass();
		changeToggle();
	}

	$: colorGradientFromPercentage(emulator.complete_percent);
	$: active = (activeId !== emulator.name) ? "": "active";
    
    
    

    
</script>

<div id={id} class="{active === 'active' ? 'emulator active': 'emulator'} "  style="border-color: rgb({color.r}, {color.g}, {color.b}">
    <button on:click={onSelect} >
        <div class="container">
        <div class="name">{emulator.name}</div>
        <div class="game-count">
            {#if emulator.game_count === 0}
                No games
            {:else if emulator.game_count === 1}
                <span class="number">1</span> game
            {:else}
            <span class="number">{emulator.game_count}</span> games
            {/if}
        
        </div>
        <div class="complete-percent" style="color: rgb({color.r}, {color.g}, {color.b}">
            <span class="value">{(Math.round(emulator.complete_percent * 100) / 100).toFixed(1)}
                <span class="sign">&percnt;</span>
            </span>
        </div>
        </div>
        
    </button>
    {#if active === "active" }
    <div class="spacer"></div>
  
    <GameList directoryPath={emulator.directory}  />
    
    {:else}
        <div></div>
    {/if}
</div>


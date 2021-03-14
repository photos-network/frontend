<div class="search" class:opened>
	<button class="icon-btn" title="Search" on:click="{open}" bind:this="{button}">
		<Icon name="search"/>
	</button>
	{#if opened}
		<input class="search-input" type="search"
			transition:fade
			on:blur="{close}"
			on:keydown|stopPropagation="{onkeydown}"
			bind:this="{input}"
			bind:value="{$query}">
	{/if}
</div>

<script>
import { tick } from 'svelte';
import { fade } from 'svelte/transition';
import Icon from '../icon';
import { query } from '../lib';

let button;
let input;
let opened = false;

function open () {
	opened = true;
	tick().then(() => input.select());
}

function close () {
	if (!$query) {
		opened = false;
		tick().then(() => button.focus());
	}
}

function onkeydown (e) {
	const key = e.key;
	if (key === 'Escape') {
		if ($query) $query = '';
		else close();
	}
}

</script>

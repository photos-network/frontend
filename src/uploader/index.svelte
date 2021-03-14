<input class="uploader-input" type="file"
	accept="image/jpeg,image/png,video/mpeg,video/mov"
	multiple
	on:change="{onchange}"
	bind:this="{inputElement}">

{#if showCover}
	<div class="uploader-cover"
		transition:fade="{{ duration: 100 }}"
		on:dragleave|preventDefault|capture="{() => showCover = false}">
			<Icon name="upload"/>
	</div>
{/if}

{#if showUploadPanel}
	<div class="uploader-panel">
		{#each uploadingFiles as file (file.id)}
			<UploadItem item="{file}" on:cancel="{cancelUpload}"/>
		{/each}
	</div>
{/if}

<svelte:window
	on:drop|preventDefault|capture="{ondrop}"
	on:dragover|preventDefault|capture=""
	on:dragenter|preventDefault|capture="{() => showCover = true}"

/>
<script>
import { onMount } from 'svelte';
import { fade } from 'svelte/transition';
import { EVENT, guid, isVideoType } from '../lib';

import Icon from '../icon';
import UploadItem from './upload-item';

let uploadingFiles = [];
let inputElement;
let showCover = false;
let showUploadPanel = false;

onMount(() => {
	EVENT.on( EVENT.uploader.browse, () => inputElement.click());
});

function ondrop (e) {
	showCover = false;
	read(e.dataTransfer?.files);
}

function onchange ({target}) {
	// file.name = decodeURI(encodeURI(target.value).replace(':%5Cfakepath%5C', '')); // chrome?
	read(target.files);
}

function read (files) {
	if (!files || !files.length) return;
	showUploadPanel = true;
	for (const file of files) {
		file.id = guid();
		updateList(file);
		const reader = new FileReader();
		reader.onload = ev => onload(file, ev.target?.result);
		if (isVideoType(file.type)) reader.readAsBinaryString(file);
		else reader.readAsDataURL(file);
	}
}

function onload (file, thumb) {
	file.thumb = thumb;
	updateList(file);
	// UPLOAD THE FILE
	console.log(file);
}

function cancelUpload (e) {
	const item = e.detail;
	uploadingFiles = uploadingFiles.filter(i => i.id !== item.id);
	if (!uploadingFiles.length) showUploadPanel = false;
}


function updateList (file) {
	const idx = uploadingFiles.findIndex(i => i.id === file.id);
	if (idx === -1) uploadingFiles.push(file);
	else Object.assign(uploadingFiles[idx], file);
	// react to changes
	uploadingFiles = uploadingFiles;
}

</script>

<div class="uploader-item" transition:fly="{{ duration: 200, x: 200 }}">
	<img class="uploader-item-thumb" src="{thumb}" alt="{item.name || ''}">
	<span class="uploader-item-name"><em>{item.name}</em><br>{formatBytes(item.size || 0)}</span>
	<button class="icon-btn" on:click="{cancelUpload}" title="Cancel upload"><Icon name="close" /></button>
	<Progressbar progress="{items.progress || 0}" />
</div>

<script>
import { createEventDispatcher } from 'svelte';
import { fly } from 'svelte/transition';
import { formatBytes, isVideoType, items } from '../lib';
import photo from '../../assets/photo.svg';
import video from '../../assets/video.svg';

import Icon from '../icon';
import Progressbar from './upload-item-progress';

const dispatch = createEventDispatcher();
const imgThumb = 'data:image/svg+xml;utf8,' + photo.replace('currentColor', 'white');
const videoThumb = 'data:image/svg+xml;utf8,' + video.replace('currentColor', 'white');
export let item = { name: '', size: 0, progress: 0, thumb: imgThumb, };

$:thumb = isVideoType(item.type) ? videoThumb : item.thumb || imgThumb;

function cancelUpload () {
	dispatch('cancel', item);
}

</script>

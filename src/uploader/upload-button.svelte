<a class="icon-btn upload-button" title="Upload" href="#upload"
	on:click|preventDefault="{() => EVENT.fire(EVENT.uploader.browse)}">
	<Icon name="upload" />
</a>

<script>
import { EVENT } from '../lib';
import Icon from '../icon';
</script>

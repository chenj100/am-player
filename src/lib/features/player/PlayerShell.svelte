<script lang="ts">
  import { playbackQueue } from '$lib/stores/player';
</script>

<section class="page">
  <h1>Player</h1>
  {#if $playbackQueue}
    <p>Queue size: {$playbackQueue.itemIds.length}</p>
  {:else}
    <p>No local playback queue selected.</p>
  {/if}
</section>

<script lang="ts">
  import { libraryItems } from '$lib/stores/library';
</script>

<section class="page">
  <h1>Library</h1>
  <p>Browse normalized metadata, duplicate groups, and playback availability.</p>

  <ul>
    {#each $libraryItems as item}
      <li>
        <strong>{item.normalizedSongTitle ?? item.originalSourceTitle}</strong>
        <div>{item.normalizedSinger ?? 'Unknown singer'} · {item.normalizedAlbum ?? 'Unknown album'}</div>
      </li>
    {/each}
  </ul>
</section>

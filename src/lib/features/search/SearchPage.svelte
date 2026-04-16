<script lang="ts">
  import StatusBanner from '$lib/components/StatusBanner.svelte';
  import { createSaveJob, defaultDestinations, searchTracks } from '$lib/api/commands';
  import { jobs, upsertJob } from '$lib/stores/jobs';
  import { uiState } from '$lib/stores/ui';
  import type { StorageDestination, TrackCandidate } from '$lib/types/models';

  let query = '';
  let sourceFilter: 'all' | 'youtube' | 'bilibili' = 'all';
  let candidates: TrackCandidate[] = [];
  let warnings: string[] = [];
  let selectedDestination = 'shared_local';
  const destinations: StorageDestination[] = defaultDestinations();
  $: currentJobs = $jobs;

  async function runSearch() {
    uiState.set({ loading: true, error: null, emptyMessage: null, retryable: false, backupOnlyMessage: null });
    try {
      const result = await searchTracks(query, sourceFilter);
      candidates = result.results;
      warnings = result.warnings;
      uiState.update((state) => ({
        ...state,
        loading: false,
        emptyMessage: result.results.length === 0 ? 'No matches found for this query.' : null
      }));
    } catch (error) {
      uiState.set({
        loading: false,
        error: error instanceof Error ? error.message : 'Search failed.',
        emptyMessage: null,
        retryable: true,
        backupOnlyMessage: null
      });
    }
  }

  async function saveCandidate(candidateId: string) {
    const response = await createSaveJob(candidateId, selectedDestination);
    upsertJob({
      jobId: response.jobId,
      jobType: selectedDestination === 'google_drive' ? 'drive_export' : 'local_save',
      sourcePlatform: candidateId.startsWith('yt') ? 'youtube' : 'bilibili',
      sourceItemId: candidateId,
      targetLibraryItemId: response.existingLibraryItemId ?? null,
      requestedDestination: selectedDestination,
      status: 'queued',
      progressBytes: 0,
      totalBytes: null,
      failureCode: null,
      failureMessage: response.duplicateWarning ? 'Possible duplicate detected.' : null,
      retryCount: 0
    });
  }
</script>

<section class="page">
  <header>
    <h1>Search and Save</h1>
    <p>Find tracks, choose a destination, and queue local saves or Drive backups.</p>
  </header>

  <div class="controls">
    <input bind:value={query} placeholder="Search by song, singer, album, or free text" />
    <select bind:value={sourceFilter}>
      <option value="all">All sources</option>
      <option value="bilibili">Bilibili</option>
      <option value="youtube">YouTube</option>
    </select>
    <select bind:value={selectedDestination}>
      {#each destinations as destination}
        <option value={destination.destinationId}>{destination.displayName}</option>
      {/each}
    </select>
    <button on:click={runSearch}>Search</button>
  </div>

  <StatusBanner message={$uiState.error ?? $uiState.emptyMessage} tone={$uiState.error ? 'error' : 'info'} />

  {#if warnings.length > 0}
    <StatusBanner message={warnings.join(' ')} tone="info" />
  {/if}

  <ul class="results">
    {#each candidates as candidate}
      <li>
        <div>
          <strong>{candidate.sourceTitle}</strong>
          <div>{candidate.uploaderName} · {candidate.sourcePlatform}</div>
        </div>
        <button on:click={() => saveCandidate(candidate.candidateId)}>Save</button>
      </li>
    {/each}
  </ul>

  <section>
    <h2>Queued jobs</h2>
    <ul>
      {#each Object.values(currentJobs) as job}
        <li>{job.jobId} · {job.status} · {job.requestedDestination}</li>
      {/each}
    </ul>
  </section>
</section>

<style>
  .page { padding: 1.25rem; max-width: 56rem; margin: 0 auto; }
  .controls { display: grid; gap: 0.75rem; grid-template-columns: 1fr; margin: 1rem 0; }
  .results { list-style: none; padding: 0; display: grid; gap: 0.75rem; }
  li { display: flex; justify-content: space-between; gap: 1rem; border: 1px solid #d9e1ea; padding: 0.9rem; border-radius: 0.85rem; }
  input, select, button { font: inherit; padding: 0.75rem; border-radius: 0.75rem; }
  input, select { border: 1px solid #bfc9d4; }
  button { border: none; background: #124734; color: #fff; }
</style>

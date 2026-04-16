<script lang="ts">
  export let message: string | null = null;
  export let tone: 'info' | 'error' | 'success' = 'info';
</script>

{#if message}
  <div class={`status status-${tone}`}>{message}</div>
{/if}

<style>
  .status {
    padding: 0.75rem 1rem;
    border-radius: 0.75rem;
    margin: 0.5rem 0;
    font-size: 0.95rem;
  }
  .status-info { background: #eef5ff; color: #123a73; }
  .status-error { background: #fdeeee; color: #7c1d1d; }
  .status-success { background: #edf8ef; color: #1a5c27; }
</style>

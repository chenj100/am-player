import { writable } from 'svelte/store';

export const uiState = writable({
  loading: false,
  error: null as string | null,
  emptyMessage: 'Search to start building your library.' as string | null,
  retryable: false,
  backupOnlyMessage: null as string | null
});

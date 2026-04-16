import { invoke } from '@tauri-apps/api/core';
import type { DownloadJob, LibraryItem, StorageDestination, TrackCandidate } from '$lib/types/models';

export async function searchTracks(query: string, sourceFilter: 'all' | 'youtube' | 'bilibili') {
  return invoke<{ results: TrackCandidate[]; warnings: string[] }>('search_tracks', {
    query,
    sourceFilter
  });
}

export async function createSaveJob(candidateId: string, destinationId: string) {
  return invoke<{ jobId: string; duplicateWarning: boolean; existingLibraryItemId?: string }>(
    'create_save_job',
    { candidateId, destinationId }
  );
}

export async function getJobStatus(jobId: string) {
  return invoke<{ job: DownloadJob }>('get_job_status', { jobId });
}

export async function listLibrary() {
  return invoke<{ items: LibraryItem[] }>('list_library');
}

export function defaultDestinations(): StorageDestination[] {
  return [
    {
      destinationId: 'shared_local',
      destinationType: 'shared_local',
      displayName: 'Device storage',
      writable: true,
      requiresAuth: false,
      capacityHintBytes: null,
      availabilityState: 'ready'
    },
    {
      destinationId: 'google_drive',
      destinationType: 'google_drive',
      displayName: 'Google Drive backup',
      writable: true,
      requiresAuth: true,
      capacityHintBytes: null,
      availabilityState: 'auth_required'
    }
  ];
}

export type SourcePlatform = 'youtube' | 'bilibili';
export type AvailabilityState = 'available' | 'unavailable' | 'restricted' | 'unknown';
export type JobStatus =
  | 'queued'
  | 'running'
  | 'paused'
  | 'retry_waiting'
  | 'completed'
  | 'failed'
  | 'cancelled';
export type DriveBackupState = 'not_exported' | 'queued' | 'exporting' | 'exported' | 'failed';
export type PlaybackState = 'idle' | 'buffering' | 'playing' | 'paused' | 'ended' | 'error';
export type StorageDestinationType = 'shared_local' | 'picked_folder' | 'google_drive';

export interface TrackCandidate {
  candidateId: string;
  sourcePlatform: SourcePlatform;
  sourceItemId: string;
  sourceUrl: string;
  sourceTitle: string;
  uploaderName: string;
  durationMs: number | null;
  thumbnailUrl: string | null;
  searchQuery: string;
  searchRank: number;
  availabilityState: AvailabilityState;
  metadataHintTitle: string | null;
  metadataHintSinger: string | null;
  metadataHintAlbum: string | null;
}

export interface LibraryItem {
  libraryItemId: string;
  sourcePlatform: SourcePlatform;
  sourceItemId: string;
  sourceUrl: string;
  originalSourceTitle: string;
  normalizedSongTitle: string | null;
  normalizedSinger: string | null;
  normalizedAlbum: string | null;
  metadataConfidence: 'high' | 'medium' | 'low' | 'unknown';
  metadataStatus: 'normalized' | 'partial' | 'unresolved';
  localFilePath: string | null;
  localPlaybackState: 'playable' | 'missing' | 'pending' | 'not_local';
  driveBackupState: DriveBackupState;
  duplicateGroupKey: string | null;
}

export interface DownloadJob {
  jobId: string;
  jobType: 'local_save' | 'drive_export';
  sourcePlatform: SourcePlatform;
  sourceItemId: string;
  targetLibraryItemId: string | null;
  requestedDestination: string;
  status: JobStatus;
  progressBytes: number | null;
  totalBytes: number | null;
  failureCode: string | null;
  failureMessage: string | null;
  retryCount: number;
}

export interface StorageDestination {
  destinationId: string;
  destinationType: StorageDestinationType;
  displayName: string;
  writable: boolean;
  requiresAuth: boolean;
  capacityHintBytes: number | null;
  availabilityState: 'ready' | 'unavailable' | 'permission_required' | 'auth_required';
}

export interface PlaybackQueue {
  queueId: string;
  itemIds: string[];
  currentIndex: number;
  playbackState: PlaybackState;
  currentPositionMs: number;
}

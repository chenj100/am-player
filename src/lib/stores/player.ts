import { writable } from 'svelte/store';
import type { PlaybackQueue } from '$lib/types/models';

export const playbackQueue = writable<PlaybackQueue | null>(null);

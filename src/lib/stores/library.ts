import { writable } from 'svelte/store';
import type { LibraryItem } from '$lib/types/models';

export const libraryItems = writable<LibraryItem[]>([]);

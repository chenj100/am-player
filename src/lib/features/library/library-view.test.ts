import { describe, expect, it } from 'vitest';
import { get } from 'svelte/store';
import { libraryItems } from '$lib/stores/library';

describe('library store', () => {
  it('starts empty', () => {
    expect(get(libraryItems)).toEqual([]);
  });
});

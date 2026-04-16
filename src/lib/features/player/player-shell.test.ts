import { describe, expect, it } from 'vitest';
import { get } from 'svelte/store';
import { playbackQueue } from '$lib/stores/player';

describe('player store', () => {
  it('starts without an active queue', () => {
    expect(get(playbackQueue)).toBeNull();
  });
});

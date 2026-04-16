import { describe, expect, it } from 'vitest';
import { defaultDestinations } from '$lib/api/commands';

describe('search flow defaults', () => {
  it('exposes local and Google Drive destinations', () => {
    const destinations = defaultDestinations();
    expect(destinations).toHaveLength(2);
  });
});

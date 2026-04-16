import { writable } from 'svelte/store';
import type { DownloadJob } from '$lib/types/models';

export const jobs = writable<Record<string, DownloadJob>>({});

export function upsertJob(job: DownloadJob) {
  jobs.update((current) => ({ ...current, [job.jobId]: job }));
}

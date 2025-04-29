import type { ParamMatcher } from '@sveltejs/kit';

export const match = ((param: string) => {
  return param === 'en' || param === 'es';
}) satisfies ParamMatcher;

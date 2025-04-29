import { redirect } from '@sveltejs/kit';

const supportedLangs = ['en', 'es'];

export const load = async ({ params, url, request }) => {
  if (!supportedLangs.includes(params.lang || '')) {
    const accept = request.headers.get('accept-language');
    let preferred = accept?.split(',')[0].split('-')[0] ?? 'en';
    preferred = supportedLangs.includes(preferred) ? preferred : 'en';
    throw redirect(302, `/${preferred}${url.pathname}`);
  }

  return {
    lang: params.lang
  };
};

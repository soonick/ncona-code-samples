import i18n from '@sveltekit-i18n/base';
import parser from '@sveltekit-i18n/parser-default';

const config = {
  parser: parser(),
  loaders: [
    {
      locale: 'en',
      key: 'common',
      loader: async () => (await import('$lib/translations/en/common.json')).default
    },
    {
      locale: 'es',
      key: 'common',
      loader: async () => (await import('$lib/translations/es/common.json')).default
    },
  ]
};

export const { t, locale, loadTranslations } = new i18n(config);

<script lang="ts">
  import { page } from '$app/stores';

  import LocalePicker from '$lib/components/LocalePicker.svelte';
  import { t, locale, loadTranslations } from '$lib/utils/Translations';

  let lang = $state($page.params.lang);
  locale.set($page.params.lang);
  $effect(() => {
    lang = $page.params.lang;
    locale.set(lang);
    loadTranslations(lang, 'common');
  });
</script>

<LocalePicker/>
<h1>{$t('common.dashboard_title')}</h1>
<p>{$t('common.dashboard_body')}</p>
<a href={`/${$page.params.lang}`}>{$t('common.homepage_link')}</a>

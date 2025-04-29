<script lang="ts">
  import { page } from '$app/stores';

  let text = $state('');
  let link = $state('');

  $effect(() => {
    const parts = $page.url.pathname.split('/');
    if ($page.params.lang === 'es') {
      text = 'English';
      parts[1] = 'en';
    } else {
      text = 'Español';
      parts[1] = 'es';
    }
    link = parts.join('/');
  });
</script>

<a href={link}>{text}</a>

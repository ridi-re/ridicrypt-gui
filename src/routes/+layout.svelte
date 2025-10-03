<script lang="ts">
    import { onMount } from "svelte";
    import { page } from "$app/state";
    let { children } = $props();
    import "../app.css";
    import { locales, localizeHref, setLocale } from "$lib/paraglide/runtime";
    import type { Locale } from "$lib/paraglide/runtime";
    import { get, set } from "$lib/store";

    function isLocale(l: string): l is Locale {
        return (locales as readonly string[]).includes(l);
    }

    onMount(async () => {
        const saved = await get<string>("locale");
        if (saved && isLocale(saved)) {
            setLocale(saved);
            return;
        }

        const envLang = navigator.language;
        const baseLang = envLang.split("-")[0];
        const locale: Locale = isLocale(envLang)
            ? envLang
            : isLocale(baseLang)
              ? baseLang
              : "en";

        setLocale(locale);
        await set("locale", locale);
    });
</script>

{@render children()}

<div style="display:none">
    {#each locales as locale}
        <a href={localizeHref(page.url.pathname, { locale })}>{locale}</a>
    {/each}
</div>

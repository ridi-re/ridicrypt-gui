<script lang="ts">
    import { HugeiconsIcon } from "@hugeicons/svelte";
    import { ArrowDown01Icon, Globe02Icon } from "@hugeicons/core-free-icons";

    import { m } from "$lib/paraglide/messages";
    import {
        locales,
        getLocale,
        setLocale as _setLocale,
    } from "$lib/paraglide/runtime";
    import type { Locale } from "$lib/paraglide/runtime";
    import { set } from "$lib/store";

    async function setLocale(locale: string) {
        await set<string>("locale", locale);
        _setLocale(locale as Locale);
    }

    let {
        class: className = "",
        ...props
    }: { class?: string; [key: string]: any } = $props();
</script>

<div class="dropdown dropdown-end block select-auto {className}" {...props}>
    <div
        tabindex="0"
        role="button"
        class="btn btn-sm btn-ghost gap-1 px-1.5 font-bold"
        aria-label="Language"
        title="Change Language"
    >
        <HugeiconsIcon icon={Globe02Icon} size={16} strokeWidth={1.5} />
        <HugeiconsIcon icon={ArrowDown01Icon} size={14} strokeWidth={1.5} />
    </div>
    <div
        class="dropdown-content bg-base-100/80 border-[length:var(--border)] border-base-300 text-base-content rounded-box top-[40px] max-h-[40vh] overflow-y-auto overflow-x-hidden backdrop-blur-[5px] shadow-lg"
    >
        <ul class="menu menu-sm w-max gap-[0.25rem]">
            {#each locales as locale}
                <li>
                    <button
                        class="gap-5 {getLocale() === locale
                            ? 'menu-active'
                            : ''}"
                        onclick={() => {
                            setLocale(locale);
                            (
                                document.activeElement as HTMLElement | null
                            )?.blur?.();
                        }}
                    >
                        <span class="font-mono uppercase">{locale}</span>
                        <span class="whitespace-nowrap">
                            {m.label({}, { locale })}
                        </span>
                    </button>
                </li>
            {/each}
        </ul>
    </div>
</div>

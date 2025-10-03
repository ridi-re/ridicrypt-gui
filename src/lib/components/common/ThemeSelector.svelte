<script lang="ts">
    import { HugeiconsIcon } from "@hugeicons/svelte";
    import { ArrowDown01Icon } from "@hugeicons/core-free-icons";

    import { onMount } from "svelte";
    import { m } from "$lib/paraglide/messages";
    import { get, set } from "$lib/store";

    const themes = [
        "ridi",
        "light",
        "dark",
        "cupcake",
        "bumblebee",
        "emerald",
        "corporate",
        "synthwave",
        "retro",
        "cyberpunk",
        "valentine",
        "halloween",
        "garden",
        "forest",
        "aqua",
        "lofi",
        "pastel",
        "fantasy",
        "wireframe",
        "black",
        "luxury",
        "dracula",
        "cmyk",
        "autumn",
        "business",
        "acid",
        "lemonade",
        "night",
        "coffee",
        "winter",
        "dim",
        "nord",
        "sunset",
        "caramellatte",
        "abyss",
        "silk",
    ];

    let currentTheme = $state("ridi");

    async function setTheme(theme: string) {
        document.documentElement.setAttribute("data-theme", theme);
        currentTheme = theme;
        await set<string>("theme", theme);
    }

    let {
        class: className = "",
        ...props
    }: { class?: string; [key: string]: any } = $props();

    onMount(async () => {
        const saved = await get<string>("theme");
        setTheme(saved && themes.includes(saved) ? saved : "ridi");
    });
</script>

<div class="dropdown dropdown-end block select-auto {className}" {...props}>
    <div
        tabindex="0"
        role="button"
        class="btn group btn-sm gap-1.5 px-1.5 btn-ghost"
        title="Change Theme"
    >
        <div
            class="bg-base-100 group-hover:border-base-content/20 border-base-content/10 grid shrink-0 grid-cols-2 gap-0.5 rounded-md border p-1 transition-colors"
        >
            <div class="bg-base-content size-1 rounded-full"></div>
            <div class="bg-primary size-1 rounded-full"></div>
            <div class="bg-secondary size-1 rounded-full"></div>
            <div class="bg-accent size-1 rounded-full"></div>
        </div>
        <HugeiconsIcon icon={ArrowDown01Icon} size={14} strokeWidth={1.5} />
    </div>
    <div
        class="dropdown-content bg-base-100/80 border-[length:var(--border)] border-base-300 text-base-content rounded-box top-[40px] max-h-[40vh] overflow-y-auto overflow-x-hidden backdrop-blur-[5px] shadow-lg"
    >
        <ul class="menu menu-sm w-max gap-[0.25rem]">
            {#each themes as theme}
                <li>
                    <button
                        class="gap-2 {currentTheme === theme
                            ? 'menu-active'
                            : ''}"
                        onclick={() => {
                            setTheme(theme);
                            (
                                document.activeElement as HTMLElement | null
                            )?.blur?.();
                        }}
                    >
                        <div
                            data-theme={theme}
                            class="bg-base-100 grid shrink-0 grid-cols-2 gap-0.5 rounded-md p-1 shadow-sm"
                        >
                            <div
                                class="bg-base-content size-1 rounded-full"
                            ></div>
                            <div class="bg-primary size-1 rounded-full"></div>
                            <div class="bg-secondary size-1 rounded-full"></div>
                            <div class="bg-accent size-1 rounded-full"></div>
                        </div>
                        <div class="whitespace-nowrap">
                            {m[
                                `theme.${theme}` as Extract<
                                    keyof typeof m,
                                    `theme.${string}`
                                >
                            ]()}
                        </div>
                    </button>
                </li>
            {/each}
        </ul>
    </div>
</div>

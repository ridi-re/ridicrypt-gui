<script lang="ts">
    import { HugeiconsIcon } from "@hugeicons/svelte";
    import {
        ArrowMoveUpRightIcon,
        BookOpen02FreeIcons,
        Folder01Icon,
        PlayIcon,
    } from "@hugeicons/core-free-icons";

    import { m } from "$lib/paraglide/messages";

    let {
        coverUrl = "",
        title = "",
        author = "",
        onOpen = () => {},
        onOpenFolder = () => {},
        onExport = () => {},
        ...restProps
    }: {
        coverUrl?: string;
        title?: string;
        author?: string;
        onOpen?: () => void;
        onOpenFolder?: () => void;
        onExport?: () => void;
        [key: string]: any;
    } = $props();

    const bookTitle = $derived(title.trim() || m["library.book.untitled"]());

    const handleAction = (action: () => void) => (event: MouseEvent) => {
        event.stopPropagation();
        action();
        (document.activeElement as HTMLElement | null)?.blur?.();
    };
</script>

<div
    class="group card relative cursor-pointer bg-base-100/75 card-border border-base-300"
    role="button"
    tabindex="0"
    onclick={handleAction(onOpen)}
    onkeydown={(e) =>
        (e.key === "Enter" || e.key === " ") && (e.preventDefault(), onOpen())}
    {...restProps}
>
    <figure class="relative aspect-[2/3] w-full select-none overflow-hidden">
        <div
            class="h-full w-full transform transition duration-300 ease-out group-hover:blur-sm group-hover:scale-[1.08] group-focus-within:blur-sm group-focus-within:scale-[1.08]"
        >
            {#if coverUrl}
                <img
                    src={coverUrl}
                    alt={bookTitle}
                    class="h-full w-full object-cover"
                    loading="lazy"
                    decoding="async"
                    draggable="false"
                />
            {:else}
                <div
                    class="flex h-full w-full items-center justify-center bg-base-200"
                >
                    <div
                        class="pointer-events-none select-none text-center text-base-content/50"
                    >
                        <div
                            class="mx-auto mb-2 flex h-12 w-12 items-center justify-center"
                        >
                            <HugeiconsIcon
                                icon={BookOpen02FreeIcons}
                                size={46}
                                strokeWidth={2}
                            />
                        </div>
                        <p class="text-xs">{m["library.book.noCover"]()}</p>
                    </div>
                </div>
            {/if}
        </div>

        <div
            class="pointer-events-none absolute inset-0 flex items-center justify-center opacity-0 transition-opacity duration-150 ease-out group-hover:pointer-events-auto group-hover:opacity-100 group-focus-within:pointer-events-auto group-focus-within:opacity-100"
        >
            <div class="flex flex-col gap-4">
                <button
                    type="button"
                    class="btn btn-primary btn-square btn-lg"
                    onclick={(e) => handleAction(onOpen)(e)}
                >
                    <HugeiconsIcon icon={PlayIcon} size={20} strokeWidth={3} />
                </button>
                <button
                    type="button"
                    class="btn btn-warning btn-square btn-lg"
                    onclick={(e) => handleAction(onOpenFolder)(e)}
                >
                    <HugeiconsIcon
                        icon={Folder01Icon}
                        size={16}
                        strokeWidth={3}
                    />
                </button>
                <button
                    type="button"
                    class="btn btn-success btn-square btn-lg"
                    onclick={(e) => handleAction(onExport)(e)}
                >
                    <HugeiconsIcon
                        icon={ArrowMoveUpRightIcon}
                        size={16}
                        strokeWidth={4}
                    />
                </button>
            </div>
        </div>
    </figure>

    <div class="card-body select-none p-3">
        <h3
            class="card-title line-clamp-2 text-sm font-medium leading-tight"
            title={bookTitle}
        >
            {bookTitle}
        </h3>
        {#if author.trim()}
            <p
                class="mt-1 line-clamp-1 text-xs text-base-content/70"
                title={author}
            >
                {author}
            </p>
        {/if}
    </div>
</div>

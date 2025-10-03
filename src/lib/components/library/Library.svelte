<script lang="ts">
    import { m } from "$lib/paraglide/messages";
    import { setContext } from "svelte";
    import type { Snippet } from "svelte";
    import { writable } from "svelte/store";
    import type { LibraryContext } from "./context";
    import { libraryContextKey } from "./context";
    import Select from "../common/Select.svelte";
    import Book from "./Book.svelte";

    type SortOption = { value: string; label: string };
    type SortDirection = "asc" | "desc";
    type SelectionAction = { value: string; label: string };

    type BookData = {
        id: string;
        title: string;
        author: string;
        coverUrl?: string;
        onOpen?: () => void;
        onOpenFolder?: () => void;
        onExport?: () => void;
    };

    let {
        users = [],
        selectedUserId = null,
        sortOptions = [],
        selectedSort = null,
        selectedSortDirection = "asc" as SortDirection,
        selectionActions = [],
        books = [],
        userchange = () => {},
        sortchange = () => {},
        selectionaction = async () => false,
        empty,
    }: {
        users?: string[];
        selectedUserId?: string | null;
        sortOptions?: SortOption[];
        selectedSort?: string | null;
        selectedSortDirection?: SortDirection;
        selectionActions?: SelectionAction[];
        books?: BookData[];
        userchange?: (payload: { userId: string | null }) => void;
        sortchange?: (payload: {
            sort: string;
            direction: SortDirection;
        }) => void;
        selectionaction?: (payload: {
            action: string;
            selected: string[];
        }) => Promise<boolean>;
        empty?: Snippet;
    } = $props();

    const selectionMode = writable(false);
    const selectedIds = writable<Set<string>>(new Set());

    const context: LibraryContext = {
        get selectionMode() {
            return $selectionMode;
        },
        get selectedIds() {
            return $selectedIds;
        },
        toggleSelection(id) {
            selectedIds.update((s) => {
                const next = new Set(s);
                next.has(id) ? next.delete(id) : next.add(id);
                return next;
            });
        },
        registerBook: () => () => {},
        clearSelection() {
            selectedIds.set(new Set());
        },
    };

    setContext<LibraryContext>(libraryContextKey, context);

    let pendingAction = $state(selectionActions[0]?.value ?? "");
    const selectedCount = $derived($selectedIds.size);
    const selectionModeEnabled = $derived($selectionMode);
    const allSelected = $derived(
        books.length > 0 && selectedCount === books.length
    );

    $effect(() => {
        if (!selectionModeEnabled) context.clearSelection();
    });

    const toggleSelectAll = () => {
        if (allSelected) {
            context.clearSelection();
        } else {
            selectedIds.set(new Set(books.map((b) => b.id)));
        }
    };

    const sortOrderOptions = [
        { value: "asc" as const, labelKey: "library.sort.asc" },
        { value: "desc" as const, labelKey: "library.sort.desc" },
    ];
</script>

<div class="flex flex-col">
    <div class="flex flex-wrap items-center gap-4 rounded-box px-4 pb-2">
        {#if users.length > 0}
            <Select
                label={m["library.user.label"]()}
                value={selectedUserId ?? ""}
                onchange={(e: Event) =>
                    userchange({
                        userId: (e.target as HTMLSelectElement).value || null,
                    })}
            >
                <option value="">{m["library.user.all"]()}</option>
                {#each users as user}
                    <option value={user}>{user}</option>
                {/each}
            </Select>
        {/if}

        {#if sortOptions.length > 0}
            <Select
                label={m["library.sort.strategy"]()}
                value={selectedSort ?? sortOptions[0]?.value ?? ""}
                onchange={(e: Event) =>
                    sortchange({
                        sort: (e.target as HTMLSelectElement).value,
                        direction: selectedSortDirection,
                    })}
            >
                {#each sortOptions as option}
                    <option value={option.value}>{option.label}</option>
                {/each}
            </Select>
            <Select
                label={m["library.sort.order"]()}
                value={selectedSortDirection}
                onchange={(e: Event) =>
                    sortchange({
                        sort: selectedSort ?? sortOptions[0]?.value ?? "",
                        direction: (e.target as HTMLSelectElement)
                            .value as SortDirection,
                    })}
            >
                {#each sortOrderOptions as option}
                    <option value={option.value}>
                        {option.labelKey === "library.sort.asc"
                            ? m["library.sort.asc"]()
                            : m["library.sort.desc"]()}
                    </option>
                {/each}
            </Select>
        {/if}
    </div>
    <div class="flex flex-wrap items-center gap-4 rounded-box px-4 pb-6">
        {#if selectionModeEnabled}
            <button
                type="button"
                class="btn btn-sm bg-base-100 border-base-300 border"
                onclick={() => selectionMode.set(false)}
            >
                {m["library.selection.exit"]()}
            </button>
            <button
                type="button"
                class="btn btn-sm bg-base-100 border-base-300 border"
                onclick={toggleSelectAll}
            >
                {allSelected
                    ? m["library.selection.deselectAll"]()
                    : m["library.selection.selectAll"]()}
            </button>
            {#if selectionActions.length > 0}
                <Select bind:value={pendingAction}>
                    {#each selectionActions as action}
                        <option value={action.value}>{action.label}</option>
                    {/each}
                </Select>
                <button
                    type="button"
                    class="btn btn-sm bg-base-100 border-base-300 border"
                    disabled={!pendingAction || selectedCount === 0}
                    onclick={async () => {
                        if (pendingAction && selectedCount > 0) {
                            const shouldExit = await selectionaction({
                                action: pendingAction,
                                selected: Array.from($selectedIds),
                            });
                            if (shouldExit) {
                                selectionMode.set(false);
                            }
                        }
                    }}
                >
                    {m["library.selection.apply"]()}
                </button>
            {/if}
            <div class="badge badge-sm badge-outline">
                {m["library.selection.count"]({ count: selectedCount })}
            </div>
        {:else}
            <button
                type="button"
                class="btn btn-sm bg-base-100 border-base-300 border"
                onclick={() => selectionMode.set(true)}
            >
                {m["library.selection.enter"]()}
            </button>
        {/if}
    </div>

    <div class="px-4 pb-4">
        <div
            class="grid auto-rows-max gap-4 [grid-template-columns:repeat(auto-fill,minmax(10rem,1fr))]"
        >
            {#each books as book}
                <div class="relative">
                    <Book
                        coverUrl={book.coverUrl ?? ""}
                        title={book.title}
                        author={book.author}
                        onOpen={book.onOpen}
                        onOpenFolder={book.onOpenFolder}
                        onExport={book.onExport}
                    />
                    {#if selectionModeEnabled}
                        <div
                            class="absolute inset-0 z-10 cursor-pointer"
                            role="button"
                            tabindex="0"
                            onclick={() => context.toggleSelection(book.id)}
                            onkeydown={(e) => {
                                if (e.key === "Enter" || e.key === " ") {
                                    e.preventDefault();
                                    context.toggleSelection(book.id);
                                }
                            }}
                        ></div>
                        <!-- Checkbox -->
                        <div
                            class="absolute right-2 top-2 z-20 pointer-events-none"
                        >
                            <input
                                type="checkbox"
                                class="checkbox checkbox-primary pointer-events-none"
                                checked={$selectedIds.has(book.id)}
                                readonly
                                tabindex="-1"
                            />
                        </div>
                    {/if}
                </div>
            {:else}
                {#if empty}
                    {@render empty()}
                {/if}
            {/each}
        </div>
    </div>
</div>

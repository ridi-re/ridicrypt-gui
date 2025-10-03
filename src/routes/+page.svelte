<script lang="ts">
    import { onMount } from "svelte";
    import Window from "$lib/components/common/Window.svelte";
    import Library from "$lib/components/library/Library.svelte";
    import { m } from "$lib/paraglide/messages";
    import { invoke } from "@tauri-apps/api/core";
    import {
        open as dialogOpen,
        save as dialogSave,
    } from "@tauri-apps/plugin-dialog";
    import {
        openPath as openWithSystem,
        revealItemInDir,
    } from "@tauri-apps/plugin-opener";

    const REFRESH_INTERVAL = 5_000;

    type SortDirection = "asc" | "desc";

    type LibraryEntry = {
        title?: { main?: string };
        authors?: Array<{ name?: string }>;
        thumbnail?: { large?: string; small?: string; xxlarge?: string };
        file?: { format?: string };
        storage?: {
            basePath?: string;
            filename?: string;
            keyFilename?: string;
            createTime?: string;
        };
    };

    type StorageInfo = {
        basePath: string;
        filename: string;
        keyFilename: string;
        createTime?: string;
    };

    type BookRecord = {
        id: string;
        bookId: string;
        ownerId: string;
        title: string;
        author: string;
        coverUrl?: string;
        storage: StorageInfo;
        downloadedAt: Date;
        format: string;
    };

    const sortOptions = [
        { value: "title", label: m["custom.library.title"]() },
        { value: "author", label: m["custom.library.author"]() },
        { value: "recent", label: m["custom.library.recent"]() },
    ];

    const selectionActions = [
        { value: "export", label: m["custom.action.export"]() },
    ];

    let rawLibrary = $state<Record<string, Record<string, LibraryEntry>>>({});
    let activeUser = $state<string | null>(null);
    let sortBy = $state(sortOptions[0]?.value ?? "title");
    let sortDirection = $state<SortDirection>("asc");

    let refreshTimer: ReturnType<typeof setInterval> | undefined;
    let windowComponent: any = $state();

    const notify = (msg: string, opts?: any) =>
        windowComponent?.notify?.(msg, opts);

    const users = $derived(Object.keys(rawLibrary));

    const allBooks = $derived(() => {
        const aggregated: BookRecord[] = [];
        for (const [userId, userBooks] of Object.entries(rawLibrary)) {
            if (!userBooks) continue;
            for (const [bookId, data] of Object.entries(userBooks)) {
                if (!data) continue;
                const storage = data.storage ?? {};
                const basePath = storage.basePath;
                const filename = storage.filename;
                const keyFilename = storage.keyFilename;
                if (!basePath || !filename || !keyFilename) {
                    continue;
                }
                const authorName =
                    data.authors
                        ?.map((a) => a?.name)
                        .filter(Boolean)
                        .join(", ") || "Unknown";
                const timestamp = Number(storage.createTime);
                const downloadedAt =
                    timestamp && !isNaN(timestamp)
                        ? new Date(timestamp * 1000)
                        : new Date(0);
                const ext = filename.split(".").pop() || "";
                const format = (data.file?.format || ext).toLowerCase();
                aggregated.push({
                    id: `${userId}:${bookId}`,
                    bookId,
                    ownerId: userId,
                    title: data.title?.main ?? bookId,
                    author: authorName,
                    coverUrl:
                        data.thumbnail?.large ??
                        data.thumbnail?.small ??
                        data.thumbnail?.xxlarge,
                    storage: {
                        basePath,
                        filename,
                        keyFilename,
                        createTime: storage.createTime,
                    },
                    downloadedAt,
                    format,
                });
            }
        }
        return aggregated;
    });

    const bookIndex = $derived(
        new Map(allBooks().map((book) => [book.id, book]))
    );

    const filteredBooks = $derived(
        allBooks().filter((book) => !activeUser || book.ownerId === activeUser)
    );

    const visibleBooks = $derived(
        [...filteredBooks].sort((a, b) => {
            const factor = sortDirection === "asc" ? 1 : -1;
            let result = 0;
            if (sortBy === "recent") {
                result = a.downloadedAt.getTime() - b.downloadedAt.getTime();
            } else if (sortBy === "author") {
                result = a.author.localeCompare(b.author, undefined, {
                    numeric: true,
                    sensitivity: "base",
                });
            } else {
                result = a.title.localeCompare(b.title, undefined, {
                    numeric: true,
                    sensitivity: "base",
                });
            }
            return result * factor;
        })
    );

    const booksForLibrary = $derived(
        visibleBooks.map((book) => ({
            id: book.id,
            title: book.title,
            author: book.author,
            coverUrl: book.coverUrl,
            onOpen: () => openBook(book),
            onOpenFolder: () => openContainingFolder(book),
            onExport: () => exportSingleBook(book),
        }))
    );

    const joinPath = (base: string, name: string) => {
        const normalizedBase = base.replace(/\\/g, "/").replace(/\/+$/, "");
        return `${normalizedBase}/${name}`;
    };

    const defaultDecryptedName = (filename: string) => {
        const index = filename.lastIndexOf(".");
        if (index === -1) {
            return `${filename}.decrypted`;
        }
        const stem = filename.slice(0, index);
        const ext = filename.slice(index + 1);
        return ext ? `${stem}.decrypted.${ext}` : `${stem}.decrypted`;
    };

    const refreshLibrary = async () => {
        try {
            const response = await invoke<{
                success: boolean;
                value?: string;
                error?: string;
            }>("get_library");
            if (response.success && response.value) {
                const parsed = JSON.parse(response.value);
                rawLibrary = parsed;
            } else {
                console.error("Failed to get library:", response.error);
                notify(m["notification.library.refreshFailed"](), {
                    type: "error",
                });
                rawLibrary = {};
            }
        } catch (error) {
            console.error("Failed to refresh library", error);
            notify(m["notification.library.refreshFailed"](), {
                type: "error",
            });
            rawLibrary = {};
        }
    };

    onMount(() => {
        refreshLibrary();
        refreshTimer = setInterval(refreshLibrary, REFRESH_INTERVAL);
        return () => clearInterval(refreshTimer);
    });

    $effect(() => {
        if (users.length === 0) {
            activeUser = null;
            return;
        }
        if (activeUser === null) {
            return;
        }
        if (users.includes(activeUser)) {
            return;
        }
        activeUser = null;
    });

    const decryptBook = async (book: BookRecord, targetPath: string) => {
        const keyPath = joinPath(
            book.storage.basePath,
            book.storage.keyFilename
        );
        const filePath = joinPath(book.storage.basePath, book.storage.filename);
        const response = await invoke<{
            success: boolean;
            value?: null;
            error?: string;
        }>("decrypt", {
            keyPath,
            filePath,
            targetPath,
        });
        if (!response.success) {
            throw new Error(response.error || "Decryption failed");
        }
    };

    const openBook = async (book: BookRecord) => {
        const loadingNotification = notify(m["notification.book.opening"](), {
            type: "info",
            duration: null,
        });
        try {
            const tempPathResponse = await invoke<{
                success: boolean;
                value?: string;
                error?: string;
            }>("get_temp_book_path", {
                bookId: book.bookId,
                ownerId: book.ownerId,
                format: book.format,
            });

            if (!tempPathResponse.success || !tempPathResponse.value) {
                throw new Error(
                    tempPathResponse.error || "Failed to get temp path"
                );
            }

            const targetPath = tempPathResponse.value;
            await decryptBook(book, targetPath);
            await openWithSystem(targetPath);

            loadingNotification?.close();
            notify(m["notification.book.opened"]({ title: book.title }), {
                type: "success",
            });
        } catch (error) {
            console.error("Failed to open book", book.bookId, error);
            loadingNotification?.close();
            notify(m["notification.book.openFailed"]({ title: book.title }), {
                type: "error",
            });
        }
    };

    let isOpeningFolder = false;
    const openContainingFolder = async (book: BookRecord) => {
        if (isOpeningFolder) return;
        isOpeningFolder = true;

        try {
            const fileToReveal = book.storage.filename
                ? joinPath(book.storage.basePath, book.storage.filename)
                : joinPath(book.storage.basePath, book.storage.keyFilename);

            console.log("Revealing item in folder:", fileToReveal);
            await revealItemInDir(fileToReveal);
            notify(m["notification.folder.opened"](), { type: "success" });
        } catch (error) {
            console.error(
                "Failed to open containing folder",
                book.storage.basePath,
                error
            );
            notify(m["notification.folder.openFailed"](), { type: "error" });
        } finally {
            isOpeningFolder = false;
        }
    };

    const exportSingleBook = async (book: BookRecord) => {
        const defaultFileName = defaultDecryptedName(book.storage.filename);
        const defaultPath = joinPath(book.storage.basePath, defaultFileName);
        try {
            const filters = book.format
                ? [
                      {
                          name: book.format.toUpperCase(),
                          extensions: [book.format],
                      },
                  ]
                : undefined;
            const targetPath = await dialogSave({ defaultPath, filters });
            if (!targetPath) {
                return;
            }

            const loadingNotification = notify(
                m["notification.export.exporting"](),
                { type: "info", duration: null }
            );
            await decryptBook(book, targetPath);
            loadingNotification?.close();
            notify(m["notification.export.exported"]({ title: book.title }), {
                type: "success",
            });
        } catch (error) {
            console.error("Failed to export book", book.bookId, error);
            notify(
                m["notification.export.exportFailed"]({ title: book.title }),
                {
                    type: "error",
                }
            );
        }
    };

    const exportBooks = async (bookIds: string[]) => {
        try {
            const target = await dialogOpen({
                directory: true,
                multiple: false,
            });
            if (!target || Array.isArray(target)) {
                return;
            }

            const loadingNotification = notify(
                m["notification.export.exportingMultiple"]({
                    count: bookIds.length,
                }),
                { type: "info", duration: null }
            );
            let successCount = 0;
            let failCount = 0;

            for (const id of bookIds) {
                const book = bookIndex.get(id);
                if (!book) {
                    failCount++;
                    continue;
                }

                const decryptedName = defaultDecryptedName(
                    book.storage.filename
                );
                const targetPath = joinPath(target, decryptedName);

                try {
                    console.log(
                        `Exporting book ${book.bookId} to ${targetPath}`
                    );
                    await decryptBook(book, targetPath);
                    successCount++;
                } catch (error) {
                    console.error("Failed to export book", book.bookId, error);
                    failCount++;
                }
            }

            loadingNotification?.close();

            if (failCount === 0) {
                notify(
                    m["notification.export.exportedAll"]({
                        count: successCount,
                    }),
                    { type: "success" }
                );
            } else if (successCount === 0) {
                notify(
                    m["notification.export.exportFailedAll"]({
                        count: failCount,
                    }),
                    { type: "error" }
                );
            } else {
                notify(
                    m["notification.export.exportPartial"]({
                        success: successCount,
                        failed: failCount,
                    }),
                    { type: "warning" }
                );
            }

            console.log(
                `Export completed: ${successCount} succeeded, ${failCount} failed`
            );
        } catch (error) {
            console.error("Failed to choose export directory", error);
            notify(m["notification.export.chooseDirFailed"](), {
                type: "error",
            });
        }
    };

    const handleUserChange = ({ userId }: { userId: string | null }) =>
        (activeUser = userId);

    const handleSortChange = ({
        sort,
        direction,
    }: {
        sort: string;
        direction: SortDirection;
    }) => {
        sortBy = sort;
        sortDirection = direction;
    };

    const handleSelectionAction = async ({
        action,
        selected,
    }: {
        action: string;
        selected: string[];
    }) => {
        if (action === "export" && selected.length > 0) {
            await exportBooks(selected);
            return true;
        }
        return false;
    };
</script>

<Window bind:this={windowComponent}>
    <Library
        {users}
        selectedUserId={activeUser}
        {sortOptions}
        selectedSort={sortBy}
        selectedSortDirection={sortDirection}
        {selectionActions}
        books={booksForLibrary}
        userchange={handleUserChange}
        sortchange={handleSortChange}
        selectionaction={handleSelectionAction}
    >
        {#snippet empty()}
            <div
                class="col-span-full flex flex-col items-center justify-center rounded-xl border border-dashed border-base-300 p-12 text-center text-base-content/60"
            >
                <h2 class="text-lg font-semibold">No books found</h2>
                <p class="text-sm">
                    Try switching user or adjusting your sort strategy.
                </p>
            </div>
        {/snippet}
    </Library>
</Window>

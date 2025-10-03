export interface LibraryContext {
    get selectionMode(): boolean;
    get selectedIds(): Set<string>;
    toggleSelection: (id: string) => void;
    registerBook: (id: string) => () => void;
    clearSelection: () => void;
}

export const libraryContextKey = Symbol("library-context");

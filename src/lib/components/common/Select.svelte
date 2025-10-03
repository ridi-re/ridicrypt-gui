<script lang="ts">
    type OptionItem = {
        value: string;
        label: string;
        disabled: boolean;
    };

    let {
        label,
        value = $bindable(),
        children,
        ...props
    }: {
        label?: string;
        value?: string | number;
        children?: () => any;
        [k: string]: any;
    } = $props();

    let selectElement: HTMLSelectElement | null = null;
    let observer: MutationObserver | null = null;
    let optionItems = $state<OptionItem[]>([]);

    const normalize = (input: string | number | null | undefined) =>
        input == null ? "" : String(input);

    const inSync = (current: OptionItem[], next: OptionItem[]) =>
        current.length === next.length &&
        current.every((item, index) => {
            const other = next[index];
            return (
                other &&
                item.value === other.value &&
                item.label === other.label &&
                item.disabled === other.disabled
            );
        });

    const syncOptions = () => {
        if (!selectElement) {
            if (optionItems.length > 0) {
                optionItems = [];
            }
            return;
        }

        const next = Array.from(selectElement.options).map((option) => ({
            value: option.value,
            label: option.label || option.text,
            disabled: option.disabled,
        }));

        if (inSync(optionItems, next)) return;
        optionItems = next;
    };

    const currentValue = $derived(normalize(value));

    const displayedText = $derived(
        optionItems.length === 0
            ? currentValue
            : (optionItems.find((item) => item.value === currentValue)?.label ??
                  currentValue)
    );

    const emitNativeChange = () => {
        if (!selectElement) return;
        queueMicrotask(() => {
            if (!selectElement) return;
            selectElement.dispatchEvent(new Event("input", { bubbles: true }));
            selectElement.dispatchEvent(new Event("change", { bubbles: true }));
        });
    };

    const selectOption = (option: OptionItem) => {
        if (option.disabled) return;
        const nextValue = option.value;
        if (nextValue === currentValue) return;
        value = nextValue;
        emitNativeChange();
        (document.activeElement as HTMLElement | null)?.blur?.();
    };

    const observeSelect = (node: HTMLSelectElement) => {
        observer?.disconnect();
        selectElement = node;
        syncOptions();
        observer = new MutationObserver(() => syncOptions());
        observer.observe(node, {
            childList: true,
            subtree: true,
            characterData: true,
            attributes: true,
        });
        return {
            destroy() {
                observer?.disconnect();
                observer = null;
                if (selectElement === node) {
                    selectElement = null;
                    if (optionItems.length > 0) {
                        optionItems = [];
                    }
                }
            },
        };
    };
</script>

<div
    class="input input-sm sel w-max pr-0 cursor-default select-none border-[length:var(--border)]"
    class:pl-0={!label}
>
    {#if label}
        <span class="font-semibold">
            {label}
        </span>
    {/if}

    <div class="dropdown dropdown-end h-full">
        <div
            tabindex="0"
            role="button"
            class="btn group flex items-center btn-ghost h-full font-normal text-xs p-2"
        >
            {#if displayedText}{displayedText}{/if}
            <span
                class="inline-block w-0 h-0 border-l-[0.3em] border-r-[0.3em] border-t-[0.3em] border-transparent border-t-current"
            ></span>
        </div>

        <div
            class="dropdown-content bg-base-100/80 border-[length:var(--border)] border-base-300 text-base-content rounded-box top-[36px] max-h-[40vh] overflow-y-auto overflow-x-hidden backdrop-blur-[5px] shadow-lg"
        >
            <ul class="menu menu-xs w-max gap-[0.25rem]">
                {#each optionItems as option}
                    <li>
                        <button
                            type="button"
                            class="gap-2"
                            class:menu-active={option.value === currentValue}
                            disabled={option.disabled}
                            onclick={() => selectOption(option)}
                        >
                            <div class="whitespace-nowrap">{option.label}</div>
                        </button>
                    </li>
                {/each}
            </ul>
        </div>
    </div>

    <select use:observeSelect bind:value {...props}>
        {@render children?.()}
    </select>
</div>

<style>
    .sel:focus-within {
        outline: none;
        box-shadow: none;
        border-color: var(--color-base-300);
    }
    :global(.sel-ghost .sel:focus-within) {
        outline: none;
        box-shadow: none;
        border: none;
    }
    select {
        display: none;
    }
</style>

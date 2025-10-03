<script lang="ts">
    // ==================== imports ====================
    // >>>>> common <<<<<
    import { getCurrentWindow, Effect } from "@tauri-apps/api/window";
    // >>>>> init <<<<<
    import { onMount } from "svelte";
    import type { Snippet } from "svelte";

    // >>>>> control <<<<<
    import { platform } from "@tauri-apps/plugin-os";

    import { HugeiconsIcon } from "@hugeicons/svelte";
    import {
        Copy01Icon,
        Cancel01Icon,
        Delete02Icon,
        Remove01Icon,
        SquareIcon,
    } from "@hugeicons/core-free-icons";

    import { m } from "$lib/paraglide/messages";
    import Icon from "./Icon.svelte";
    import ThemeSelector from "./ThemeSelector.svelte";
    import LanguageSelector from "./LanguageSelector.svelte";

    // >>>>> notification <<<<<
    import { fly, scale } from "svelte/transition";
    import { flip } from "svelte/animate";
    import { cubicOut, quintOut } from "svelte/easing";

    // ==================== types ====================
    type NotificationType = "info" | "success" | "warning" | "error";
    type Notification = {
        id: number;
        type: NotificationType;
        message: string;
        duration: number | null;
        timerId: number;
    };

    // ==================== main ====================
    const currentPlatform = platform();
    const appWindow = getCurrentWindow();

    let { children }: { children?: Snippet } = $props();

    let isMaximized = $state(false);

    // ==================== notifications ====================
    const DURATION_MAP: Record<NotificationType, number | null> = {
        success: 3000,
        info: 5000,
        warning: 10000,
        error: null,
    };

    const ANIMATION = {
        in: { y: 20, duration: 600, easing: quintOut },
        out: { start: 0.95, duration: 300, easing: cubicOut, opacity: 0 },
        flip: { duration: 500, easing: quintOut },
    } as const;

    let notifications = $state<Notification[]>([]);
    let notificationIdCounter = 0;

    export function notify(
        message: string,
        options?: {
            type?: NotificationType;
            duration?: number | null;
        }
    ) {
        const type = options?.type ?? "info";
        const duration =
            options?.duration !== undefined
                ? options.duration
                : DURATION_MAP[type];

        const id = ++notificationIdCounter;
        const timerId =
            duration !== null && duration > 0
                ? window.setTimeout(() => removeNotification(id), duration)
                : 0;

        notifications = [
            ...notifications,
            { id, type, message, duration, timerId },
        ];

        return { close: () => removeNotification(id), id };
    }

    function removeNotification(id: number) {
        notifications = notifications.filter((n) => {
            if (n.id === id && n.timerId) window.clearTimeout(n.timerId);
            return n.id !== id;
        });
    }

    $effect(() => {
        appWindow.isMaximized().then((v: boolean) => (isMaximized = v));

        const unlisten = appWindow.onResized(async () => {
            isMaximized = await appWindow.isMaximized();
        });

        return () => {
            unlisten.then((fn: () => void) => fn());
            // cleanup all notification timers on unmount
            notifications.forEach(
                (n) => n.timerId && window.clearTimeout(n.timerId)
            );
        };
    });

    onMount(async () => {
        // window effects
        await appWindow.setEffects({
            effects: [Effect.Acrylic, Effect.Blur],
        });

        // expose notify to window
        (window as any).notify = notify;
    });
</script>

<main class="flex flex-col">
    <!-- notification -->
    <div
        class="absolute right-[15px] w-[400px] gap-[12px] top-[50px] z-20 flex flex-col max-h-[calc(100vh-60px)] overflow-y-auto overflow-x-hidden select-none [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none]"
    >
        {#each notifications as notification (notification.id)}
            {@const isAlert =
                notification.type === "error" ||
                notification.type === "warning"}
            <div
                role={isAlert ? "alert" : "status"}
                aria-live={isAlert ? "assertive" : "polite"}
                class="alert alert-soft alert-vertical sm:alert-horizontal flex p-2 opacity-98"
                class:alert-info={notification.type === "info"}
                class:alert-success={notification.type === "success"}
                class:alert-warning={notification.type === "warning"}
                class:alert-error={notification.type === "error"}
                in:fly={ANIMATION.in}
                out:scale={ANIMATION.out}
                animate:flip={ANIMATION.flip}
            >
                <div class="flex-1">
                    <div class="text-sm pl-2 break-words">
                        {notification.message}
                    </div>
                </div>
                <button
                    class="btn btn-ghost btn-sm btn-soft btn-circle"
                    class:btn-info={notification.type === "info"}
                    class:btn-success={notification.type === "success"}
                    class:btn-warning={notification.type === "warning"}
                    class:btn-error={notification.type === "error"}
                    aria-label="Close notification"
                    onclick={() => removeNotification(notification.id)}
                >
                    <HugeiconsIcon icon={Delete02Icon} size={22} />
                </button>
            </div>
        {/each}
    </div>

    <!-- title bar -->
    <div data-tauri-drag-region class="h-[40px] flex flex-shrink-0">
        {#if currentPlatform != "macos"}
            <span
                data-tauri-drag-region
                class="flex items-center gap-[5px] px-[7px] font-[500] text-primary pointer-events-none"
            >
                <Icon
                    class="pointer-events-none"
                    src="/favicon.svg"
                    size={24}
                />
                {m.title()}
            </span>
        {/if}
        <div data-tauri-drag-region class="flex-1"></div>
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <span
            class="flex items-center gap-[5px]"
            class:p-[5px]={currentPlatform === "macos"}
            oncontextmenu={(e) => e.preventDefault()}
        >
            <ThemeSelector />
            <LanguageSelector />

            {#if currentPlatform != "macos"}
                <button
                    class="w-[45px] h-full grid place-items-center hover:bg-[color-mix(in_srgb,var(--color-base-content)_20%,var(--color-base-100)_80%)] active:bg-[color-mix(in_srgb,var(--color-base-content)_30%,var(--color-base-100)_70%)]"
                    aria-label="Minimize"
                    onclick={appWindow.minimize}
                >
                    <HugeiconsIcon
                        icon={Remove01Icon}
                        size={18}
                        strokeWidth={2}
                    />
                </button>
                <button
                    class="w-[45px] h-full grid place-items-center hover:bg-[color-mix(in_srgb,var(--color-base-content)_20%,var(--color-base-100)_80%)] active:bg-[color-mix(in_srgb,var(--color-base-content)_30%,var(--color-base-100)_70%)]"
                    aria-label="Maximize"
                    onclick={appWindow.toggleMaximize}
                >
                    {#if isMaximized}
                        <HugeiconsIcon
                            icon={Copy01Icon}
                            size={13}
                            strokeWidth={2.8}
                        />
                    {:else}
                        <HugeiconsIcon
                            icon={SquareIcon}
                            size={13}
                            strokeWidth={2.8}
                        />
                    {/if}
                </button>
                <button
                    class="w-[45px] h-full grid place-items-center hover:bg-error hover:text-white active:bg-[color-mix(in_srgb,var(--color-error)_50%,var(--color-base-content)_50%)] active:text-white"
                    aria-label="Close"
                    onclick={appWindow.close}
                >
                    <HugeiconsIcon
                        icon={Cancel01Icon}
                        size={20}
                        strokeWidth={2}
                    />
                </button>
            {/if}
        </span>
    </div>

    <!-- content -->
    <div class="overflow-y-auto flex-1">
        {@render children?.()}
    </div>
</main>

<style>
    :root {
        background-color: color-mix(
            in srgb,
            var(--color-base-100) 70%,
            transparent
        );
    }
    :global(html, body) {
        height: 100%;
    }
    main {
        height: 100%;
        background: radial-gradient(
                1200px 900px at 40% 10%,
                color-mix(in srgb, var(--color-base-300) 70%, transparent) 10%,
                transparent 60%
            ),
            radial-gradient(
                700px 500px at 90% 90%,
                color-mix(in srgb, var(--color-accent) 12.5%, transparent) 0,
                transparent 60%
            ),
            radial-gradient(
                600px 800px at 20% 50%,
                color-mix(in srgb, var(--color-warning) 10%, transparent) 0,
                transparent 60%
            );
    }
</style>

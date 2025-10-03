<script lang="ts">
    import { onMount } from "svelte";

    let {
        src,
        size = 24,
        class: className = "",
        style: style = "",
        ...props
    }: {
        src: string;
        size?: number;
        class?: string;
        style?: string;
        [key: string]: any;
    } = $props();

    let svgContent = $state("");

    const sanitizeSvg = (content: string): string => {
        return content
            .replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, "")
            .replace(/on\w+="[^"]*"/gi, "")
            .replace(/javascript:/gi, "");
    };

    onMount(async () => {
        const res = await fetch(src);
        svgContent = sanitizeSvg(await res.text());
    });
</script>

<div
    class="icon {className}"
    style="width: {size}px; height: {size}px; {style}"
    {...props}
>
    {@html svgContent}
</div>

<style>
    .icon {
        display: inline-block;
        color: currentColor;
    }
</style>

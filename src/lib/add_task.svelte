<script lang="ts">
    import type { Task } from "$lib";
    import { onMount } from "svelte";

    export let show = false;
    export let onClose = () => {};
    export let callback = (task: Task) => {};
    export let id: number;

    let task = {
        id: id,
        title: "",
        desc: "",
        link: "",
        repeat: false,
    };

    onMount(() => {
        if (show) {
            document.body.style.overflow = "hidden";
        }
        return () => {
            document.body.style.overflow = "auto";
        };
    });
</script>

{#if show}
    <div class="overlay">
        <div class="modal">
            <button class="close-button" onclick={onClose}>✕</button>
            <slot>
                <form onsubmit={() => callback(task)}>
                    <div class="field">
                        <label for="title">Task Title</label>
                        <input
                            type="text"
                            id="title"
                            bind:value={task.title}
                            required
                        />
                    </div>
                    <div class="field">
                        <label for="desc">Task Description</label>
                        <textarea id="desc" bind:value={task.desc} required
                        ></textarea>
                    </div>
                    <div class="field">
                        <label for="link">Task Link</label>
                        <input type="url" id="link" bind:value={task.link} />
                    </div>
                    <div class="field">
                        <label for="repeat">Repeat Task</label>
                        <input
                            type="checkbox"
                            id="repeat"
                            bind:checked={task.repeat}
                        />
                    </div>
                    <button type="submit">Add Task</button>
                </form>
            </slot>
        </div>
    </div>
{/if}

<style>
    .overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background-color: rgba(0, 0, 0, 0.8);
        z-index: 1000;
    }

    .modal {
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background-color: var(--primary-color);
        padding: 20px;
        border-radius: 8px;
        width: 500px;
        height: 0px;
        max-width: 400px;
        z-index: 1001;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
        display: flex;
        flex-direction: column;
    }

    .close-button {
        position: absolute;
        top: 10px;
        right: 10px;
        border: none;
        background: none;
        font-size: 1.2em;
        cursor: pointer;
    }
</style>

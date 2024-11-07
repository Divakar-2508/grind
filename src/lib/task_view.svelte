<script lang="ts">
    import type { Task } from "$lib";

    export let task_details: Task;
    export let callback: (id: number) => void;

    function go_to_link() {
        if(task_details.link) {
            window.open(task_details.link, "_blank");
        }
    }
</script>

<div class="task_container">
    <h3 class="fg_white title">{task_details.title}</h3>
    <p class="fg_white desc">{task_details.desc}</p>
    <div class="button_container">
        {#if task_details.link}
            <button onclick={go_to_link} class="bg_red">Go</button>
        {/if}
        <button class="bg_green" onclick={() => callback(task_details.id)}>Done</button>
    </div>
</div>

<style>
    .task_container {
        min-height: 50px;
        height: fit-content;
        width: 80%;
        background-color: var(--secondary-color);
        border: var(--primary-color) 2px solid;
        border-radius: 20px;
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0 10px;
        position: relative;
    }

    .title {
        position: absolute;
        top: 5px;
        left: 20px;
        margin: 0;
    }

    .desc {
        max-width: 600px;
        position: absolute;
        left: 30%;
    }

    .button_container {
        position: absolute;
        right: 10px;
        display: flex;
        flex-direction: row;
        gap: 10px;
        justify-content: space-evenly;
    }

    button {
        width: 50px;
        height: 25px;
        border: none;
        color: var(--fg-color);
        border-radius: 20px;
        text-align: center;
        text-decoration: none;
        font-size: 15px;
        margin: 4px 2px;
        cursor: pointer;
    }

    button:hover {
        opacity: 0.8;
        color: ghostwhite;
    }

    .bg_red {
        background-color: #E56565;
    }

    .bg_green {
        background-color: #6DD466;
    }
</style>

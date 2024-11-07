<script lang="ts">
  import type { Task } from "$lib/index";
  import AddTask from "$lib/add_task.svelte";
  import { onMount } from "svelte";
  import TaskView from "$lib/task_view.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let tasks: Task[] = [];
  let streak = 5;
  let task_left = 0;

  onMount(async () => {
    try {
      tasks = await invoke("get_tasks");
    } catch (e) {
      console.error(e);
    }
  });

  async function add_task(task: Task) {
    try {
      await invoke("add_task", task);
    } catch (e) {
      console.error(e);
    }
    tasks = [...tasks, task];
  }

  function delete_task(id: number) {
    tasks = tasks.filter((task) => task.id !== id);
  }

  let show_modal = false;
  function show_add_task() {
    show_modal = true;
  }

  function hide_add_task() {
    show_modal = false;
  }
</script>

<div class="app">
  <div class="notch">
    <div class="field fg_white">
      <p>🔥 {streak}</p>
    </div>
    <div class="field">
      <p>🍩 {task_left}</p>
    </div>
  </div>
  <button class="add_task" onclick={show_add_task}>Add Task</button>
  <div class="task_list">
    {#if show_modal}
      <AddTask
        show={show_modal}
        onClose={hide_add_task}
        callback={add_task}
        id={tasks.length}
      />
    {/if}
    {#each tasks as task (task.id)}
      <TaskView task_details={task} callback={delete_task} />
    {/each}
  </div>
</div>

<style>
  :root {
    --primary-color: #5c5c5c;
    --secondary-color: #3b3b3b;
    --fg-color: #e0e0e0;
  }

  .app {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20px;
  }

  .notch {
    width: 220px;
    height: 35px;
    background-color: #3b3b3b;
    border: var(--primary-color) 2px solid;
    border-bottom: black 2px solid;
    border-radius: 0px 0px 30px 30px;
    display: flex;
    justify-content: space-around;
    align-items: center;
  }

  .add_task {
    cursor: pointer;
    outline: none;
    position: absolute;
    top: 80px;
    width: 100px;
    height: 30px;
    border-radius: 30px;
    background-color: var(--primary-color);
    border: none;
    color: var(--fg-color);
  }

  .add_task:hover {
    transform: scale(1.1);
    transition: transform 0.2s ease-in-out;
  }

  .task_list {
    position: absolute;
    top: 180px;
    width: 100vw;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20px;
  }

  :global(.primary_bg) {
    background-color: var(--primary-color);
  }

  :global(.fg_white) {
    color: #e0e0e0;
  }
</style>

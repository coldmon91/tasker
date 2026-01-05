<script lang="ts">
  import {
    CheckCircle2,
    Circle,
    Plus,
    Settings,
    MoreVertical,
    ListTodo,
    Trash2,
    GripVertical,
    AlertCircle,
    ChevronDown,
    ChevronRight
  } from 'lucide-svelte';
  import { page } from '$app/state';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  type Priority = 'High' | 'Medium' | 'Low';

  interface Task {
    id: string;
    title: string;
    completed: boolean;
    priority: Priority;
    due_date?: string | null; // Rust Option<String> maps to string | null
    category: string;
    position: number;
  }

  let tasks = $state<Task[]>([]);
  let newTaskTitle = $state('');
  let draggedTaskId = $state<string | null>(null);
  let dragOverTaskId = $state<string | null>(null);
  let isDragging = $state(false);

  // Collapsible section states
  let isActiveCollapsed = $state(false);
  let isCompletedCollapsed = $state(false);

  async function loadTasks() {
    try {
      tasks = await invoke('get_tasks');
    } catch (e) {
      console.error('Failed to load tasks:', e);
    }
  }

  onMount(() => {
    loadTasks();
  });

  async function addTask() {
    if (newTaskTitle.trim()) {
      const maxPos = tasks.reduce((max, t) => Math.max(max, t.position), -1);
      const newTask: Task = {
        id: crypto.randomUUID(),
        title: newTaskTitle,
        completed: false,
        priority: 'Medium',
        category: 'In-box',
        due_date: null,
        position: maxPos + 1
      };
      
      try {
        await invoke('add_task', { task: newTask });
        tasks.push(newTask);
        newTaskTitle = '';
      } catch (e) {
        console.error('Failed to add task:', e);
      }
    }
  }

  async function toggleTask(id: string) {
    const taskIndex = tasks.findIndex(t => t.id === id);
    if (taskIndex !== -1) {
      const updatedTask = { ...tasks[taskIndex], completed: !tasks[taskIndex].completed };
      try {
        await invoke('update_task', { task: updatedTask });
        tasks[taskIndex] = updatedTask;
      } catch (e) {
        console.error('Failed to update task:', e);
      }
    }
  }

  async function deleteTask(id: string) {
    try {
      await invoke('delete_task', { id });
      tasks = tasks.filter(t => t.id !== id);
    } catch (e) {
      console.error('Failed to delete task:', e);
    }
  }

  function handleMouseDown(taskId: string, e: MouseEvent) {
    // 버튼 클릭은 무시
    if ((e.target as HTMLElement).closest('button')) {
      return;
    }
    console.log('[DEBUG] mousedown on task:', taskId);
    isDragging = true;
    draggedTaskId = taskId;
  }

  function handleMouseEnter(taskId: string) {
    if (isDragging && draggedTaskId && draggedTaskId !== taskId) {
      console.log('[DEBUG] mouse enter task:', taskId);
      dragOverTaskId = taskId;
    }
  }

  function handleMouseUp(e: MouseEvent) {
    console.log('[DEBUG] mouseup, isDragging:', isDragging, 'dragOverTaskId:', dragOverTaskId);
    if (isDragging && dragOverTaskId) {
      handleDrop(dragOverTaskId);
    }
    isDragging = false;
    draggedTaskId = null;
    dragOverTaskId = null;
  }

  async function handleDrop(targetId: string) {
    console.log('[DEBUG] handleDrop called with targetId:', targetId);
    console.log('[DEBUG] draggedTaskId:', draggedTaskId);

    if (!draggedTaskId || draggedTaskId === targetId) {
      console.log('[DEBUG] Early return - same task or no dragged task');
      return;
    }

    const originalIndex = tasks.findIndex(t => t.id === draggedTaskId);
    const targetIndex = tasks.findIndex(t => t.id === targetId);

    console.log('[DEBUG] originalIndex:', originalIndex, 'targetIndex:', targetIndex);

    if (originalIndex !== -1 && targetIndex !== -1) {
      // Reorder locally
      const newTasks = [...tasks];
      const [movedTask] = newTasks.splice(originalIndex, 1);
      newTasks.splice(targetIndex, 0, movedTask);

      console.log('[DEBUG] Reordered tasks locally');

      // Update positions for subsequent reloads consistency (optional but good for local logic)
      newTasks.forEach((t, i) => { t.position = i; });

      tasks = newTasks;

      console.log('[DEBUG] Updated tasks state, new order:', tasks.map(t => t.title));

      // Update order in backend
      try {
        const ordered_ids = tasks.map(t => t.id);
        console.log('[DEBUG] Calling update_task_order with ids:', ordered_ids);
        await invoke('update_task_order', { ordered_ids });
        console.log('[DEBUG] Successfully updated task order in backend');
      } catch (e) {
        console.error('Failed to update task order:', e);
      }
    }
    draggedTaskId = null;
    dragOverTaskId = null;
  }

  let currentFilter = $derived(() => {
    const filter = page.url.searchParams.get('filter') || 'All';
    return filter;
  });

  let activeTasks = $derived(() => tasks.filter(t => !t.completed));
  let completedTasks = $derived(() => tasks.filter(t => t.completed));
</script>

<header class="h-16 bg-white border-b border-gray-200 flex items-center justify-between px-8 flex-shrink-0">
  <h2 class="text-xl font-semibold">{currentFilter()} Tasks</h2>
  <div class="flex items-center gap-4">
    <span class="text-sm text-gray-500">{activeTasks().length} tasks remaining</span>
  </div>
</header>

<div class="flex-1 overflow-y-auto p-8">
  <div class="max-w-3xl mx-auto space-y-6">
    <!-- Add Task Input -->
    <div class="relative group">
      <input
        type="text"
        bind:value={newTaskTitle}
        placeholder="Add a new task..."
        onkeydown={(e) => e.key === 'Enter' && addTask()}
        class="w-full bg-white border border-gray-300 rounded-xl py-4 pl-12 pr-4 focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none transition-all shadow-sm group-hover:border-gray-400"
      />
      <Plus class="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400" size={24} />
    </div>

    <!-- Task List -->
    <div
      class="space-y-8"
      ondragover={(e) => {
        console.log('[DEBUG] dragover on container');
      }}
      ondrop={(e) => {
        console.log('[DEBUG] drop on container (outside cards)');
      }}
    >
      {#snippet taskCard(task: Task)}
        <div
          onmousedown={(e) => handleMouseDown(task.id, e)}
          onmouseenter={() => handleMouseEnter(task.id)}
          onmouseup={handleMouseUp}
          ondblclick={() => goto(`/task/${task.id}`)}
          class="group flex items-center gap-4 bg-white p-4 rounded-xl border border-gray-200 shadow-sm transition-all cursor-grab active:cursor-grabbing select-none
          {draggedTaskId === task.id ? 'opacity-40 ring-2 ring-indigo-400 border-indigo-400 border-dashed' : ''}
          {dragOverTaskId === task.id && draggedTaskId !== task.id ? 'border-indigo-500 ring-2 ring-indigo-200 transform scale-[1.02]' : 'hover:border-indigo-200 hover:shadow-md'}"
        >
          <!-- Drag Handle (visual indicator) -->
          <div class="text-gray-300 hover:text-gray-500 flex-shrink-0">
            <GripVertical size={20} />
          </div>
          <button
            onclick={(e) => { e.stopPropagation(); toggleTask(task.id); }}
            class="text-gray-400 hover:text-indigo-600 transition-colors flex-shrink-0 cursor-pointer">
            {#if task.completed}
              <CheckCircle2 class="text-green-500" size={24} />
            {:else}
              <Circle size={24} />
            {/if}
          </button>
          
          <div class="flex-1 min-w-0">
            <p class="text-lg {task.completed ? 'line-through text-gray-400' : 'text-gray-900'} truncate font-medium">
              {task.title}
            </p>
            <div class="flex items-center gap-3 mt-1">
              <span class="text-[8px] uppercase tracking-tight font-semibold px-1.5 py-0.5 rounded flex items-center gap-1 {
                task.priority === 'High' ? 'bg-red-50 text-red-600 border border-red-100' : 
                task.priority === 'Medium' ? 'bg-orange-50 text-orange-600 border border-orange-100' : 
                'bg-blue-50 text-blue-600 border border-blue-100'
              }">
                <AlertCircle size={10} strokeWidth={2.5} />
                {task.priority}
              </span>
              <span class="text-xs text-gray-400 flex items-center gap-1">
                <div class="w-1.5 h-1.5 rounded-full bg-gray-300"></div>
                {task.category}
              </span>
            </div>
          </div>

          <div class="flex items-center gap-2 opacity-0 group-hover:opacity-100 transition-all">
            <button
              onclick={(e) => { e.stopPropagation(); deleteTask(task.id); }}
              class="text-gray-400 hover:text-red-500 transition-all p-2 rounded-lg hover:bg-red-50">
              <Trash2 size={18} />
            </button>
          </div>
        </div>
      {/snippet}

      <!-- Active Section -->
      {#if currentFilter() === 'All' || currentFilter() === 'Active'}
        <div class="space-y-3">
          {#if currentFilter() === 'All' && activeTasks().length > 0}
            <button 
              onclick={() => isActiveCollapsed = !isActiveCollapsed}
              class="flex items-center gap-2 px-1 py-1 w-full hover:bg-gray-50 rounded-lg transition-colors group/header"
            >
              {#if isActiveCollapsed}
                <ChevronRight size={16} class="text-gray-400 group-hover/header:text-gray-600" />
              {:else}
                <ChevronDown size={16} class="text-gray-400 group-hover/header:text-gray-600" />
              {/if}
              <h3 class="text-xs font-semibold text-gray-500 uppercase tracking-wider flex items-center gap-2">
                Active
                <span class="text-[10px] bg-gray-100 px-1.5 py-0.5 rounded-full text-gray-400">{activeTasks().length}</span>
              </h3>
            </button>
          {/if}
          
          {#if !isActiveCollapsed || currentFilter() === 'Active'}
            {#each activeTasks() as task (task.id)}
              {@render taskCard(task)}
            {:else}
              {#if currentFilter() === 'Active'}
                <div class="text-center py-20">
                  <div class="bg-indigo-50 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                    <ListTodo size={32} class="text-indigo-600" />
                  </div>
                  <h3 class="text-lg font-medium text-gray-900">No active tasks!</h3>
                  <p class="text-gray-500">Enjoy your free time or add a new task.</p>
                </div>
              {/if}
            {/each}
          {/if}
        </div>
      {/if}

      <!-- Completed Section -->
      {#if currentFilter() === 'All' || currentFilter() === 'Completed'}
        <div class="space-y-3">
          {#if currentFilter() === 'All' && completedTasks().length > 0}
            <button 
              onclick={() => isCompletedCollapsed = !isCompletedCollapsed}
              class="flex items-center gap-2 px-1 py-1 w-full hover:bg-gray-50 rounded-lg transition-colors group/header mt-4"
            >
              {#if isCompletedCollapsed}
                <ChevronRight size={16} class="text-gray-400 group-hover/header:text-gray-600" />
              {:else}
                <ChevronDown size={16} class="text-gray-400 group-hover/header:text-gray-600" />
              {/if}
              <div class="flex items-center gap-2">
                <h3 class="text-xs font-semibold text-gray-500 uppercase tracking-wider">Completed</h3>
                <span class="text-[10px] bg-gray-100 px-1.5 py-0.5 rounded-full text-gray-400">{completedTasks().length}</span>
              </div>
            </button>
          {/if}

          {#if !isCompletedCollapsed || currentFilter() === 'Completed'}
            {#each completedTasks() as task (task.id)}
              {@render taskCard(task)}
            {:else}
              {#if currentFilter() === 'Completed'}
                <div class="text-center py-20">
                  <div class="bg-indigo-50 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                    <CheckCircle2 size={32} class="text-indigo-600" />
                  </div>
                  <h3 class="text-lg font-medium text-gray-900">No completed tasks yet</h3>
                  <p class="text-gray-500">Get some work done and check them off!</p>
                </div>
              {/if}
            {/each}
          {/if}
        </div>
      {/if}

      {#if currentFilter() === 'All' && activeTasks().length === 0 && completedTasks().length === 0}
        <div class="text-center py-20">
          <div class="bg-indigo-50 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
            <ListTodo size={32} class="text-indigo-600" />
          </div>
          <h3 class="text-lg font-medium text-gray-900">All caught up!</h3>
          <p class="text-gray-500">No tasks found. Start by adding one above!</p>
        </div>
      {/if}
    </div>
  </div>
</div>
<script lang="ts">
  import { 
    CheckCircle2, 
    Circle, 
    Plus, 
    Settings,
    MoreVertical,
    ListTodo,
    Trash2
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
  }

  let tasks = $state<Task[]>([]);
  let newTaskTitle = $state('');

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
      const newTask: Task = {
        id: crypto.randomUUID(),
        title: newTaskTitle,
        completed: false,
        priority: 'Medium',
        category: 'In-box',
        due_date: null
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

  let currentFilter = $derived(() => {
    const filter = page.url.searchParams.get('filter') || 'All';
    return filter;
  });

  let filteredTasks = $derived(() => {
    const filter = currentFilter();
    if (filter === 'Completed') {
      return tasks.filter(t => t.completed);
    }
    if (filter === 'Active') {
      return tasks.filter(t => !t.completed);
    }
    return tasks;
  });
</script>

<header class="h-16 bg-white border-b border-gray-200 flex items-center justify-between px-8 flex-shrink-0">
  <h2 class="text-xl font-semibold">{currentFilter()} Tasks</h2>
  <div class="flex items-center gap-4">
    <span class="text-sm text-gray-500">{tasks.filter(t => !t.completed).length} tasks remaining</span>
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
    <div class="space-y-3">
      {#each filteredTasks() as task (task.id)}
        <div class="group flex items-center gap-4 bg-white p-4 rounded-xl border border-gray-200 shadow-sm hover:border-indigo-200 hover:shadow-md transition-all">
          <button 
            onclick={() => toggleTask(task.id)}
            class="text-gray-400 hover:text-indigo-600 transition-colors flex-shrink-0">
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
              <span class="text-[10px] uppercase tracking-wider font-bold px-2 py-0.5 rounded-md {
                task.priority === 'High' ? 'bg-red-50 text-red-600 border border-red-100' : 
                task.priority === 'Medium' ? 'bg-orange-50 text-orange-600 border border-orange-100' : 
                'bg-blue-50 text-blue-600 border border-blue-100'
              }">
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
              onclick={() => deleteTask(task.id)}
              class="text-gray-400 hover:text-red-500 transition-all p-2 rounded-lg hover:bg-red-50">
              <Trash2 size={18} />
            </button>
            <button 
              onclick={() => goto(`/task/${task.id}`)}
              class="text-gray-400 hover:text-indigo-600 transition-all p-2 rounded-lg hover:bg-indigo-50">
              <MoreVertical size={18} />
            </button>
          </div>
        </div>
      {:else}
        <div class="text-center py-20">
          <div class="bg-indigo-50 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
            <ListTodo size={32} class="text-indigo-600" />
          </div>
          <h3 class="text-lg font-medium text-gray-900">All caught up!</h3>
          <p class="text-gray-500">No tasks found for this view.</p>
        </div>
      {/each}
    </div>
  </div>
</div>
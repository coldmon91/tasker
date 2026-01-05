<script lang="ts">
  import { page } from '$app/state';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { 
    ChevronLeft, 
    Save, 
    Trash2,
    Calendar as CalendarIcon,
    Tag,
    AlertCircle,
    Type
  } from 'lucide-svelte';
  import { goto } from '$app/navigation';

  type Priority = 'High' | 'Medium' | 'Low';

  interface Task {
    id: string;
    title: string;
    completed: boolean;
    priority: Priority;
    due_date?: string | null;
    category: string;
    position: number;
  }

  let task = $state<Task | null>(null);
  let loading = $state(true);

  const taskId = page.params.id;

  onMount(async () => {
    try {
      task = await invoke('get_task', { id: taskId });
    } catch (e) {
      console.error('Failed to load task:', e);
    } finally {
      loading = false;
    }
  });

  async function saveTask() {
    if (task) {
      try {
        await invoke('update_task', { task });
        goto('/');
      } catch (e) {
        console.error('Failed to save task:', e);
      }
    }
  }

  async function deleteTask() {
    if (confirm('Are you sure you want to delete this task?')) {
      try {
        await invoke('delete_task', { id: taskId });
        goto('/');
      } catch (e) {
        console.error('Failed to delete task:', e);
      }
    }
  }
</script>

<div class="flex flex-col h-full bg-gray-50">
  <header class="h-16 bg-white border-b border-gray-200 flex items-center justify-between px-8 flex-shrink-0">
    <div class="flex items-center gap-4">
      <button 
        onclick={() => goto('/')}
        class="p-2 hover:bg-gray-100 rounded-lg transition-all text-gray-500">
        <ChevronLeft size={24} />
      </button>
      <h2 class="text-xl font-semibold text-gray-900">Task Details</h2>
    </div>
    <div class="flex items-center gap-3">
      <button 
        onclick={deleteTask}
        class="p-2 hover:bg-red-50 text-red-500 rounded-lg transition-all">
        <Trash2 size={20} />
      </button>
      <button 
        onclick={saveTask}
        class="flex items-center gap-2 px-4 py-2 bg-indigo-600 text-white rounded-lg font-medium hover:bg-indigo-700 transition-all shadow-sm">
        <Save size={18} />
        <span>Save Changes</span>
      </button>
    </div>
  </header>

  <div class="flex-1 overflow-y-auto p-8">
    <div class="max-w-2xl mx-auto">
      {#if loading}
        <div class="flex items-center justify-center py-20">
          <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600"></div>
        </div>
      {:else if task}
        <div class="space-y-8 bg-white p-8 rounded-2xl border border-gray-200 shadow-sm">
          <!-- Title -->
          <div class="space-y-2">
            <label for="title" class="text-sm font-semibold text-gray-700 flex items-center gap-2">
              <Type size={16} class="text-gray-400" />
              Title
            </label>
            <input 
              id="title"
              type="text" 
              bind:value={task.title}
              class="w-full text-2xl font-bold border-none focus:ring-0 p-0 placeholder-gray-300"
              placeholder="What needs to be done?"
            />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
            <!-- Priority -->
            <div class="space-y-2">
              <label for="priority" class="text-xs font-semibold text-gray-500 flex items-center gap-2 uppercase tracking-wider">
                <AlertCircle size={14} class="text-gray-400" />
                Priority
              </label>
              <select 
                id="priority"
                bind:value={task.priority}
                class="w-full bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all outline-none text-sm"
              >
                <option value="High">High</option>
                <option value="Medium">Medium</option>
                <option value="Low">Low</option>
              </select>
            </div>

            <!-- Category -->
            <div class="space-y-2">
              <label for="category" class="text-xs font-semibold text-gray-500 flex items-center gap-2 uppercase tracking-wider">
                <Tag size={14} class="text-gray-400" />
                Category
              </label>
              <input 
                id="category"
                type="text" 
                bind:value={task.category}
                placeholder="e.g. Work, Personal"
                class="w-full bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all outline-none text-sm"
              />
            </div>

            <!-- Due Date -->
            <div class="space-y-2 md:col-span-2">
              <label for="due_date" class="text-xs font-semibold text-gray-500 flex items-center gap-2 uppercase tracking-wider">
                <CalendarIcon size={14} class="text-gray-400" />
                Due Date
              </label>
              <input 
                id="due_date"
                type="date" 
                bind:value={task.due_date}
                class="w-full bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all outline-none text-sm"
              />
            </div>
          </div>

          <!-- Status toggle -->
          <div class="pt-6 border-t border-gray-100 flex items-center justify-between">
            <span class="text-sm font-medium text-gray-700">Mark as completed</span>
            <button 
              onclick={() => task && (task.completed = !task.completed)}
              class="w-12 h-6 rounded-full transition-colors relative {task.completed ? 'bg-green-500' : 'bg-gray-200'}">
              <div class="absolute top-1 left-1 bg-white w-4 h-4 rounded-full transition-transform {task.completed ? 'translate-x-6' : ''}"></div>
            </button>
          </div>
        </div>
      {:else}
        <div class="text-center py-20">
          <p class="text-gray-500">Task not found.</p>
          <button onclick={() => goto('/')} class="mt-4 text-indigo-600 font-medium">Back to tasks</button>
        </div>
      {/if}
    </div>
  </div>
</div>

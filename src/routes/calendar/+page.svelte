<script lang="ts">
  import { 
    ChevronLeft, 
    ChevronRight,
    Calendar as CalendarIcon,
    ListTodo
  } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  const daysOfWeek = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
  const monthNames = [
    'January', 'February', 'March', 'April', 'May', 'June',
    'July', 'August', 'September', 'October', 'November', 'December'
  ];

  type Priority = 'High' | 'Medium' | 'Low';

  interface Task {
    id: string;
    title: string;
    completed: boolean;
    priority: Priority;
    due_date?: string | null;
    category: string;
  }

  let tasks = $state<Task[]>([]);
  let today = new Date();
  let currentMonth = $state(today.getMonth());
  let currentYear = $state(today.getFullYear());

  onMount(async () => {
    try {
      tasks = await invoke('get_tasks');
    } catch (e) {
      console.error('Failed to load tasks:', e);
    }
  });

  let daysInMonth = $derived(() => {
    const date = new Date(currentYear, currentMonth, 1);
    const days = [];
    
    // Previous month padding
    const firstDayIndex = date.getDay();
    const prevMonthDate = new Date(currentYear, currentMonth, 0);
    const prevMonthLastDay = prevMonthDate.getDate();
    const prevMonth = prevMonthDate.getMonth();
    const prevYear = prevMonthDate.getFullYear();

    for (let i = firstDayIndex - 1; i >= 0; i--) {
      days.push({ 
        day: prevMonthLastDay - i, 
        month: prevMonth,
        year: prevYear,
        current: false,
        isToday: false
      });
    }

    // Current month days
    const lastDate = new Date(currentYear, currentMonth + 1, 0).getDate();
    for (let i = 1; i <= lastDate; i++) {
      days.push({ 
        day: i,
        month: currentMonth,
        year: currentYear, 
        current: true, 
        isToday: i === today.getDate() && currentMonth === today.getMonth() && currentYear === today.getFullYear()
      });
    }

    // Next month padding
    const remaining = 42 - days.length; // 6 rows of 7 days
    const nextMonthDate = new Date(currentYear, currentMonth + 1, 1);
    const nextMonth = nextMonthDate.getMonth();
    const nextYear = nextMonthDate.getFullYear();

    for (let i = 1; i <= remaining; i++) {
      days.push({ 
        day: i, 
        month: nextMonth,
        year: nextYear,
        current: false,
        isToday: false
      });
    }

    return days;
  });

  function nextMonth() {
    if (currentMonth === 11) {
      currentMonth = 0;
      currentYear++;
    } else {
      currentMonth++;
    }
  }

  function prevMonth() {
    if (currentMonth === 0) {
      currentMonth = 11;
      currentYear--;
    } else {
      currentMonth--;
    }
  }

  function getTasksForDate(year: number, month: number, day: number) {
    // Format date as YYYY-MM-DD to match HTML date input format
    const monthStr = String(month + 1).padStart(2, '0');
    const dayStr = String(day).padStart(2, '0');
    const dateStr = `${year}-${monthStr}-${dayStr}`;
    
    return tasks.filter(t => t.due_date === dateStr);
  }
</script>

<div class="flex flex-col h-full bg-white">
  <header class="h-16 border-b border-gray-200 flex items-center justify-between px-8 bg-white flex-shrink-0">
    <div class="flex items-center gap-4">
      <h2 class="text-xl font-bold text-gray-900">{monthNames[currentMonth]} {currentYear}</h2>
      <div class="flex items-center bg-gray-100 rounded-lg p-1">
        <button onclick={prevMonth} class="p-1 hover:bg-white rounded transition-all">
          <ChevronLeft size={20} />
        </button>
        <button onclick={nextMonth} class="p-1 hover:bg-white rounded transition-all">
          <ChevronRight size={20} />
        </button>
      </div>
      <button 
        onclick={() => { currentMonth = today.getMonth(); currentYear = today.getFullYear(); }}
        class="text-sm font-medium px-3 py-1 bg-white border border-gray-200 rounded-lg hover:bg-gray-50 transition-all">
        Today
      </button>
    </div>
    
    <div class="flex items-center gap-2">
      <!-- Future: Add Event Button -->
    </div>
  </header>

  <div class="flex-1 overflow-auto flex flex-col">
    <div class="grid grid-cols-7 border-b border-gray-200 bg-gray-50 flex-shrink-0">
      {#each daysOfWeek as day}
        <div class="py-2 text-center text-xs font-semibold text-gray-500 uppercase tracking-wider">
          {day}
        </div>
      {/each}
    </div>

    <div class="grid grid-cols-7 flex-1 auto-rows-fr">
      {#each daysInMonth() as { day, month, year, current, isToday }}
        <div class="border-r border-b border-gray-100 p-2 hover:bg-gray-50 transition-colors group flex flex-col gap-1 overflow-hidden">
          <div class="flex items-center justify-between flex-shrink-0">
            <span class="text-sm font-medium {current ? 'text-gray-900' : 'text-gray-300'} {isToday ? 'bg-indigo-600 text-white w-7 h-7 flex items-center justify-center rounded-full' : ''}">
              {day}
            </span>
          </div>
          
          <div class="flex-1 overflow-y-auto space-y-1 custom-scrollbar">
            {#each getTasksForDate(year, month, day) as task}
              <button 
                onclick={() => goto(`/task/${task.id}`)}
                class="w-full text-left text-[10px] px-1.5 py-1 rounded border truncate transition-all 
                {task.completed ? 'bg-gray-50 text-gray-400 border-gray-100 line-through' : 'bg-indigo-50 text-indigo-700 border-indigo-100 hover:border-indigo-300'}">
                {task.title}
              </button>
            {/each}
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  /* Minimal scrollbar for calendar cells */
  .custom-scrollbar::-webkit-scrollbar {
    width: 2px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background-color: #e5e7eb;
    border-radius: 20px;
  }
</style>
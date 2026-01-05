<script lang="ts">
  import { 
    ChevronLeft, 
    ChevronRight,
    Calendar as CalendarIcon,
    ListTodo
  } from 'lucide-svelte';

  const daysOfWeek = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
  const monthNames = [
    'January', 'February', 'March', 'April', 'May', 'June',
    'July', 'August', 'September', 'October', 'November', 'December'
  ];

  let today = new Date();
  let currentMonth = $state(today.getMonth());
  let currentYear = $state(today.getFullYear());

  let daysInMonth = $derived(() => {
    const date = new Date(currentYear, currentMonth, 1);
    const days = [];
    
    // Previous month padding
    const firstDayIndex = date.getDay();
    const prevMonthLastDate = new Date(currentYear, currentMonth, 0).getDate();
    for (let i = firstDayIndex - 1; i >= 0; i--) {
      days.push({ day: prevMonthLastDate - i, current: false });
    }

    // Current month days
    const lastDate = new Date(currentYear, currentMonth + 1, 0).getDate();
    for (let i = 1; i <= lastDate; i++) {
      days.push({ 
        day: i, 
        current: true, 
        isToday: i === today.getDate() && currentMonth === today.getMonth() && currentYear === today.getFullYear()
      });
    }

    // Next month padding
    const remaining = 42 - days.length; // 6 rows of 7 days
    for (let i = 1; i <= remaining; i++) {
      days.push({ day: i, current: false });
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
</script>

<div class="flex flex-col h-full bg-white">
  <header class="h-16 border-b border-gray-200 flex items-center justify-between px-8 bg-white">
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
      <button class="px-4 py-2 bg-indigo-600 text-white rounded-lg text-sm font-medium hover:bg-indigo-700 transition-all">
        Add Event
      </button>
    </div>
  </header>

  <div class="flex-1 overflow-auto">
    <div class="grid grid-cols-7 border-b border-gray-200 bg-gray-50">
      {#each daysOfWeek as day}
        <div class="py-2 text-center text-xs font-semibold text-gray-500 uppercase tracking-wider">
          {day}
        </div>
      {/each}
    </div>

    <div class="grid grid-cols-7 h-full min-h-[600px]">
      {#each daysInMonth() as { day, current, isToday }}
        <div class="border-r border-b border-gray-100 p-2 min-h-[100px] hover:bg-gray-50 transition-colors group">
          <div class="flex items-center justify-between">
            <span class="text-sm font-medium {current ? 'text-gray-900' : 'text-gray-300'} {isToday ? 'bg-indigo-600 text-white w-7 h-7 flex items-center justify-center rounded-full' : ''}">
              {day}
            </span>
          </div>
          
          <!-- Mock events -->
          {#if isToday}
            <div class="mt-2 space-y-1">
              <div class="text-[10px] bg-indigo-50 text-indigo-700 p-1 rounded border border-indigo-100 truncate">
                Tauri App Demo
              </div>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </div>
</div>

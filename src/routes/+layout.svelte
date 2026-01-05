<script lang="ts">
  import "../app.css";
  import { 
    CheckCircle2, 
    Circle, 
    Plus, 
    Calendar as CalendarIcon, 
    ListTodo, 
    Settings,
    Star,
    Clock,
    Search,
    ChevronLeft,
    ChevronRight
  } from 'lucide-svelte';
  import { page } from '$app/state';

  let { children } = $props();
  let isSidebarCollapsed = $state(false);

  const navItems = [
    { name: 'Tasks', path: '/', icon: ListTodo },
    { name: 'Calendar', path: '/calendar', icon: CalendarIcon },
    { name: 'Active', path: '/?filter=Active', icon: Clock },
    { name: 'Completed', path: '/?filter=Completed', icon: CheckCircle2 },
  ];
</script>

<div class="flex h-screen bg-gray-50 text-gray-900 font-sans overflow-hidden">
  <!-- Sidebar -->
  <aside class="{isSidebarCollapsed ? 'w-20' : 'w-64'} bg-white border-r border-gray-200 flex flex-col flex-shrink-0 transition-all duration-300 relative">
    <div class="p-6 flex items-center justify-between h-[88px]">
      {#if !isSidebarCollapsed}
        <h1 class="text-2xl font-bold text-indigo-600 flex items-center gap-2 whitespace-nowrap overflow-hidden">
          <ListTodo size={28} /> Tasker
        </h1>
      {:else}
        <div class="w-full flex justify-center">
           <ListTodo size={28} class="text-indigo-600" />
        </div>
      {/if}
      
      <button 
        onclick={() => isSidebarCollapsed = !isSidebarCollapsed} 
        class="text-gray-400 hover:text-indigo-600 transition-all {!isSidebarCollapsed ? '' : 'absolute right-[-12px] top-9 bg-white border border-gray-200 rounded-full p-1 shadow-sm z-10'}"
      >
        {#if !isSidebarCollapsed}
          <ChevronLeft size={20} />
        {:else}
          <ChevronRight size={16} />
        {/if}
      </button>
    </div>

    <div class="px-4 mb-6 {isSidebarCollapsed ? 'hidden' : 'block'}">
      <div class="relative">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" size={16} />
        <input 
          type="text" 
          placeholder="Search..." 
          class="w-full bg-gray-100 border-none rounded-lg py-2 pl-9 pr-4 text-sm focus:ring-2 focus:ring-indigo-500 transition-all outline-none"
        />
      </div>
    </div>
    
    <nav class="flex-1 px-4 space-y-1">
      {#each navItems as item}
        <a 
          href={item.path}
          class="w-full flex items-center {isSidebarCollapsed ? 'justify-center px-0' : 'gap-3 px-3'} py-2 rounded-lg transition-colors {page.url.pathname === item.path ? 'bg-indigo-50 text-indigo-600 font-medium' : 'text-gray-600 hover:bg-gray-100'}"
          title={isSidebarCollapsed ? item.name : ''}
        >
          <item.icon size={20} />
          {#if !isSidebarCollapsed}
            <span class="whitespace-nowrap overflow-hidden">{item.name}</span>
          {/if}
        </a>
      {/each}
      
      <div class="mt-8 pt-4 border-t border-gray-100">
        {#if !isSidebarCollapsed}
          <h3 class="px-3 text-xs font-semibold text-gray-400 uppercase tracking-wider mb-2">Categories</h3>
        {/if}
        <button 
          class="w-full flex items-center {isSidebarCollapsed ? 'justify-center px-0' : 'gap-3 px-3'} py-2 rounded-lg text-gray-600 hover:bg-gray-100 transition-colors"
          title={isSidebarCollapsed ? 'Work' : ''}
        >
          <div class="w-2 h-2 rounded-full bg-red-400 flex-shrink-0"></div>
          {#if !isSidebarCollapsed}
            <span class="whitespace-nowrap overflow-hidden">Work</span>
          {/if}
        </button>
        <button 
          class="w-full flex items-center {isSidebarCollapsed ? 'justify-center px-0' : 'gap-3 px-3'} py-2 rounded-lg text-gray-600 hover:bg-gray-100 transition-colors"
          title={isSidebarCollapsed ? 'Personal' : ''}
        >
          <div class="w-2 h-2 rounded-full bg-blue-400 flex-shrink-0"></div>
          {#if !isSidebarCollapsed}
            <span class="whitespace-nowrap overflow-hidden">Personal</span>
          {/if}
        </button>
      </div>
    </nav>

    <div class="p-4 border-t border-gray-200">
      <a 
        href="/settings"
        class="w-full flex items-center {isSidebarCollapsed ? 'justify-center px-0' : 'gap-3 px-3'} py-2 rounded-lg transition-colors {page.url.pathname === '/settings' ? 'bg-indigo-50 text-indigo-600 font-medium' : 'text-gray-600 hover:bg-gray-100'}"
        title={isSidebarCollapsed ? 'Settings' : ''}
      >
        <Settings size={20} />
        {#if !isSidebarCollapsed}
          <span>Settings</span>
        {/if}
      </a>
    </div>
  </aside>

  <!-- Main Content Area -->
  <main class="flex-1 flex flex-col min-w-0 overflow-y-auto relative">
    {@render children()}
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
  }
</style>
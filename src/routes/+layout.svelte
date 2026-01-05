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
    Search
  } from 'lucide-svelte';
  import { page } from '$app/state';

  let { children } = $props();

  const navItems = [
    { name: 'Tasks', path: '/', icon: ListTodo },
    { name: 'Calendar', path: '/calendar', icon: CalendarIcon },
    { name: 'Active', path: '/?filter=Active', icon: Clock },
    { name: 'Completed', path: '/?filter=Completed', icon: CheckCircle2 },
  ];
</script>

<div class="flex h-screen bg-gray-50 text-gray-900 font-sans overflow-hidden">
  <!-- Sidebar -->
  <aside class="w-64 bg-white border-r border-gray-200 flex flex-col flex-shrink-0">
    <div class="p-6">
      <h1 class="text-2xl font-bold text-indigo-600 flex items-center gap-2">
        <ListTodo size={28} /> Tasker
      </h1>
    </div>

    <div class="px-4 mb-6">
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
          class="w-full flex items-center gap-3 px-3 py-2 rounded-lg transition-colors {page.url.pathname === item.path ? 'bg-indigo-50 text-indigo-600 font-medium' : 'text-gray-600 hover:bg-gray-100'}">
          <item.icon size={20} />
          <span>{item.name}</span>
        </a>
      {/each}
      
      <div class="mt-8 pt-4 border-t border-gray-100">
        <h3 class="px-3 text-xs font-semibold text-gray-400 uppercase tracking-wider mb-2">Categories</h3>
        <button class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-gray-600 hover:bg-gray-100 transition-colors">
          <div class="w-2 h-2 rounded-full bg-red-400"></div>
          <span>Work</span>
        </button>
        <button class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-gray-600 hover:bg-gray-100 transition-colors">
          <div class="w-2 h-2 rounded-full bg-blue-400"></div>
          <span>Personal</span>
        </button>
      </div>
    </nav>

    <div class="p-4 border-t border-gray-200">
      <button class="w-full flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-gray-100 transition-colors text-gray-500">
        <Settings size={20} />
        <span>Settings</span>
      </button>
    </div>
  </aside>

  <!-- Main Content Area -->
  <main class="flex-1 flex flex-col min-w-0 overflow-hidden relative">
    {@render children()}
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
  }
</style>
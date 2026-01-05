<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { CheckCircle2, Loader2, RefreshCw, Download, Eye, EyeOff, User, Smartphone, Globe, ChevronRight, ChevronDown, LogOut } from 'lucide-svelte';
  import { onMount } from 'svelte';

  let connecting = $state(false);
  let isConnected = $state(false);
  let googleUser: { name: string, email: string, picture: string | null } | null = $state(null);
  let taskLists: { id: string, title: string, updated: string }[] = $state([]);
  let loadingLists = $state(false);
  let importStatus: Record<string, string> = $state({});

  // UI States
  let activeSection = $state<'google' | 'apple' | null>(null);

  async function startAuth() {
    try {
      connecting = true;
      const authUrl = await invoke('get_google_auth_url');
      
      // Open browser for authentication
      await openUrl(authUrl);
      
      // Start OAuth callback server and wait for auth code
      googleUser = await invoke('complete_google_auth');
      isConnected = true;
      connecting = false;
      fetchLists();
    } catch (e) {
      console.error('Failed to complete auth:', e);
      alert('Authentication failed: ' + e);
      connecting = false;
    }
  }

  async function fetchLists() {
    loadingLists = true;
    try {
      taskLists = await invoke('get_google_task_lists');
    } catch (e) {
      console.error('Failed to fetch lists:', e);
    } finally {
      loadingLists = false;
    }
  }

  async function importList(listId: string) {
    importStatus[listId] = 'importing';
    try {
      const count = await invoke('import_google_tasks', { listId });
      importStatus[listId] = `Imported ${count} tasks`;
    } catch (e) {
      console.error('Import failed:', e);
      importStatus[listId] = 'Failed: ' + e;
    }
  }

  async function checkLogin() {
    try {
      googleUser = await invoke('get_google_user');
      isConnected = true;
      fetchLists();
    } catch(e) {
      console.log("Not logged in or token expired");
      isConnected = false;
    }
  }

  onMount(() => {
    checkLogin();
  });
</script>

<div class="p-8 max-w-4xl mx-auto space-y-8">
  <header>
    <h1 class="text-3xl font-bold text-gray-900">Settings</h1>
    <p class="text-gray-500 mt-2">Manage your account and third-party integrations.</p>
  </header>

  <!-- Tasker Account -->
  <section class="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
    <div class="p-6 border-b border-gray-100 flex items-center gap-3">
      <div class="p-2 bg-indigo-100 rounded-lg text-indigo-600">
        <User size={20} />
      </div>
      <div>
        <h2 class="text-lg font-semibold text-gray-900">Tasker Account</h2>
        <p class="text-sm text-gray-500">Log in to sync your data across devices.</p>
      </div>
    </div>
    <div class="p-6">
      <button class="w-full sm:w-auto px-4 py-2 bg-indigo-600 text-white rounded-lg font-medium hover:bg-indigo-700 transition-colors flex items-center justify-center gap-2">
        Sign in to Tasker
      </button>
    </div>
  </section>

  <!-- Integrations -->
  <section class="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
    <div class="p-6 border-b border-gray-100">
      <h2 class="text-xl font-semibold text-gray-900">Integrations</h2>
      <p class="text-sm text-gray-500 mt-1">Connect with other services to import tasks.</p>
    </div>

    <div class="divide-y divide-gray-100">
      
      <!-- Google Account -->
      <div>
        <button 
          onclick={() => activeSection = activeSection === 'google' ? null : 'google'}
          class="w-full flex items-center justify-between p-6 hover:bg-gray-50 transition-colors text-left">
          <div class="flex items-center gap-4">
            <div class="p-2 bg-blue-100 rounded-lg text-blue-600">
              <Globe size={24} />
            </div>
            <div>
              <h3 class="text-base font-semibold text-gray-900 flex items-center gap-2">
                Google Account
                {#if isConnected && googleUser}
                  <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full bg-green-100 text-green-700 text-xs font-medium">
                    <CheckCircle2 size={12} /> {googleUser.email}
                  </span>
                {/if}
              </h3>
              <p class="text-sm text-gray-500">Sync Tasks, Calendar and more</p>
            </div>
          </div>
          <div class="text-gray-400">
            {#if activeSection === 'google'}
              <ChevronDown size={20} />
            {:else}
              <ChevronRight size={20} />
            {/if}
          </div>
        </button>

        {#if activeSection === 'google'}
          <div class="px-6 pb-6 pt-2 bg-gray-50/50">
             {#if !isConnected && !connecting}
              <div class="bg-white p-6 rounded-lg border border-gray-200 shadow-sm text-center">
                 <div class="mb-4 text-gray-600">
                   Connect your Google account to sync Tasks and Calendar.
                 </div>
                 <button 
                    onclick={startAuth}
                    class="px-6 py-2 bg-white border border-gray-300 rounded-lg text-gray-700 font-medium hover:bg-gray-50 transition-colors shadow-sm inline-flex items-center gap-2">
                    <img src="https://www.google.com/favicon.ico" alt="Google" class="w-4 h-4" />
                    Connect with Google
                  </button>
              </div>
            {/if}

            {#if connecting && !isConnected}
              <div class="bg-indigo-50 p-4 rounded-lg border border-indigo-100 mt-4 text-center">
                <div class="flex items-center justify-center gap-2 mb-2">
                  <Loader2 size={20} class="animate-spin text-indigo-600" />
                  <p class="text-sm font-medium text-indigo-900">Waiting for authentication...</p>
                </div>
                <p class="text-xs text-indigo-700">
                  Please complete the authentication in your browser window.
                </p>
              </div>
            {/if}

            {#if isConnected}
              <div class="mt-4 space-y-4">
                <!-- Connected Features -->
                <div class="bg-white p-4 rounded-lg border border-gray-200">
                  <h4 class="font-medium text-gray-900 mb-3">Enabled Features</h4>
                  <ul class="space-y-3">
                    <li class="flex items-start gap-3">
                       <div class="mt-0.5 text-green-500"><CheckCircle2 size={16} /></div>
                       <div class="flex-1">
                          <div class="flex items-center justify-between">
                            <span class="font-medium text-gray-800">Tasks</span>
                            <button 
                              onclick={fetchLists}
                              class="text-gray-400 hover:text-indigo-600 transition-colors"
                              title="Refresh Lists">
                              <RefreshCw size={14} class={loadingLists ? "animate-spin" : ""} />
                            </button>
                          </div>
                          
                          <!-- Task Lists -->
                          {#if loadingLists && taskLists.length === 0}
                             <div class="text-xs text-gray-400 mt-1">Loading lists...</div>
                          {:else if taskLists.length > 0}
                            <div class="mt-2 space-y-2">
                              {#each taskLists as list}
                                <div class="flex items-center justify-between bg-gray-50 px-3 py-2 rounded text-sm">
                                  <span class="text-gray-700">{list.title}</span>
                                  <div class="flex items-center gap-2">
                                     {#if importStatus[list.id]}
                                      <span class="text-xs text-gray-500">{importStatus[list.id]}</span>
                                    {/if}
                                    <button 
                                      onclick={() => importList(list.id)}
                                      disabled={importStatus[list.id] === 'importing'}
                                      class="text-xs px-2 py-1 bg-white border border-gray-200 rounded hover:bg-gray-100 flex items-center gap-1 disabled:opacity-50">
                                      {#if importStatus[list.id] === 'importing'}
                                        <Loader2 size={10} class="animate-spin" />
                                      {:else}
                                        <Download size={10} />
                                      {/if}
                                      Import
                                    </button>
                                  </div>
                                </div>
                              {/each}
                            </div>
                          {/if}
                       </div>
                    </li>
                    <li class="flex items-start gap-3 opacity-60">
                       <div class="mt-0.5 text-green-500"><CheckCircle2 size={16} /></div>
                       <div>
                          <span class="font-medium text-gray-800">Calendar</span>
                          <p class="text-xs text-gray-500">Calendar sync coming soon.</p>
                       </div>
                    </li>
                  </ul>
                </div>
                
                <div class="text-right">
                  <button 
                    onclick={() => { isConnected = false; googleUser = null; }}
                    class="text-sm text-red-500 hover:text-red-600 underline flex items-center justify-end gap-1 ml-auto">
                    <LogOut size={14} /> Disconnect Account
                  </button>
                </div>
              </div>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Apple Reminders -->
      <div>
        <button 
          onclick={() => activeSection = activeSection === 'apple' ? null : 'apple'}
          class="w-full flex items-center justify-between p-6 hover:bg-gray-50 transition-colors text-left opacity-60">
          <div class="flex items-center gap-4">
            <div class="p-2 bg-gray-100 rounded-lg text-gray-600">
              <Smartphone size={24} />
            </div>
            <div>
              <h3 class="text-base font-semibold text-gray-900">Apple Reminders</h3>
              <p class="text-sm text-gray-500">Sync with Apple Reminders (Coming Soon)</p>
            </div>
          </div>
          <div class="text-gray-400">
            {#if activeSection === 'apple'}
              <ChevronDown size={20} />
            {:else}
              <ChevronRight size={20} />
            {/if}
          </div>
        </button>
        {#if activeSection === 'apple'}
          <div class="px-6 pb-6 pt-2 bg-gray-50/50">
            <p class="text-sm text-gray-500 p-4 bg-white border border-gray-200 rounded-lg">
              Support for Apple Reminders is currently under development. Please check back later for updates.
            </p>
          </div>
        {/if}
      </div>

    </div>
  </section>
</div>
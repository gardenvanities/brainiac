class UiStore {
  sidebarLeftWidth = $state(260);
  sidebarRightWidth = $state(340);
  sidebarLeftOpen = $state(true);
  sidebarRightOpen = $state(true);
  activeDocumentId = $state<string | null>(null);
  activeAgentId = $state<string>("00000000-0000-0000-0000-000000000001");
}

export const uiStore = new UiStore();

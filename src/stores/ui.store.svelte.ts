class UiStore {
  sidebarLeftWidth = $state(260);
  sidebarRightWidth = $state(340);
  sidebarLeftOpen = $state(true);
  sidebarRightOpen = $state(true);

  // Snapshot NÃO reativo do layout antes do modo foco
  // (memorizado apenas quando o foco é ativado)
  private focusSnapshot = { left: true, right: true };

  toggleLeft() {
    this.sidebarLeftOpen = !this.sidebarLeftOpen;
  }

  toggleRight() {
    this.sidebarRightOpen = !this.sidebarRightOpen;
  }

  // Modo foco: esconde ambas; repetir restaura o layout anterior
  toggleFocus() {
    if (this.sidebarLeftOpen || this.sidebarRightOpen) {
      this.focusSnapshot = {
        left: this.sidebarLeftOpen,
        right: this.sidebarRightOpen,
      };
      this.sidebarLeftOpen = false;
      this.sidebarRightOpen = false;
    } else {
      this.sidebarLeftOpen = this.focusSnapshot.left;
      this.sidebarRightOpen = this.focusSnapshot.right;
    }
  }
}

export { UiStore };
export const uiStore = new UiStore();

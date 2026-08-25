export type AppTheme = "light" | "dark" | "system";

export interface AppConfig {
  litellmBaseUrl: string;
  defaultModel: string;
  documentsPath: string;
  theme: AppTheme;
  sidebarLeftWidth: number;
  sidebarRightWidth: number;
}

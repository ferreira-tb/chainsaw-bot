export enum Command {
  CloseWindow = 'close_window',
  ConnectDiscord = 'connect_discord',
  ShowWindow = 'show_window'
}

export function closeWindow() {
  invoke(Command.CloseWindow).catch(handleError);
}

export async function connectDiscord() {
  await invoke(Command.ConnectDiscord);
}

export async function showWindow() {
  await invoke(Command.ShowWindow);
}

export enum Command {
  CloseWindow = 'close_window',
  ShowWindow = 'show_window'
}

export function closeWindow() {
  invoke(Command.CloseWindow).catch(handleError);
}

export async function showWindow() {
  await invoke(Command.ShowWindow);
}

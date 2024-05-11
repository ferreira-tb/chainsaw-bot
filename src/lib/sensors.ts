function disableDefaultSensors() {
  preventContextMenu();
  preventKeyDown(['F3', 'F7']);

  // Focus move.
  preventKeyDown('Tab', { shiftKey: true });
}

export function setGlobalSensors() {
  disableDefaultSensors();
}

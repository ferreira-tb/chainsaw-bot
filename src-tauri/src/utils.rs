pub mod app {
  use crate::state::Chainsaw;
  use tauri::{Manager, Runtime, State, WebviewWindow};

  #[allow(dead_code)]
  pub trait AppHandleExt<R: Runtime>: Manager<R> {
    fn chainsaw(&self) -> State<Chainsaw> {
      self.state::<Chainsaw>()
    }

    fn get_main_window(&self) -> WebviewWindow<R> {
      self.get_webview_window("main").unwrap()
    }
  }

  impl<R: Runtime> AppHandleExt<R> for tauri::AppHandle<R> {}
}

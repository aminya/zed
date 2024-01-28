use crate::*;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use wgpu::Instance;

pub struct WgpuPlatform {
    instance: Instance,

    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    become_active: Option<Box<dyn FnMut()>>,
    resign_active: Option<Box<dyn FnMut()>>,
    reopen: Option<Box<dyn FnMut()>>,
    quit: Option<Box<dyn FnMut()>>,
    event: Option<Box<dyn FnMut(PlatformInput) -> bool>>,
    menu_command: Option<Box<dyn FnMut(&dyn Action)>>,
    validate_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
    will_open_menu: Option<Box<dyn FnMut()>>,
    menu_actions: Vec<Box<dyn Action>>,
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    finish_launching: Option<Box<dyn FnOnce()>>,
}

pub struct WgpuPlatformTextSystem {
    instance: Instance,
}

impl PlatformTextSystem for WgpuPlatformTextSystem {
    fn add_fonts(&self, fonts: &[Arc<Vec<u8>>]) -> Result<()> {
        todo!()
    }

    fn all_font_names(&self) -> Vec<String> {
        todo!()
    }

    fn all_font_families(&self) -> Vec<String> {
        todo!()
    }

    fn font_id(&self, descriptor: &Font) -> Result<FontId> {
        todo!()
    }

    fn font_metrics(&self, font_id: FontId) -> FontMetrics {
        todo!()
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>> {
        todo!()
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        todo!()
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        todo!()
    }

    fn glyph_raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        todo!()
    }

    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        raster_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        todo!()
    }

    fn layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> LineLayout {
        todo!()
    }

    fn wrap_line(
        &self,
        text: &str,
        font_id: FontId,
        font_size: Pixels,
        width: Pixels,
    ) -> Vec<usize> {
        todo!()
    }
}

impl Platform for WgpuPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        // Return the stored background executor
        self.background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        // Return the stored foreground executor
        self.foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        // Return the stored text system
        self.text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>) {
        // Store the on_finish_launching function for later use
        self.finish_launching = Some(on_finish_launching);
    }

    fn quit(&self) {
        // Call the stored quit function if it exists
        if let Some(quit_fn) = &mut self.quit {
            quit_fn();
        }
    }

    fn restart(&self) {
        todo!()
    }

    fn activate(&self, ignoring_other_apps: bool) {
        todo!()
    }

    fn hide(&self) {
        todo!()
    }

    fn hide_other_apps(&self) {
        todo!()
    }

    fn unhide_other_apps(&self) {
        todo!()
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        todo!()
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        todo!()
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        todo!()
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn PlatformWindow> {
        todo!()
    }

    fn set_display_link_output_callback(
        &self,
        display_id: DisplayId,
        callback: Box<dyn FnMut() + Send>,
    ) {
        todo!()
    }

    fn start_display_link(&self, display_id: DisplayId) {
        todo!()
    }

    fn stop_display_link(&self, display_id: DisplayId) {
        todo!()
    }

    fn open_url(&self, url: &str) {
        todo!()
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        todo!()
    }

    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        todo!()
    }

    fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Option<PathBuf>> {
        todo!()
    }

    fn reveal_path(&self, path: &Path) {
        todo!()
    }

    fn on_become_active(&self, callback: Box<dyn FnMut()>) {
        todo!()
    }

    fn on_resign_active(&self, callback: Box<dyn FnMut()>) {
        todo!()
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        todo!()
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        todo!()
    }

    fn on_event(&self, callback: Box<dyn FnMut(PlatformInput) -> bool>) {
        todo!()
    }

    fn set_menus(&self, menus: Vec<Menu>, keymap: &Keymap) {
        todo!()
    }

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        todo!()
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        todo!()
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        todo!()
    }

    fn os_name(&self) -> &'static str {
        "Wgpu"
    }

    fn os_version(&self) -> Result<SemanticVersion> {
        todo!()
    }

    fn app_version(&self) -> Result<SemanticVersion> {
        todo!()
    }

    fn app_path(&self) -> Result<PathBuf> {
        todo!()
    }

    fn local_timezone(&self) -> UtcOffset {
        todo!()
    }

    fn double_click_interval(&self) -> Duration {
        todo!()
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        todo!()
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        todo!()
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        todo!()
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        todo!()
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        todo!()
    }

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>> {
        todo!()
    }

    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        todo!()
    }

    fn delete_credentials(&self, url: &str) -> Task<Result<()>> {
        todo!()
    }
}

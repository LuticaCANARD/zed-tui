use std::sync::OnceLock;

use anyhow::{Context as _, Result};
use windows::Win32::{
    Foundation::HANDLE,
    Storage::FileSystem::WriteFile,
    System::Console::{
        CONSOLE_MODE, DISABLE_NEWLINE_AUTO_RETURN, ENABLE_PROCESSED_OUTPUT,
        ENABLE_VIRTUAL_TERMINAL_PROCESSING, GetConsoleMode, GetConsoleOutputCP, GetStdHandle,
        STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, SetConsoleCtrlHandler, SetConsoleMode,
        SetConsoleOutputCP,
    },
};

static ORIGINAL_CONSOLE: OnceLock<ConsoleState> = OnceLock::new();

#[derive(Clone, Copy)]
struct ConsoleState {
    input: usize,
    output: usize,
    input_mode: CONSOLE_MODE,
    output_mode: CONSOLE_MODE,
    output_code_page: u32,
}

impl ConsoleState {
    fn restore(self) -> Result<()> {
        unsafe {
            let input = SetConsoleMode(HANDLE(self.input as *mut _), self.input_mode);
            let output = SetConsoleMode(HANDLE(self.output as *mut _), self.output_mode);
            let code_page = SetConsoleOutputCP(self.output_code_page);
            input?;
            output?;
            code_page?;
        }
        Ok(())
    }
}

pub(super) struct ConsoleGuard {
    original: ConsoleState,
    handler_installed: bool,
}

impl ConsoleGuard {
    pub(super) fn enter() -> Result<Self> {
        unsafe {
            let input = GetStdHandle(STD_INPUT_HANDLE)?;
            let output = GetStdHandle(STD_OUTPUT_HANDLE)?;
            let mut input_mode = CONSOLE_MODE::default();
            let mut output_mode = CONSOLE_MODE::default();
            GetConsoleMode(input, &mut input_mode).context("reading console input mode")?;
            GetConsoleMode(output, &mut output_mode).context("reading console output mode")?;
            let mut guard = Self {
                original: ConsoleState {
                    input: input.0 as usize,
                    output: output.0 as usize,
                    input_mode,
                    output_mode,
                    output_code_page: GetConsoleOutputCP(),
                },
                handler_installed: false,
            };
            SetConsoleMode(
                output,
                output_mode
                    | ENABLE_PROCESSED_OUTPUT
                    | ENABLE_VIRTUAL_TERMINAL_PROCESSING
                    | DISABLE_NEWLINE_AUTO_RETURN,
            )
            .context("enabling virtual terminal output; use Windows Terminal")?;
            SetConsoleOutputCP(65001).context("enabling UTF-8 console output")?;
            ORIGINAL_CONSOLE.get_or_init(|| guard.original);
            SetConsoleCtrlHandler(Some(restore_console), true)
                .context("installing the console cleanup handler")?;
            guard.handler_installed = true;
            Ok(guard)
        }
    }
}

impl Drop for ConsoleGuard {
    fn drop(&mut self) {
        if self.handler_installed
            && let Err(error) = unsafe { SetConsoleCtrlHandler(Some(restore_console), false) }
        {
            log::error!("could not remove the console cleanup handler: {error}");
        }
        if let Err(error) = self.original.restore() {
            log::error!("could not restore the console: {error:#}");
        }
    }
}

unsafe extern "system" fn restore_console(_event: u32) -> windows::core::BOOL {
    if let Some(original) = ORIGINAL_CONSOLE.get() {
        // A console-close callback runs on another thread. Avoid stdout locks,
        // which may be held by the renderer while Windows is terminating us.
        let restore = b"\x1b[0m\x1b[0 q\x1b[?1000l\x1b[?1002l\x1b[?1003l\x1b[?1006l\x1b[?2004l\x1b[?25h\x1b[?1049l";
        let result =
            unsafe { WriteFile(HANDLE(original.output as *mut _), Some(restore), None, None) };
        // Logging also takes locks, so only report cleanup failures on normal
        // shutdown through ConsoleGuard::drop.
        let restored = original.restore();
        if result.is_err() || restored.is_err() {
            unsafe {
                windows::Win32::System::Diagnostics::Debug::OutputDebugStringW(windows::core::w!(
                    "Zed TUI could not restore the console during termination"
                ));
            }
        }
    }
    false.into()
}

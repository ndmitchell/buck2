/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::cmp;
use std::env;
use std::io;

use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::tty::IsTty;
use crossterm::QueueableCommand;

use crate::ansi_support::enable_ansi_support;
use crate::components::Canvas;
use crate::components::Component;
use crate::components::DrawMode;
use crate::content::Line;
use crate::output::BlockingSuperConsoleOutput;
use crate::output::SuperConsoleOutput;
use crate::Dimensions;
use crate::Direction;
use crate::Lines;

const MINIMUM_EMIT: usize = 5;
const MAX_GRAPHEME_BUFFER: usize = 1000000;

/// Handles rendering the console using the user-defined [Component](Component)s and emitted messages.
/// A Canvas area at the bottom of the terminal is re-rendered in place at each tick for the components,
/// while a log area of emitted messages is produced above.
/// Producing output from sources other than SuperConsole while break the TUI.
pub struct SuperConsole {
    root: Canvas,
    to_emit: Lines,
    // A default screen size to use if the size cannot be fetched
    // from the terminal. This generally is only used for testing
    // situations.
    fallback_size: Option<Dimensions>,
    pub(crate) output: Box<dyn SuperConsoleOutput>,
}

impl SuperConsole {
    /// Build a new SuperConsole with a root component.
    pub fn new() -> Option<Self> {
        Self::compatible().then(|| {
            Self::new_internal(
                None,
                Box::new(BlockingSuperConsoleOutput::new(Box::new(io::stderr()))),
            )
        })
    }

    /// Force a new SuperConsole to be built with a root component, regardless of
    /// whether the tty is compatible
    pub fn forced_new(fallback_size: Dimensions) -> Self {
        Self::new_internal(
            Some(fallback_size),
            Box::new(BlockingSuperConsoleOutput::new(Box::new(io::stderr()))),
        )
    }

    pub(crate) fn new_internal(
        fallback_size: Option<Dimensions>,
        output: Box<dyn SuperConsoleOutput>,
    ) -> Self {
        Self {
            root: Canvas::new(),
            to_emit: Lines::new(),
            fallback_size,
            output,
        }
    }

    pub fn compatible() -> bool {
        // Superconsole only renders on the stderr, so we can display the superconsole
        // even if someone does `command > out.txt`.
        io::stderr().is_tty() && !Self::is_term_dumb() && enable_ansi_support().is_ok()
    }

    fn is_term_dumb() -> bool {
        // Some terminals (e.g. Emacs' eshell) are very limited in functionality and don't support
        // the control codes required for superconsole to work.
        matches!(env::var("TERM"), Ok(kind) if kind == "dumb")
    }

    /// Render at a given tick.  Draws all components and drains the emitted events buffer.
    /// This will produce any pending emitting events above the Canvas and will re-render the drawing area.
    pub fn render(&mut self, root: &dyn Component) -> anyhow::Result<()> {
        // `render_general` refuses to drain more than a single frame, so repeat until done.
        // or until the rendered frame is too large to print anything.
        let mut anything_emitted = true;
        let mut has_rendered = false;
        while !has_rendered || (anything_emitted && !self.to_emit.is_empty()) {
            if !self.output.should_render() {
                break;
            }

            let last_len = self.to_emit.len();
            self.render_with_mode(root, DrawMode::Normal)?;
            anything_emitted = last_len == self.to_emit.len();
            has_rendered = true;
        }

        Ok(())
    }

    /// Perform a final render with [`DrawMode::Final`].
    /// Each component will have a chance to finalize themselves before the terminal is disposed of.
    pub fn finalize(self, root: &dyn Component) -> anyhow::Result<()> {
        self.finalize_with_mode(root, DrawMode::Final)
    }

    /// Perform a final render, using a specified [`DrawMode`].
    /// Each component will have a chance to finalize themselves before the terminal is disposed of.
    pub fn finalize_with_mode(
        mut self,
        root: &dyn Component,

        mode: DrawMode,
    ) -> anyhow::Result<()> {
        self.render_with_mode(root, mode)?;
        self.output.finalize()
    }

    /// Convenience method:
    /// - Calls queue_emit to add the lines.
    /// - Next, re-renders the `superconsole`.
    ///
    /// Because this re-renders the console, it requires passed state.
    /// Overuse of this method can cause `superconsole` to use significant CPU.
    pub fn emit_now(&mut self, lines: Lines, root: &dyn Component) -> anyhow::Result<()> {
        self.emit(lines);
        self.render(root)
    }

    /// Queues the passed lines to be drawn on the next render.
    /// The lines *will not* appear until the next render is called.
    pub fn emit(&mut self, mut lines: Lines) {
        self.to_emit.0.append(&mut lines.0);
    }

    fn size(&self) -> anyhow::Result<Dimensions> {
        // We want to get the size, but if that fails or is empty use the fallback_size if available.
        match (self.output.terminal_size(), self.fallback_size) {
            (Ok(size), Some(fallback)) if size.width == 0 || size.height == 0 => Ok(fallback),
            (Ok(size), _) => Ok(size),
            (Err(_), Some(fallback)) => Ok(fallback),
            (Err(e), None) => Err(e),
        }
    }

    /// Clears the canvas portion of the superconsole.
    pub fn clear(&mut self) -> anyhow::Result<()> {
        let mut buffer = vec![];
        self.root.clear(&mut buffer)?;
        self.output.output(buffer)
    }

    /// Helper method to share render + finalize behavior by specifying mode.
    fn render_with_mode(&mut self, root: &dyn Component, mode: DrawMode) -> anyhow::Result<()> {
        // TODO(cjhopman): We may need to try to keep each write call to be under the pipe buffer
        // size so it can be completed in a single syscall otherwise we might see a partially
        // rendered frame.

        // We remove the last line as we always have a blank final line in our output.
        let size = self.size()?.saturating_sub(1, Direction::Vertical);
        let mut buffer = Vec::new();

        self.render_general(&mut buffer, root, mode, size)?;
        self.output.output(buffer)
    }

    /// Helper method that makes rendering highly configurable.
    fn render_general(
        &mut self,
        buffer: &mut Vec<u8>,
        root: &dyn Component,

        mode: DrawMode,
        size: Dimensions,
    ) -> anyhow::Result<()> {
        /// Heuristic to determine if a buffer is too large to buffer.
        /// Can be tuned, but is currently set to 1000000 graphemes.
        #[allow(clippy::ptr_arg)]
        fn is_big(buf: &Lines) -> bool {
            let len: usize = buf.iter().map(Line::len).sum();
            len > MAX_GRAPHEME_BUFFER
        }

        // Go the beginning of the canvas.
        self.root.move_up(buffer)?;

        // Pre-draw the frame *and then* start rendering emitted messages.
        let mut frame = self.root.draw(root, size, mode)?;
        // Render at most a single frame if this not the last render.
        // Does not buffer if there is a ridiculous amount of data.
        let limit = match mode {
            DrawMode::Normal if !is_big(&self.to_emit) => {
                let limit = size.height.saturating_sub(frame.len());
                // arbitrary value picked so we don't starve `emit` on small terminal sizes.
                Some(cmp::max(limit, MINIMUM_EMIT))
            }
            _ => None,
        };
        self.to_emit.render(buffer, limit)?;
        frame.render(buffer, None)?;

        // clear any residue from the previous render.
        buffer.queue(Clear(ClearType::FromCursorDown))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Context as _;
    use derive_more::AsRef;

    use super::*;
    use crate::components::echo::Echo;
    use crate::testing::frame_contains;
    use crate::testing::test_console;
    use crate::testing::SuperConsoleTestingExt;
    use crate::Lines;

    #[derive(AsRef, Debug)]
    struct Msg(Lines);

    #[test]
    fn test_small_buffer() -> anyhow::Result<()> {
        let mut console = test_console();
        let msg_count = MINIMUM_EMIT + 5;
        console.emit(Lines(vec![vec!["line 1"].try_into()?; msg_count]));
        let msg = Lines(vec![vec!["line"].try_into()?; msg_count]);
        let root = Echo(msg);
        let mut buffer = Vec::new();

        // even though the canvas is larger than the tty
        console.render_general(
            &mut buffer,
            &root,
            DrawMode::Normal,
            Dimensions::new(100, 2),
        )?;

        // we should still drain a minimum of 5 messages.
        assert_eq!(console.to_emit.len(), msg_count - MINIMUM_EMIT);

        Ok(())
    }

    #[test]
    fn test_huge_buffer() -> anyhow::Result<()> {
        let mut console = test_console();
        console.emit(Lines(vec![
            vec!["line 1"].try_into()?;
            MAX_GRAPHEME_BUFFER * 2
        ]));
        let root = Echo(Lines(vec![vec!["line"].try_into()?; 1]));
        let mut buffer = Vec::new();

        // Even though we have more messages than fit on the screen in the `to_emit` buffer
        console.render_general(
            &mut buffer,
            &root,
            DrawMode::Normal,
            Dimensions::new(100, 20),
        )?;

        // We have so many that we should just drain them all.
        assert!(console.to_emit.is_empty());

        Ok(())
    }

    /// Check that no frames are produced when should_render returns false.
    #[test]
    fn test_block_render() -> anyhow::Result<()> {
        let mut console = test_console();

        let root = Echo(Lines(vec![vec!["state"].try_into()?; 1]));

        console.render(&root)?;
        assert_eq!(console.test_output()?.frames.len(), 1);

        console.test_output_mut()?.should_render = false;
        console.render(&root)?;
        assert_eq!(console.test_output()?.frames.len(), 1);

        console.emit(Lines(vec![vec!["line 1"].try_into()?]));
        console.render(&root)?;
        assert_eq!(console.test_output()?.frames.len(), 1);

        Ok(())
    }

    /// Check that lines are deferred when should_render returns false, and emitted once the output
    /// is unblocked.
    #[test]
    fn test_block_lines() -> anyhow::Result<()> {
        let mut console = test_console();

        let root = Echo(Lines(vec![vec!["state"].try_into()?; 1]));

        console.test_output_mut()?.should_render = false;
        console.emit(Lines(vec![vec!["line 1"].try_into()?]));
        console.render(&root)?;
        assert_eq!(console.test_output()?.frames.len(), 0);

        console.test_output_mut()?.should_render = true;
        console.emit(Lines(vec![vec!["line 2"].try_into()?]));
        console.render(&root)?;

        let frame = console
            .test_output_mut()?
            .frames
            .pop()
            .context("No frame was emitted")?;

        assert!(frame_contains(&frame, "state"));
        assert!(frame_contains(&frame, "line 1"));
        assert!(frame_contains(&frame, "line 2"));

        Ok(())
    }

    /// Check that render_with_mode does not respect should_render.
    #[test]
    fn test_block_finalize() -> anyhow::Result<()> {
        let mut console = test_console();

        let root = Echo(Lines(vec![vec!["state"].try_into()?; 1]));

        console.test_output_mut()?.should_render = false;
        console.emit(Lines(vec![vec!["line 1"].try_into()?]));
        console.emit(Lines(vec![vec!["line 2"].try_into()?]));
        console.render_with_mode(&root, DrawMode::Final)?;

        let frame = console
            .test_output_mut()?
            .frames
            .pop()
            .context("No frame was emitted")?;

        assert!(frame_contains(&frame, "state"));
        assert!(frame_contains(&frame, "line 1"));
        assert!(frame_contains(&frame, "line 2"));

        Ok(())
    }
}

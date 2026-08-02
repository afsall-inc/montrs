//! Video creation pipeline for montrs-motion.
//!
//! Inspired by Remotion — creates videos frame-by-frame using MontRS rendering,
//! then stitches frames with FFmpeg.
//!
//! # Example
//! ```rust,ignore
//! use montrs_motion::video::{VideoComposition, Frame};
//!
//! let mut video = VideoComposition::new(1920, 1080, 30, 90);
//! for frame in 0..90 {
//!     video.add_frame(Frame::new(frame, format!("<svg>...</svg>")))?;
//! }
//! video.render("output.mp4")?;
//! ```

use std::path::Path;

/// A single frame in a video composition.
#[derive(Debug, Clone)]
pub struct Frame {
    /// Frame index (0-based).
    pub index: u32,
    /// SVG content for this frame (rendered as raster).
    pub svg: String,
}

impl Frame {
    pub fn new(index: u32, svg: String) -> Self {
        Self { index, svg }
    }
}

/// A video composition with configurable dimensions, framerate, and duration.
///
/// Like Remotion's `<Composition>` — defines the video's dimensions, FPS,
/// and duration in frames, then accepts frames to render.
#[derive(Debug, Clone)]
pub struct VideoComposition {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub duration_in_frames: u32,
    frames: Vec<Frame>,
}

impl VideoComposition {
    pub fn new(width: u32, height: u32, fps: u32, duration_in_frames: u32) -> Self {
        Self {
            width,
            height,
            fps,
            duration_in_frames,
            frames: Vec::with_capacity(duration_in_frames as usize),
        }
    }

    /// Add a frame to the composition.
    pub fn add_frame(&mut self, frame: Frame) -> Result<(), VideoError> {
        if frame.index >= self.duration_in_frames {
            return Err(VideoError::FrameOutOfRange {
                frame: frame.index,
                max: self.duration_in_frames - 1,
            });
        }
        self.frames.push(frame);
        Ok(())
    }

    /// Render the video to a file using FFmpeg.
    ///
    /// Each frame's SVG is rendered to a temporary PNG, then FFmpeg stitches
    /// them into a video with the configured FPS.
    pub fn render(&self, output: &Path) -> Result<(), VideoError> {
        if self.frames.is_empty() {
            return Err(VideoError::NoFrames);
        }

        // TODO: Implement actual frame rendering pipeline:
        // 1. Render each SVG to a pixel buffer (via montrs-renderer or resvg)
        // 2. Write PNG files to temp dir
        // 3. Run FFmpeg: ffmpeg -framerate {fps} -i temp/frame_%04d.png -c:v libx264 -pix_fmt yuv420p {output}
        // 4. Clean up temp dir

        // For now, provide a descriptive error
        Err(VideoError::RenderNotImplemented {
            message: format!(
                "Video rendering requires FFmpeg. Would render {} frames at {}x{} @ {}fps to {}",
                self.frames.len(),
                self.width,
                self.height,
                self.fps,
                output.display()
            ),
        })
    }
}

/// Errors that can occur during video creation.
#[derive(Debug, thiserror::Error)]
pub enum VideoError {
    #[error("Frame {frame} out of range (max: {max})")]
    FrameOutOfRange { frame: u32, max: u32 },

    #[error("No frames in composition")]
    NoFrames,

    #[error("Render not implemented: {message}")]
    RenderNotImplemented { message: String },

    #[cfg(feature = "ffmpeg")]
    #[error("FFmpeg error: {0}")]
    Ffmpeg(String),
}

/// Converts a sequence of SVG strings to a video file.
///
/// Convenience function that creates a `VideoComposition`, adds all frames,
/// and renders.
pub fn render_svg_sequence(
    svg_frames: Vec<String>,
    width: u32,
    height: u32,
    fps: u32,
    output: &Path,
) -> Result<(), VideoError> {
    let duration = svg_frames.len() as u32;
    let mut composition = VideoComposition::new(width, height, fps, duration);

    for (i, svg) in svg_frames.into_iter().enumerate() {
        composition.add_frame(Frame::new(i as u32, svg))?;
    }

    composition.render(output)
}
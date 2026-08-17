// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::{path::Path, process::Command};

#[derive(Debug, Clone)]
pub struct Frame {
    pub index: u32,
    pub svg: String,
}

impl Frame {
    pub fn new(index: u32, svg: String) -> Self {
        Self { index, svg }
    }
}

#[derive(Debug, Clone)]
pub struct VideoComposition {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub duration_in_frames: u32,
    frames: Vec<Frame>,
}

impl VideoComposition {
    pub fn new(
        width: u32,
        height: u32,
        fps: u32,
        duration_in_frames: u32,
    ) -> Self {
        Self {
            width,
            height,
            fps,
            duration_in_frames,
            frames: Vec::with_capacity(duration_in_frames as usize),
        }
    }

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

    pub fn render(&self, output: &Path) -> Result<(), VideoError> {
        if self.frames.is_empty() {
            return Err(VideoError::NoFrames);
        }
        let temp_dir =
            tempfile::tempdir().map_err(|e| VideoError::Io(e.to_string()))?;
        let temp_path = temp_dir.path();
        for frame in &self.frames {
            let png_path =
                temp_path.join(format!("frame_{:04}.png", frame.index));
            render_svg_to_png(&frame.svg, self.width, self.height, &png_path)?;
        }
        let input_pattern = temp_path.join("frame_%04d.png");
        let status = Command::new("ffmpeg")
            .args([
                "-framerate",
                &self.fps.to_string(),
                "-i",
                &input_pattern.to_string_lossy(),
                "-c:v",
                "libx264",
                "-pix_fmt",
                "yuv420p",
                "-y",
                &output.to_string_lossy(),
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map_err(|e| {
                VideoError::Ffmpeg(format!("FFmpeg not found: {}", e))
            })?;
        if !status.success() {
            return Err(VideoError::Ffmpeg("FFmpeg encoding failed".into()));
        }
        Ok(())
    }
}

fn render_svg_to_png(
    svg: &str,
    width: u32,
    height: u32,
    output: &Path,
) -> Result<(), VideoError> {
    #[cfg(feature = "video")]
    {
        let opt = usvg::Options::default();
        let tree = usvg::Tree::from_str(svg, &opt)
            .map_err(|e| VideoError::Svg(format!("SVG parse error: {}", e)))?;
        let pixmap_size = resvg::IntSize::from_wh(width, height)
            .ok_or(VideoError::Svg("Invalid dimensions".into()))?;
        let mut pixmap =
            tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
                .ok_or(VideoError::Svg("Failed to create pixmap".into()))?;
        resvg::render(&tree, resvg::Transform::default(), &mut pixmap.as_mut());
        pixmap
            .save_png(output)
            .map_err(|e| VideoError::Io(e.to_string()))
    }
    #[cfg(not(feature = "video"))]
    {
        let _ = (svg, width, height, output);
        Err(VideoError::Svg(
            "Video feature not enabled. Use --features video".into(),
        ))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VideoError {
    #[error("Frame {frame} out of range (max: {max})")]
    FrameOutOfRange { frame: u32, max: u32 },
    #[error("No frames in composition")]
    NoFrames,
    #[error("FFmpeg error: {0}")]
    Ffmpeg(String),
    #[error("SVG error: {0}")]
    Svg(String),
    #[error("IO error: {0}")]
    Io(String),
}

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

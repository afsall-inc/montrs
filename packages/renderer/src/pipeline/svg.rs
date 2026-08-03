#[cfg(feature = "svg")]
use std::collections::HashMap;

#[cfg(feature = "svg")]
pub struct SvgEntry {
    pub pixmap: tiny_skia::Pixmap,
    pub width: u32,
    pub height: u32,
}

#[cfg(feature = "svg")]
pub struct SvgPipeline {
    entries: HashMap<u64, SvgEntry>,
    next_id: u64,
}

#[cfg(not(feature = "svg"))]
pub struct SvgPipeline;

#[cfg(not(feature = "svg"))]
impl Default for SvgPipeline {
    fn default() -> Self {
        Self
    }
}

#[cfg(feature = "svg")]
impl Default for SvgPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "svg")]
impl SvgPipeline {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn render(&mut self, svg_data: &str, width: u32, height: u32) -> u64 {
        let opt = usvg::Options::default();
        let rtree = match usvg::Tree::from_str(svg_data, &opt) {
            Ok(tree) => tree,
            Err(_) => return 0,
        };

        let mut pixmap = match tiny_skia::Pixmap::new(width, height) {
            Some(p) => p,
            None => return 0,
        };

        resvg::render(
            &rtree,
            tiny_skia::Transform::default(),
            &mut pixmap.as_mut(),
        );

        let id = self.next_id;
        self.next_id += 1;
        self.entries.insert(
            id,
            SvgEntry {
                pixmap,
                width,
                height,
            },
        );
        id
    }

    pub fn get_pixmap(&self, id: u64) -> Option<&tiny_skia::Pixmap> {
        self.entries.get(&id).map(|e| &e.pixmap)
    }
}

#[cfg(not(feature = "svg"))]
impl SvgPipeline {
    pub fn new() -> Self {
        Self
    }

    pub fn render(
        &mut self,
        _svg_data: &str,
        _width: u32,
        _height: u32,
    ) -> u64 {
        0
    }
}

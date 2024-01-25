//! Module to perform image modification.
//!

use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

use image::io::Reader as ImageReader;
use image::GenericImage;
use image::Rgb;
use imageproc::drawing::{self, Canvas};
use imageproc::rect::Rect;
use strem::datastream::frame::sample::detections::{Annotation, DetectionRecord, ImageSource};
use strem::datastream::frame::sample::Sample;
use strem::datastream::frame::Frame;

/// Image printer.
pub struct Imager {}

impl Imager {
    /// Create a new [`Imager`].
    pub fn new() -> Self {
        Imager {}
    }

    pub fn draw(&self, frame: &Frame, indir: &Path, outdir: &Path) -> Result<(), Box<dyn Error>> {
        for sample in frame.samples.iter() {
            match sample {
                Sample::ObjectDetection(d) => self.detection(d, indir, outdir)?,
            }
        }

        Ok(())
    }

    pub fn detection(
        &self,
        record: &DetectionRecord,
        indir: &Path,
        outdir: &Path,
    ) -> Result<(), Box<dyn Error>> {
        let mut outfile = PathBuf::from(outdir);

        if let Some(image) = &record.image {
            let mut img = match &image.source {
                ImageSource::File(path) => {
                    if let Some(filename) = path.file_name() {
                        outfile.push(filename);
                    }

                    let mut infile = PathBuf::new();
                    infile.push(indir);
                    infile.push(path);

                    ImageReader::open(&infile)
                        .or(Err(Box::new(ImagerError::from(format!(
                            "image at `{}` not found",
                            infile.display()
                        )))))?
                        .decode()?
                        .into_rgb8()
                }
            };

            for annotations in record.annotations.values() {
                for annotation in annotations.iter() {
                    img = self.bbox(img, annotation);
                }
            }

            img.save(outfile)
                .or(Err(Box::new(ImagerError::from(format!(
                    "`{}` directory not found",
                    outdir.display()
                )))))?
        }

        Ok(())
    }

    fn bbox<C>(&self, mut canvas: C, annotation: &Annotation) -> C
    where
        C: Canvas<Pixel = Rgb<u8>> + GenericImage,
    {
        let color = match &annotation.label[..] {
            "car" => Rgb([255, 15, 0]),          // red
            "pedestrian" => Rgb([0, 111, 255]),  // blue
            "truck" => Rgb([255, 0, 255]),       // magenta
            "sign" => Rgb([255, 255, 0]),        // yellow
            "cyclist" => Rgb([0, 253, 255]),     // cyan
            "animal" => Rgb([102, 0, 255]),      // purple
            "motorcycle" => Rgb([252, 132, 39]), // orange
            _ => Rgb([102, 255, 0]),             // green
        };

        let xmin = annotation.bbox.min.x as i32;
        let ymin = annotation.bbox.min.y as i32;
        let xmax = annotation.bbox.max.x as i32;
        let ymax = annotation.bbox.max.y as i32;

        const THICKNESS: u32 = 5;

        // draw top bar
        drawing::draw_filled_rect_mut(
            &mut canvas,
            Rect::at(xmin, ymin).of_size((xmax - xmin) as u32, THICKNESS),
            color,
        );

        // draw bottom bar
        drawing::draw_filled_rect_mut(
            &mut canvas,
            Rect::at(xmin, ymax).of_size(((xmax - xmin) as u32) + THICKNESS, THICKNESS),
            color,
        );

        // draw left bar
        drawing::draw_filled_rect_mut(
            &mut canvas,
            Rect::at(xmin, ymin).of_size(THICKNESS, (ymax - ymin) as u32),
            color,
        );

        // draw right bar
        drawing::draw_filled_rect_mut(
            &mut canvas,
            Rect::at(xmax, ymin).of_size(THICKNESS, (ymax - ymin) as u32),
            color,
        );

        canvas
    }
}

#[derive(Debug, Clone)]
struct ImagerError {
    msg: String,
}

impl From<&str> for ImagerError {
    fn from(msg: &str) -> Self {
        ImagerError {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for ImagerError {
    fn from(msg: String) -> Self {
        ImagerError { msg }
    }
}

impl fmt::Display for ImagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "imager: {}", self.msg)
    }
}

impl Error for ImagerError {}

//! Module to perform image modification.
//!

use std::{
    error::Error,
    path::{Path, PathBuf},
};

use image::{io::Reader as ImageReader, GenericImage, Rgb};
use imageproc::{
    drawing::{self, Canvas},
    rect::Rect,
};

use strem::{
    config::Configuration,
    datastream::{
        frame::Frame,
        reader::{
            detection::{annotation::Annotation, image::ImageSource, Detection},
            Sample,
        },
    },
};

/// Image printer.
#[allow(dead_code)]
pub struct Imager<'a> {
    config: &'a Configuration<'a>,
    path: Option<&'a PathBuf>,
}

impl<'a> Imager<'a> {
    /// Create a new [`Imager`].
    pub fn new(config: &'a Configuration, path: Option<&'a PathBuf>) -> Self {
        Imager { config, path }
    }

    pub fn draw(&self, frame: &Frame, outdir: &Path) -> Result<(), Box<dyn Error>> {
        for sample in frame.samples.iter() {
            match sample {
                Sample::ObjectDetection(d) => self.detection(d, outdir)?,
            }
        }

        Ok(())
    }

    pub fn detection(&self, detection: &Detection, outdir: &Path) -> Result<(), Box<dyn Error>> {
        let mut outfile = PathBuf::from(outdir);

        if let Some(image) = &detection.image {
            let mut img = match image.source {
                ImageSource::File { ref path } => {
                    if let Some(filename) = path.file_name() {
                        outfile.push(filename);
                    }

                    let mut infile = PathBuf::new();

                    if let Some(stream) = self.path {
                        if let Some(parent) = stream.parent() {
                            infile.push(parent);
                        }
                    }

                    infile.push(path);

                    ImageReader::open(infile)?.decode()?.into_rgb8()
                }
            };

            for annotations in detection.annotations.values() {
                for annotation in annotations.iter() {
                    img = self.bbox(img, annotation);
                }
            }

            img.save(outfile)?;
        }

        Ok(())
    }

    fn bbox<C>(&self, mut canvas: C, annotation: &Annotation) -> C
    where
        C: Canvas<Pixel = Rgb<u8>> + GenericImage,
    {
        let color = match annotation.category {
            0 => Rgb([255, 15, 0]),   // red
            1 => Rgb([0, 111, 255]),  // blue
            2 => Rgb([255, 0, 255]),  // magenta
            3 => Rgb([255, 255, 0]),  // yellow
            4 => Rgb([0, 253, 255]),  // cyan
            5 => Rgb([102, 0, 255]),  // purple
            6 => Rgb([252, 132, 39]), // orange
            _ => Rgb([102, 255, 0]),  // green
        };

        let xmin = (annotation.bbox.translation.x - annotation.bbox.dimensions.width / 2.0) as i32;
        let ymin = (annotation.bbox.translation.y - annotation.bbox.dimensions.height / 2.0) as i32;
        let xmax = (annotation.bbox.translation.x + annotation.bbox.dimensions.width / 2.0) as i32;
        let ymax = (annotation.bbox.translation.y + annotation.bbox.dimensions.height / 2.0) as i32;

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

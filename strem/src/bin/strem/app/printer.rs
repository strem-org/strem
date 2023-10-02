//! Application printer.
//!

mod imager;

use std::{error::Error, fmt, path::PathBuf};

use strem::{config::Configuration, datastream::reader::Sample, matcher::Match};

use crate::app::printer::imager::Imager;

pub struct Printer<'a> {
    config: &'a Configuration<'a>,
    path: Option<&'a PathBuf>,
    count: usize,
}

impl<'a> Printer<'a> {
    /// Create a new [`Printer`] with a [`Configuration`].
    pub fn new(config: &'a Configuration, path: Option<&'a PathBuf>) -> Self {
        Printer {
            config,
            path,
            count: 0,
        }
    }

    /// Print a [`Match`].
    pub fn print(&mut self, m: &Match) -> Result<(), Box<dyn Error>> {
        self.count += 1;

        let output = if self.config.count {
            format!("{}", self.count)
        } else if let Some(fmtstr) = self.config.format {
            self.format(m, fmtstr)
        } else {
            self.format(m, "F: %m")
        };

        println!("{}", output);

        if let Some(outdir) = self.config.draw {
            let imager = Imager::new(self.config, self.path);

            for frame in m.frames.iter() {
                imager.draw(frame, outdir)?;
            }
        }

        Ok(())
    }

    fn format(&self, m: &Match, fmtstr: &str) -> String {
        let mut line = String::new();
        let fmtchars = fmtstr.chars().collect::<Vec<char>>();

        /// Append a shortened version of an item to the line for printing.
        fn _short<T: fmt::Display>(item: &Option<T>, line: &mut String) {
            if let Some(item) = item {
                let s = format!("{}", item);

                // shorten it
                let s = s.chars().take(8).collect::<String>();
                line.push_str(s.as_str());
            }
        }

        /// Add the match indices range to the line for printing.
        fn indices(m: &Match, line: &mut String) {
            let s = format!(
                "{}..{}",
                m.frames.first().unwrap().index,
                m.frames.last().unwrap().index
            );

            line.push_str(s.as_str());
        }

        /// Add the associated channel.
        fn channel(m: &Match, line: &mut String) {
            match m.frames.first().unwrap().samples.first().unwrap() {
                Sample::ObjectDetection(d) => {
                    line.push_str(&d.channel);
                }
            }
        }

        let mut i = 0;

        while i < fmtchars.len() {
            match fmtchars[i] {
                '%' => {
                    // format item
                    i += 1;
                    if let Some(tag) = fmtchars.get(i) {
                        match tag {
                            '%' => line.push('%'),
                            'm' => indices(m, &mut line),
                            'c' => channel(m, &mut line),
                            _ => {
                                i += 1; // skip unrecognized type
                                continue;
                            }
                        }
                    }
                }
                _ => {
                    // copy character to output
                    line.push(fmtchars[i]);
                }
            }

            i += 1;
        }

        line
    }
}

// impl<'a> Printer<'a> {
//     /// Create new Printer with [`Configuration`].
//     pub fn new(config: &'a Configuration) -> Self {
//         Printer { config }
//     }

//     /// Print the matches from a [`ContextMatch`].
//     pub fn print(&self, result: Vec<SourceContext>) -> Result<(), Box<dyn error::Error>> {
//         if self.config.count.active {
//             for source in result {
//                 if let Some(path) = source.path {
//                     if let Some(matches) = source.matches {
//                         println!("{}:{}", path, matches.len());
//                     }
//                 }
//             }

//             return Ok(());
//         }

//         for (i, source) in result.into_iter().enumerate() {
//             if let Some(matches) = source.matches {
//                 for ctx in matches {
//                     let line = self.format(&ctx, &source.path, &self.config.format);
//                     println!("{}", line);

//                     if self.config.export.active {
//                         // Export image.
//                         if let Some(source) = &source.path {
//                             let mut target = PathBuf::from(source);

//                             target.push("out");
//                             target.push(ctx.scene.unwrap_or(String::from("scene")));
//                             target.push(ctx.channel.unwrap_or(String::from("channel")));
//                             target.push(format!("{:0>4}", i));

//                             fs::create_dir_all(&target)?;

//                             if let Some(frames) = ctx.frames {
//                                 let imager = Imager::new();

//                                 for frame in frames.iter() {
//                                     imager.draw(Path::new(source), &target, frame)?;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         Ok(())
//     }
// }

#[derive(Debug, Clone)]
struct PrinterError {
    msg: String,
}

impl From<&str> for PrinterError {
    fn from(msg: &str) -> Self {
        PrinterError {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for PrinterError {
    fn from(msg: String) -> Self {
        PrinterError { msg }
    }
}

impl fmt::Display for PrinterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "printer: {}", self.msg)
    }
}

impl Error for PrinterError {}

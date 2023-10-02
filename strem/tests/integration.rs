use strem::{config::Configuration, controller::Controller};

pub mod common;

#[test]
#[ignore] // local dataset needed
fn application() {
    let config: Configuration = common::config("[[:pedestrian:]]", "CAM_FRONT");

    let controller = Controller::new(&config);

    let sources = controller.run().unwrap();
    let source = sources.first().unwrap();

    let matches = &source.matches;

    if let Some(matches) = matches {
        assert_eq!(matches.len(), 10);
    }
}

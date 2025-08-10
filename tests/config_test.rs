use conway::Config;

macro_rules! svec {
    ($($x:expr),*) => {
        vec![$($x.to_string()),*]
    };
}

#[test]
fn parse_simple() {
    let args = svec!["conway", "--width", "600", "--height", "800", "--save-frames", "--max-iterations", "10"];
    let result = Config::build(&args).unwrap();

    assert_eq!(result.width, 600);
    assert_eq!(result.height, 800);
    assert_eq!(result.save_frames, true);
    assert_eq!(result.max_iterations, Some(10));
}

#[test]
fn parse_default() {
    let args = svec!["conway"];
    let result = Config::build(&args).unwrap();

    assert_eq!(result.width, 200);
    assert_eq!(result.height, 100);
    assert_eq!(result.save_frames, false);
    assert_eq!(result.max_iterations, None);
}

#[test]
fn parse_unknown_options() {
    let args = svec!["conway", "--foo"];
    let result = Config::build(&args);
    assert!(result.is_err());
}

#[test]
fn parse_missing_value() {
    let args = svec!["conway", "--width", "--height", "100"];
    let result = Config::build(&args);
    assert!(result.is_err());
}

#[test]
fn parse_unparsable_value() {
    let args = svec!["conway", "--height", "foobar"];
    let result = Config::build(&args);
    assert!(result.is_err());
}
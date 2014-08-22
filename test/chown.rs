use std::io::process::Command;
use std::str;

static PROGNAME: &'static str = "./chown";
static USAGE_STRING: &'static str = "Usage: chown [-fhv] [-R [-H | -L | -P]] owner[:group] file ...\n       chown [-fhv] [-R [-H | -L | -P]] :group file ...";

#[test]
fn test_usage_returned_when_incorrect_call() {
    let process = Command::new(PROGNAME).arg("-h").output().unwrap();
    let out = str::from_utf8(process.output.as_slice()).unwrap().trim();

    assert_eq!(out, USAGE_STRING);
}

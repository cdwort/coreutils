#![crate_name = "chown"]
/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Amy Unger <amy.e.unger@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

#![feature(macro_rules)]
extern crate getopts;
extern crate libc;

use c_types::{get_pw_from_args, get_group};
use getopts::{getopts, optflag};
use libc::funcs::posix88::unistd::{chown};

#[path = "../common/util.rs"] mod util;
#[path = "../common/c_types.rs"] mod c_types;

static NAME: &'static str = "chown";
static VERSION: &'static str = "1.0.0";

pub fn uumain(args: Vec<String>) -> int {

    let options = [
        optflag("f", "", "Don't report any failure to change file owner or group, \
                          nor modify the exit status to reflect such failures."),
        optflag("H", "", "If the -R option is specified, symbolic links on the \
                          command line are followed.  (Symbolic links encountered \
                          in the tree traversal are not followed.)"),
        optflag("", "help", "Print usage."),
        optflag("V", "version", "Print version.")

    ];

    let opts = match getopts(args.tail(), options) {
        Ok(m) => m,
        Err(f) => {
            show_error!("{}", f);
            print_usage();
            return 1
        }
    };

    if opts.opt_present("V") { version(); return 0 }
    if opts.opt_present("help") { print_usage(); return 0 }

    let success = 0i;
    success
}

fn version() {
    println!("{} v{}", NAME, VERSION)
}

fn print_usage() {
    println!("Usage: chown [-fhv] [-R [-H | -L | -P]] owner[:group] file ...");
    println!("       chown [-fhv] [-R [-H | -L | -P]] :group file ...");
}


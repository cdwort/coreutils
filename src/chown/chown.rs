#![crate_name = "chown"]
/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Amy Unger <amy.e.unger@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

extern crate getopts;

use getopts::{optflag};

pub fn uumain(args: Vec<String>) -> int {
    print_usage();

    let options = [
        optflag("f", "", "Don't report any failure to change file owner or group, \
                          nor modify the exit status to reflect such failures."),
        optflag("H", "", "If the -R option is specified, symbolic links on the \
                          command line are followed.  (Symbolic links encountered \
                          in the tree traversal are not followed.)")
    ];

    let success = 0i;
    success
}

fn print_usage() {
    println!("Usage: chown [-fhv] [-R [-H | -L | -P]] owner[:group] file ...");
    println!("       chown [-fhv] [-R [-H | -L | -P]] :group file ...");
}


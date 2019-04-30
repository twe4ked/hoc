//! A command line tool to calculate the hits-of-code metric in a source code repository using Git.
//!
//! You can read more about hits-of-code metric in this blog post:
//! [Hits-of-Code Instead of SLoC][blog_post].
//!
//! Based on the [Ruby version by Yegor Bugayenko][ruby_version].
//!
//! [blog_post]: https://www.yegor256.com/2014/11/14/hits-of-code.html
//! [ruby_version]: https://github.com/yegor256/hoc
use std::{process, process::Command, str};

const COMMAND_ARGUMENTS: &'static [&'static str] = &[
    "log",
    "--pretty=tformat:",
    "--numstat",
    "--ignore-space-change",
    "--ignore-all-space",
    "--ignore-submodules",
    "--no-color",
    "--diff-filter=ACDM",
];

pub fn run(find_renames_and_copies: bool) {
    let mut extra_command_arguments: &'static [&'static str] = &[];
    // Enabling this option causes the git command to be significantly slower.
    if find_renames_and_copies {
        // From the git-log man page:
        //
        // -M[<n>], --find-renames[=<n>]
        //     If generating diffs, detect and report renames for each commit. For following files
        //     across renames while traversing history, see --follow. If n is specified, it is a
        //     threshold on the similarity index (i.e. amount of addition/deletions compared to the
        //     file's size). For example, -M90% means Git should consider a delete/add pair to be a
        //     rename if more than 90% of the file hasn't changed. Without a % sign, the number is
        //     to be read as a fraction, with a decimal point before it. I.e., -M5 becomes 0.5, and
        //     is thus the same as -M50%. Similarly, -M05 is the same as -M5%. To limit detection
        //     to exact renames, use -M100%. The default similarity index is 50%.
        //
        // -C[<n>], --find-copies[=<n>]
        //     Detect copies as well as renames. See also --find-copies-harder. If n is specified,
        //     it has the same meaning as for -M<n>.
        //
        // --find-copies-harder
        //     For performance reasons, by default, -C option finds copies only if the original
        //     file of the copy was modified in the same changeset. This flag makes the command
        //     inspect unmodified files as candidates for the source of copy. This is a very
        //     expensive operation for large projects, so use it with caution. Giving more than one
        //     -C option has the same effect.
        extra_command_arguments = &["--find-renames", "--find-copies-harder"];
    }

    let output = Command::new("git")
        .args([COMMAND_ARGUMENTS, extra_command_arguments].concat())
        .output()
        .expect("git command failed");

    // Output format (whitespace formatting added for readability):
    //
    // 21   \t  0   \t  .gitignore  \n
    // 7    \t  11  \t  Cargo.toml  \n
    // 3    \t  0   \t  src/main.rs \n
    if output.status.success() {
        let mut total: usize = 0;
        let mut num: Vec<u8> = vec![];
        let mut in_word = false;

        for c in output.stdout.iter() {
            match c {
                b'\t' => {
                    if !num.is_empty() {
                        total += integer_from_slice(num.as_slice()) as usize;
                        num.clear();
                    }
                }
                b'\n' => in_word = false,
                b'0'...b'9' => {
                    if !in_word {
                        num.push(*c);
                    }
                }
                _ => in_word = true,
            }
        }
        println!("{}", total);
    } else {
        if let Some(code) = output.status.code() {
            eprintln!("git command exited with status {}", code)
        } else {
            eprintln!("git command killed with a signal")
        }
        process::exit(1);
    }
}

fn integer_from_slice(slice: &[u8]) -> usize {
    let n: usize = str::from_utf8(slice)
        .expect("not a string")
        .parse()
        .expect("not a number");
    n
}

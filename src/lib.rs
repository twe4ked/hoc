//! A command line tool to calculate the hits-of-code metric in a source code repository using Git.
//!
//! You can read more about hits-of-code metric in this blog post:
//! [Hits-of-Code Instead of SLoC][blog_post].
//!
//! Based on the [Ruby version by Yegor Bugayenko][ruby_version].
//!
//! [blog_post]: https://www.yegor256.com/2014/11/14/hits-of-code.html
//! [ruby_version]: https://github.com/yegor256/hoc
use git2::{DiffOptions, Repository};

pub fn hoc(_find_renames_and_copies: bool) -> Result<u64, git2::Error> {
    let repo = Repository::open(".")?;

    // "git log",
    // "--pretty=tformat:",
    // "--numstat",
    // "--ignore-space-change",
    // "--ignore-all-space",
    // "--ignore-submodules",
    // "--no-color",
    // "--diff-filter=ACDM", // added copied deleted modified

    // TODO: Are there other things we should ignore? I.e. ignore_filemode()
    let mut diffopts = DiffOptions::new();
    diffopts.ignore_whitespace(true); // ASSUMING: --ignore-all-space (might also need ignore_whitespace_eol?)
    diffopts.ignore_whitespace_change(true); // ASSUMING: --ignore-space-change
    diffopts.ignore_submodules(true); // ASSUMING: --ignore-submodules

    // TODO: find_renames_and_copies
    //
    // Enabling this option causes the git command to be significantly slower.
    //
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

    let mut c: u64 = 0;

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let commits = revwalk.filter_map(|id| id.map(|id| repo.find_commit(id)).ok());

    for commit in commits {
        let commit = commit?;

        let old_tree = if commit.parents().len() == 1 {
            let parent = commit.parent(0)?;
            Some(parent.tree()?)
        } else {
            None
        };
        let new_tree = commit.tree()?;

        let stats = repo
            .diff_tree_to_tree(old_tree.as_ref(), Some(&new_tree), Some(&mut diffopts))?
            .stats()?;

        c += stats.files_changed() as u64;
        c += stats.insertions() as u64;
        c += stats.deletions() as u64;
    }

    Ok(c)
}

# Astral Filing Cabinet

The Astral Filing Cabinet (AFC) is a means for working with large files through
pointer files committed to Git.

This software is just getting started and is fully of baby dragons who haven't
had their breakfast.

## Remotes

AFC plans to support pushing and pulling data from multiple types of remote
storage:

- [ ] Local file tree
- [ ] SFTP
- [ ] S3 (and compatible stores, such as Minio)
- [ ] WebDAV (with only HTTP[S] required for download)
- [ ] Google Drive
- [ ] Backblaze B2

## Other Software

Unlike other solutions like Git LFS and `git-annex`, AFC keeps the pointer files
visible (typically as `.afc` files), and the `afc` command synchronizes your
working tree state with the pointer files.

This is heavily inspired by [Data Version Control][dvc] — it aspires to be an
equivalent to DVC's file management without the pipeline, parameters, and other
such features.  Just the data.  Eventually, it may also support DVC's pointers
(`.dvc` files and `dvc.lock` files) and cache to allow you to use AFC to fetch
and manipulate data in a DVC repository.  I'm focused on getting the core
functionality working before adding support for multiple layouts.

Right now AFC is only tested with Git, but there is no reason why it could not
be used with Mercurial or another DVCS.  It just needs a couple things:

- Commit pointer files (`.afc` files) to the VCS
- Ignore the tracked files in the VCS

[dvc]: https://dvc.org

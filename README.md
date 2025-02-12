# Lindy Hop videos

This is a personal platform to host and share videos from my classes and camps of Lindy Hop.

## Data format

This section documents the format of the files in the `data` folder, which is not commited into git.

### `data/new_lindy_files`

Produced by the command `copy-new-videos`, which will copy the most recent videos from the connected Android phone.

### `data/tagging_in_progress/part-X`

Produced by the command `prepare-new-videos-for-tagging`, which will move the videos from `data/new_lindy_files` into
separate folders to easy the inclusion of a large batch of files

### `data/copied_files.json`

Auto-managed file to remember all files that were copied from the Android phone

### `data/restrictions.json`

List the access rules used to protect some videos with a code.

### `data/build/tags.txt`

Contains all video names and related tags. This can be manually edited later, making it easy to batch update the whole
library. This is auto-updated by the command `build`.

### `data/build/public/videos`

Contains all videos in the library. The file name is the hash of its contents. Videos are copied from
`data/tagging_in_progress` into here by the command `build`.

### `data/build/public/thumbnails`

Contains all the thumbnails for the videos. The file name is the truncated hash of the video content. The thumbnails are
public information, but the full video name should only be known if the video is public or the user has the correct code
to access it.

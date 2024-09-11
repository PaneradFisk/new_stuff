# new_stuff
cli tool to create files and dirs
***
Got annoyed by having to run mkdir and touch with flags and whatnot.  
Idea is to be able to write `ns cool_dir/sub_dir/my_awesome_file.txt` and if none of it exists, it will create it all, if any of it exists, then it'll skip creating it.  
And if you specify `ns /cool_dir/other_file.txt` it will understand it's an absolute path.  
If you write `ns cool_dir/other_dir/` it will understand to just create dirs and no files.

***
[AGPL LICENSE](https://github.com/PaneradFisk/new_stuff/blob/main/LICENSE).

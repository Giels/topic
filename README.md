# Topic

A WIP, abandoned anonymous forum in rust.

Beside the admin pages, it should work with a valid DB and config. It supports markdown, including image embeds and text markdown, keeps track of posts and correctly identifies admin posts. Passwords and post-deletion work too. It is a hybrid between classical forums and imageboards and was designed to have volume-based activity timestamp thread pruning instead of fixed-capacity infinite-threads as in imageboards, or infinite-capacity infinte-threads as in classical forums.

Database scripts have been recreated in a hurry after being lost. Should work in its current state, but there is poor error handling and no i18n. In particular, you must `cargo run` from the `src/` subdirectory, or the server will crash horribly as it fails to find any of its config and other resources.

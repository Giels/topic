# Topic
A WIP, abandonned anonymous forum in rust.

The databse init script was lost and I gave up on this, but beside the admin pages, it should work with a valid DB and config.
It supports markdown, including image embeds and text markdown, keeps track of posts and correctly identifies admin posts.
Passwords and post-deletion work too.
It is a hybrid between classical forums and imageboards and was designed to have volume-based activity timestamp thread pruning
instead of fixed-capacity infinite-threads as in imageboards, or infinite-capacity infinte-threads as in classical forums.

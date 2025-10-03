# EasyInit

An alternative init system and service manager for Linux

# Packaging Guidelines
> Since the software is still in development and not released, please don't package EasyInit
> in the current state.

Install it in `/sbin` directory.
If the init is the one that ships with the distribution, add a symlink pointing to the executable,
with the link name being `/sbin/init`, otherwise allow the user to set kernel parameters manually to prevent accidentally
making a unbootable system or best case a recoverable system where the user is unsure

# License
We use the LGPLv3.0-or-later license.

## TL;DR (from a non-lawyer)

* You're allowed to modify this software, free of charge
* With the exception with the use of dynamically linking, (and limitedly, statically linking), you must license your code under LGPLv3 or a later version or  sublicense that follows the terms (i.e using GPLv3)   
* Statically linking is allowed if linkable object files or the source code is provided.


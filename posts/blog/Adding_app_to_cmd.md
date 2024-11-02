---
title: Adding your terminal app to your PATH environment in Linux
description: This is a guide to making your terminal app accessible from anywhere in your terminal directory by just calling a single identifier or name of the app in the terminal.
date: 2024-06-12
published: true
---

# App to PATH in linux

To make your Rust command-line application accessible from anywhere on Linux, you need to ensure that its executable binary is available in a directory that's included in the system's PATH environment variable. Here's how you can achieve this:

1. **Build Your Rust Application:**
   Make sure you've built your Rust application and have the executable binary ready.

2. **Choose a Directory:**
   Decide on a directory where you want to store your executable. Common choices are `/usr/local/bin` or `~/bin` (if you want to keep it in your home directory). The former is accessible system-wide, while the latter is user-specific.

3. **Move the Executable:**
   Move your executable binary to the chosen directory. You might need superuser privileges (using `sudo`) to move it to system-wide directories like `/usr/local/bin`.

   ```bash
   sudo mv /path/to/your/executable /usr/local/bin
   ```

   Or, for a user-specific directory:

   ```bash
   mv /path/to/your/executable ~/bin
   ```

4. **Add the Directory to PATH:**
   To make your application executable from anywhere, you need to ensure that the chosen directory is included in your system's PATH. This is typically done by modifying your shell's configuration file (e.g., `~/.bashrc` for Bash users or `~/.zshrc` for Zsh users).

   Open the appropriate configuration file in a text editor:

   ```bash
   nano ~/.bashrc
   ```

   Add the following line to the file, replacing `~/bin` with the path to the directory you've chosen:

   ```bash
   export PATH="$PATH:~/bin"
   ```

   Save the file and exit the text editor.

5. **Reload the Shell:**
   To apply the changes, either restart your terminal or run:

   ```bash
   source ~/.bashrc
   ```

   This will reload the configuration and update your PATH.

6. **Test the Application:**
   You should now be able to run your Rust application from anywhere in the terminal:

   ```bash
   your-app-name
   ```

Your Rust command-line application should now be accessible from anywhere on Linux, allowing you to run it regardless of your current working directory.

Remember to replace placeholders like `your-app-name` and `/path/to/your/executable` with the actual names and paths relevant to your application.

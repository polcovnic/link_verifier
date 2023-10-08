# Link Verifier

Link Verifier is a CLI tool designed to verify links within given `.md` files. Designed with versatility in mind, it can seamlessly integrate with GitHub Actions, making it an essential tool for maintaining up-to-date and accurate links in your markdown files.

## Features

- **Local Filesystem Verification**: Check if the links that refer to locations in the filesystem are correct.

- **External Link Verification**: Examine external links that require an active internet connection.

- **Filesystem Link Correction**: When an incorrect link to a file or directory is detected, the tool suggests potential fixes by searching for filenames or directories with similar names.

## Motivation

Maintaining updated links in repositories, especially in large projects, can be a tedious task. Broken links can confuse readers or lead to missing resources. The Link Verifier aims to alleviate this problem by not only identifying these issues but also suggesting corrective measures. Think of the time you'd save if this tool took care of links in your repository!




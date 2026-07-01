# Restructure Flat Documentation Into Organized Hierarchy

A project's `docs/` directory has grown organically into a flat mess of files
with no clear organization. Users struggle to find relevant documentation.

Here is the current flat file listing:

```
docs/
├── authentication.md
├── installation-linux.md
├── installation-mac.md
├── installation-windows.md
├── api-users.md
├── api-posts.md
├── api-comments.md
├── api-rate-limiting.md
├── deployment-aws.md
├── deployment-docker.md
├── development-setup.md
├── testing-guide.md
├── architecture-overview.md
├── faq.md
└── contributing.md
```

Produce a `docs/README.md` that:

1. Shows the new directory tree structure as a fenced code block, with files grouped
   into subdirectories by category.
2. Acts as a documentation index with navigation links organized by user journey
   (Getting Started, Guides, Reference, Contributing).
3. Groups installation files into an installation/ subdirectory.
4. Groups API files into an api/ subdirectory.
5. Groups deployment files into a deployment/ subdirectory.
6. Uses kebab-case filenames, keeps hierarchy shallow (max 3 levels), and preserves
   every original subject somewhere in the new structure.

Produce a single file `docs/README.md` containing the new tree and navigation.

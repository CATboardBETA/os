# Style Guide

1. All code must be formatted using `rustfmt` (`cargo fmt`), under default settings.
   1. Specific exceptions can be made using `#[rustfmt::skip]` on a case-by-case basis.
2. Clippy should be run prior to submitting PRs.
   1. Again, specific exceptions can be made. 
   2. If using a JetBrains IDE (as I am), you may need to specify these options to the external linter:
      1. "--keep-going -Zbuild-std=core,alloc,compiler_builtins -p kernel"
      2. Toolchain: "nightly"
      3. This is because [JetBrains currently uses --all-targets (YouTrack)](https://youtrack.jetbrains.com/projects/RUST/issues/RUST-17668/Add-ability-to-fully-customize-cargo-check-command-line)
3. All functions *should* have documentation.
   1. You may notice that they currently do not, the next few commits should fix that.
4. Try to make your code testable
   1. Add tests if you can, 
   2. If not, they'll be added later.
5. References to code or programming terms in comments should be enclosed within backticks.
6. Full code snippets should or examples:
   1. Be enclosed within triple backticks.
   2. Use the attributes in [the `rustdoc` documentation](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#attributes).
   3. Use `#` to hide lines when appropriate, but not excessively.
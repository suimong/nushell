#!/usr/bin/env nu

# Define your dev files here
let dev_files = [
  "flake.nix"
  "flake.lock"
  ".envrc"
  "start-feature.nu"
]

# Input: feature branch name
def main [
  feature_branch: string
] {
  let dev_branch = "dev"
  let main_branch = "origin/main"

  print $"Creating feature branch: ($feature_branch) from ($main_branch)..."
  ^git fetch origin
  ^git checkout -b $feature_branch $main_branch

  for file in $dev_files {
    let file_in_dev = (try { ^git show $"($dev_branch):($file)" } catch { null })

    if ($file_in_dev != null) {
      ^git show $"($dev_branch):($file)" | save --force $file
      print $"Checked out ($file) from ($dev_branch)"
    } else {
      print $"Warning: ($file) not found in ($dev_branch), skipping"
    }

    let exclude_path = ".git/info/exclude"
    let already_excluded = (open $exclude_path | lines | any {|line| $line == $file })

    if not $already_excluded {
      print $"Excluding ($file) from git tracking"
      $file | save --append $exclude_path
    }
  }

  print $"✅ Branch ($feature_branch) ready with dev files (not tracked by Git)"
}


<div align="center"> <img src="https://raw.githubusercontent.com/p-tupe/zettel-merken/refs/heads/main/logo.png" alt="Zettel Merken Image"> </div>

<hr />
<p align="center"><strong>Supercharge your learning by combining two of the most revolutionary ideas in knowledge enhancement!</strong></p>
<hr />

# Introduction

Zettel stands for "note" and Merken stands for "remember" in German. A literal translation would imply "Remember your notes", but that is an overly simplistic definition of what the title stands for.

To be precise, Zettel Merken is a fusion of two impactful ideas in the field of knowledge management and learning enhancement: "Zettelkasten" and "Spaced Repetition". See [#What-is-Zettelkasten?] for more.

## Setup

**_NOTE: Code was written in and tested on MacOS 26.6 with Rust edition 2024_**

1. Install

```bash
cargo install zettel-merken
```

2. Initialize

```bash
zettel-merken init <notes_dir>
```

Create a `config.json` (and `zettelmerken.db`) in either `~/.config/zettelmerken/` or your systems default config directory. I'd say back these up somewhere on the regular.

3. Run

```bash
zettel-merken run
```

My goal is to have the program be fire 'n forget type. You can edit the config anytime, it'll pick it up on each run and work accordingly. It will also sync your notes' folders before each run so adding/removing notes does not require changing your review setup. The idea is to tie this in with a launchd/systemd like runner, see [#Macos-launchd-for-daily-run] for an example.

## Configure

```bash
zettel-merken config
```

This opens the config file in your default editor. A default config is created on init.

```json
{
  "version": ..,
  "notes": [ .. ],
  "exclude": [ ".*" ],
  "max_per_review": 5,
  "notification": "local"
}
```

`version` is used internally to track upgrades.

`notes` is a list of directories you'd like to include for review. Initially, it will contain the `notes_dir` you supply during init; you may add other paths in there.

`exclude` is a glob of files to exclude, following standard unix glob patterns.

`max_per_review` is the maximum number of notes to be reviewed in a single run. The list is sorted in the ascending order of last modified time, so your oldest notes bubble up first for review.

`notification` is either `local`, `mail` or `slack`. The latter two are wip, and `local` is currently only tested on macos.

## What is Zettelkasten?

The [Wikipedia article](https://en.wikipedia.org/wiki/Zettelkasten) defines zettelkasten as "The zettelkasten is a system of note-taking and personal knowledge management used in research and study".

<br />
<div align="center">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/p-tupe/zettel-merken/refs/heads/main/zettelkasten.png">
      <img src="https://raw.githubusercontent.com/p-tupe/zettel-merken/refs/heads/main/zettelkasten-light.png" alt="Zettelkasten Image">
    </picture>
</div>
<br />

[zettelkasten.de](https://zettelkasten.de/introduction/) is a wonderful little site that is all about, well, zettelkasten. Do read the [introduction](https://zettelkasten.de/introduction/). To pick an excerpt from there:

> A Zettelkasten is a personal tool for thinking and writing. [...] The difference to other systems is that you create a web of thoughts instead of notes of arbitrary size and form, and emphasize connection, not a collection.

If I had to explain the concept to someone in a hurry, I'd say: Zettelkasten = Mind Map + Flash Cards

Of course, it is _not_ entirely either, so I would recommend following the links above for a more detailed understanding.

### How I use Zettelkasten

I take notes for _everything_! From doing research for my web novels to learning new languages, from my transaction history to ongoing projects, suffice to say I have a _lot_ of notes. However, I have come to realize that not all my notes are highly connected. Rather, a collection of notes is usually extremely cohesive with one another but largely decoupled from the rest. So I follow a sort-of watered-down system like so:

0. One single folder called "notes"
1. An index (No direct notes made here, only links), usually named "index.md".
2. A "hub" for each topic. Imagine a hub as a collection (like a notebook or a drawer). One hub usually points to one large topic.
3. A "zettel" for each atomic piece of info. All zettels for a topic are linked into the hub and are stored in a folder usually named after the same.

That's it! To expand on the above, here is a sample of my current notes directory:

```sh

~/Notes
├── index.md
├── books-and-articles.md
├── books-and-articles
│  ├── atomic-habits.md
│  └── ledger-accounting.md
├── code-notes.md
├── code-notes
│  ├── python.md
│  └── vim.md
├── learning-french.md
├── learning-french
│  ├── basics-1.1.md
│  ├── basics-1.2.md
│  └── basics-1.3.md
├── transactions.md
└── transactions
   ├── 01-2022.md
   └── 02-2022.md

```

As you can see above, I have hubs after each topic: zettel-merken, books-and-articles, learning-french, etc. Each hub has a file.md and folder with the same name. I take all my notes in neovim in markdown format No special plugins, just a couple of functions and mapping. See wiki.

Thus, my `index.md` will look like:

```markdown
# INDEX

- [Learning French](./learning-french.md)
```

and my `learning-french.md`:

```markdown
# Learning French

- [Basics 1.1](./learning-french/basics-1.1.md)

- [Basics 1.2](./learning-french/basics-1.2.md)

- [Basics 1.3](./learning-french/basics-1.3.md)
```

Concerning zettels, I try to have them in an easily digestible format. Each zettels has a microscopic focus on the information it is trying to convey. That is - all the content inside a zettel must directly relate to a single matter or flow in one direction. The size of the file is irrelevant, although I try to keep it short and simple.

For example, `basics-1.1` might look like:

```markdown
# Basic 1.1

## Level 0

- un = a (sounds: un)

- et = and (sounds: ae)

- un chat = a cat (sounds: un shaa)

- un homme = a man (sounds: un oum)

- un garcon = a boy (sounds: un gars-on)

- un chat et un homme = A cat and a man
```

Also, I _try_ to avoid more than one layer of nesting below the "notes" folder but in some cases, it is inevitable. However, there should never be a need to go beyond two layers.

After following the above system consistently for a few months, you'll have a decent-sized collection of notes all linked together in a proper structure. That being said, simply "collecting" notes is never going to help you learn in the long term. That is where the Spaced Repetition comes in!

## What is Spaced Repetition?

Excerpt from [Wikipedia article](https://en.wikipedia.org/wiki/Spaced_repetition):

> The basis for spaced repetition research was laid by Hermann Ebbinghaus, who suggested that information loss over time follows a forgetting curve, but that forgetting could be reset with repetition based on active recall.

<br />
<div align="center">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/p-tupe/zettel-merken/refs/heads/main/spaced-repetition.png">
      <img src="https://raw.githubusercontent.com/p-tupe/zettel-merken/refs/heads/main/spaced-repetition-light.png" alt="Spaced Repetition Image">
    </picture>
</div>
<br />

Excerpt from [e-student.org](https://e-student.org/spaced-repetition/):

> Spaced repetition is a memory technique that involves reviewing and recalling information at optimal spacing intervals until the information is learned at a sufficient level.

It is quite difficult to manually track hundreds of notes and review a set everyday. You'd have to keep logs of when each topic was visited, how many repetitions were completed, when the next review will be and so on. Quite cumbersome!

That is were Zettel Merken comes into play. Not only does this program keep track of your every note and its schedule, it also automatically notifies about notes that are due for review for the day! How awesome is that? It is quite easy to use too!

## Macos launchd for daily run

1. Save this in `~/Library/LaunchAgents/com.ptupe.zettelmerken.plist`

```xml
<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
  <dict>
    <key>Label</key>
    <string>com.ptupe.zettelmerken</string>
    <key>ServiceDescription</key>
    <string>Zettel Merken Daily Review Run</string>
    <key>ProgramArguments</key>
    <array>
      <!-- Replace your_username with whatever you have -->
      <string>/Users/your_username/.cargo/bin/zettel-merken</string>
      <string>run</string>
    </array>
    <key>StartCalendarInterval</key>
    <dict>
      <key>Hour</key>
      <integer>11</integer>
      <key>Minute</key>
      <integer>00</integer>
    </dict>
  </dict>
</plist>
```

2. Run the following command to enable

```bash
launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/com.ptupe.zettelmerken.plist
```

## Roadmap

- Add linux/windows support
- Add email support
- Add slack support
- Add view to see reviewed notes
- Add preview to see upcoming notes

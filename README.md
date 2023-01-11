# Advent of Code 2022

What it says on the tin. This year is in Rust, which I am enjoying
very much.

There is [a retrospective blog post](https://blog.sulami.xyz/posts/aoc-in-one-second/) about this project.

I have some self-imposed rules and goals:

1. I'm trying to minimise library use. I use `nom` in some places
   where manual input parsing would be laborious and error-prone
   otherwise, but for example would refrain from pulling a graph
   library off the shelf.
2. No manual steps, use the raw input and output solutions that can be
   pasted directly into the website. An exception here are the puzzles
   that essentially require OCR, the "which characters does this
   spell" kind of ones.
3. Keep up the quality. I am trying to use reasonable abstractions,
   include comments and some tests. The expected lifetime of this code
   a is a few days, but I'm using this as an exercise to improve my
   Rust skills, not for code golfing.
4. Try to go fast. I'm currently on track for the full solution to run
   in under 100ms, but even under 1s would be great. All it takes for
   this to fail is one particularly crunchy day, but fingers crossed.
   I might resort to multi-threading if that can help.
5. Self-found only. No looking up approaches or solutions elsewhere.
   Looking up algorithms is fine though.

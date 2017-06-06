# CourseSynth

## Initial documentation from form
Project Title: CourseSynth

Project Summary: Our group intends to create a website that is designed to tutor RPI and K-12 students using autonomous programs instead of human tutors. The programs will walk students through example questions while giving them a chance to ask for a more detailed explanation for any particular steps. We also intend to create step by step calculators for certain kinds of math problems and transformations.
Similar Projects/Overlap: There don’t seem to be any RCOS projects that are particularly similar to this. There are however, many sites that provide interactive tutorials. There are also several step by step calculators available online for simple math problems. Many of these calculators do seem to have limitations that we could potentially improve on.

Project Audience: Struggling students or those who simply want an extra way to study.

Project Team: We don’t have specific roles yet but we do have 3 members.

<table>
<tr><td>RCOS Slack ID</td><td>Name</td></tr>
<tr><td>pauloa</td><td>Ajay Paulose</td></tr>
<tr><td>kenjliu</td><td>Jinxin Liu</td></tr>
<tr><td>aweinstock</td><td>Avi Weinstock</td></tr>
</table>

Project Timeline: Our tentative goal for the 6 week summer session is to create the website and then focus on and implement tutorial programs for a low level math course here at RPI. Calculus 1 is the course we are most likely to focus on.

## Planned features list

- Generating problem sets (parameterized by some complexity?) for a particular course
- Solving given [math] problems and providing an explanation trace, step-by-step
- TODO: think of more stuff based on progress?

## Slack Transcripts / Brainstorming
```
pauloa
I have a few ideas but they all seem kind of simple compared to the past projects. My first idea is to write a program that takes a poor google translation from any language to English as an input and refines it into more fluent English. My second idea would be to write a program that basically tutored students about a specific course here at RPI. Possibly a math course so that we could work on step by step calculators for certain types of questions as well. I'm totally open to joining someone else's project but I'll probably just write my proposal on one of these.
```

```
aweinstock
@pauloa The first idea (improved machine translation) is open-research-problem level of difficulty in NLP, and doesn't seem to have a well-defined success metric (figuring out what makes a translation "poor" or "fluent" is part of the problem to solve). If you decide to go with that, I think PCFGs and RNNs are good algorithms to start experimenting with, and the Penn Treebank is a good starting dataset. The second idea seems more straightforward, and has well-known algorithms: differentiation can be done via recursion on syntax trees, integration has the Risch algorithm (for a large family of functions), and systems of linear equations have Gauss-Jordan elimination. A symbolic execution engine would be useful for security too (in addition to use as a teaching aide), so I'd be interested in working on it.
```

```
aweinstock
I agree with the selection of Calculus 1 as a first choice. The talk from the economist got me thinking that maybe we could eventually plan on including economics material as well (as a long term goal), and there's some overlap with Calc {1,2} material. I think I mentioned wanting the mathematical engine to be useful efficiently machine-solving [security] problems (in addition to to emitting human-readable traces/derivations), the calculus focus will probably be a good excuse to teach myself differential cryptanalysis.In terms of program structure, there's a few components we could split it into:

- a mathematical engine for generating problem sets and solutions, possibly with subcomponents per course (I'd want this in Rust, for efficient/parallel solving algorithms)
- a web server/backend for keeping track of sessions/invoking the problem set generator (this part could be in any language, ideally something compiled+GC'd like Haskell/Scala)
- some web frontend stuff (HTML, CSS, JS) to present the problem sets/course descriptions/derivations aesthetically
```

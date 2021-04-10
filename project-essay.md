# Chess Engine Project

## Overview
* [Why I will make a chess engine](#why-i-will-make-a-chess-engine)
* [Picking a language](#picking-a-language)
	- [C#](#c\#)
	- [C++](#c++)
	- [Rust](#rust)
* [Board state representation](#board-state-representation)
* [Graphical User Interface](#graphical-user-interface)
* [Minimax](#minimax)
* [Finding legal moves](#finding-legal-moves)
* [Minimax improvements](#minimax-improvements)
* [Evaluation](#evaluation)
* [Summary](#summary)

## Why I will make a chess engine

Chess is a game complex enough that no strategy has been found that guarantees the best outcome. This allows a programmer to be creative with their implementation and ensures that the algorithm can always be improved. This makes chess engines an excellent candidate for a project to improve a programmer's skills.

In addition, chess has always been of interest to computer scientists because creating a chess playing program proves the ability of a computer to make decisions of its own.

"Although perhaps of no practical importance, the question is of theoretical interest, and it is hoped that a satisfactory solution of this problem will act as a wedge in attacking other problems of a similar nature and of greater significance." - [Philosophical Magazine, Ser.7, Vol. 41, No. 314 - March 1950 - Programming a Computer for Playing Chess by Claude E. Shannon](https://www.pi.infn.it/~carosi/chess/shannon.txt).

## Picking a language

### C# 

Prior to this project - C# was the only language I knew well enough to feel confident starting a new project in. For this reason, I initially started the chess engine in C#. However, I discovered that it was slow compared to lower level languages. If I were to continue creating the engine in C# - the search depth would have to be reduced to compensate for the reduced speed. Using a lower level language would enable me to have a greater search depth as the moves could be processed faster.

### C++

C++ is the [most popular system level language](https://insights.stackoverflow.com/survey/2020#most-popular-technologies). This, combined with its object oriented capability, made it seem like a logical choice for my chess engine. However, I found that the use of header files, and all other aspects of its linking, was tedious and slowed the development process. For this reason I decided to look into other, more modern, system level languages that have better support for multi-pass compilation which would negate the need for header files.

### Rust

Rust is the language that "[combines low-level control over performance with high-level convenience](https://blog.rust-lang.org/2015/05/15/Rust-1.0.html)". This appeared to be a solution to the inconvenience of C++ while keeping the speed that low level control enables. Rust has also been voted the "Most Loved" programming language on Stack Overflows developer surveys for the past 6 years! 
\([2015](https://insights.stackoverflow.com/survey/2015),
[2016](https://insights.stackoverflow.com/survey/2016),
[2017](https://insights.stackoverflow.com/survey/2017),
[2018](https://insights.stackoverflow.com/survey/2018),
[2019](https://insights.stackoverflow.com/survey/2019),
[2020](https://insights.stackoverflow.com/survey/2020)\).

I started learning Rust by following the [official rust book](https://doc.rust-lang.org/book/) and its advantages quickly became apparent. It is [fast](https://github.com/kostya/benchmarks/blob/master/README.md); It has an easy to use package manager ([cargo](https://doc.rust-lang.org/cargo/)); Its syntax is consistent (especially when contrasted with my C++) and documentation for the language and its crates are of high quality. Rust's downsides are that its [borrow checking rules can have quite a steep learning curve](https://arxiv.org/pdf/1901.01001.pdf) and that its ecosystem is still maturing.

After creating a few simple Rust programs, I felt I was ready to start working on the chess engine. I would encounter the common beginner problem of "[fighting the borrow checker](https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html)", but by the end of the project I had internalised Rust's ownership model and had refactored all mistakes I had made while learning it.

## Board state representation

The representation of board states in memory will affect the performance of all aspects of the chess engine as the manipulation and assessment of these states is the sole purpose of the program. The most important part of a board state is the locations of the pieces on it. There are several [common ways a board state's pieces can be expressed in memory](https://web.archive.org/web/20130212063528/http://www.cis.uab.edu/hyatt/boardrep.html). I chose to use a [mailbox addressing scheme](https://www.chessprogramming.org/Mailbox) that is indexed by two 8-bit bytes: one for the rank and one for the file. This is implemented using a two dimensional array of size 8 by 8 that holds an enumerated type of possible pieces for each location. It would have been possible to use a one dimensional array of size 64 to halve the memory size of an array index. However, this would increase complexity which I was not willing to do as I was still rather new to the Rust language. It is worth mentioning [bit boards](http://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/rep.html) - in chess, these use a 64 bit value for the locations of each piece type (perfect for modern 64-bit processors). However, this would increase complexity even more as it would involve using many bitwise operations throughout the entirety of the program to achieve any substantial performance increase from bit boards.

## Graphical User Interface

Before working on any of the logic that makes the computer play its own moves, I wanted to create a simple graphical user interface to render the board state. This would enable quick visual testing of the possible move generator that I was planning on making next. I needed a way to create a cross-platform window and draw some images onto it so I started looking for a 2D rendering crate on "[are we game yet](https://arewegameyet.rs/ecosystem/2drendering/)" (a website that documents the best Rust libraries for making video games). I chose to use [ggez](https://github.com/ggez/ggez) as it seemed the simplest, enabling me to start working on the algorithm itself sooner.

## Minimax

There are two main types of chess computer: [minimax](https://link.springer.com/article/10.1007/BF01448847) and ones based on machine learning. While a minimax method can use [machine learning for some aspects of its board evaluation](https://www.researchgate.net/publication/322539902_Learning_to_Evaluate_Chess_Positions_with_Deep_Neural_Networks_and_Limited_Lookahead), when I refer to a machine learning chess algorithm I am talking about something that uses machine learning for move selection, not just evaluation. [Alpha zero](https://arxiv.org/pdf/1712.01815.pdf) is an example of a machine learning chess engine and [Stockfish](https://stockfishchess.org/about/) is an example of a minimax engine.

I am choosing to make the minimax type because it is simpler and much easier to debug, but perhaps in the future I could add some kind of reinforcement learning into the evaluation function.

The first step in making a minimax algorithm it to write a function that finds all the legal moves in any given position.

## Finding legal moves

My initial idea for finding all legal moves was to iterate over every board position and call a method that would find that piece's legal moves and append it to a vector. However, this resulted in a [huge and unreadable method](https://github.com/Chris-F5/ChessEngine/blob/319f796d2d8617e8b8181024276997ceb3982dfa/src/rules.rs#L163-L448) (285 lines!) responsible for finding an arbitrary piece's legal moves. The solution to this was to modularise finding legal moves into 8 different function calls, each of which could independently iterate over the board positions and append (or remove) legal moves from the mutable reference to the legal moves vector it was passed. The 8 so called "action_rule" structures are as follows: pawn actions, knight actions, diagonal actions (used for both bishop and queen), straight actions (rook and queen), king actions, castling actions, remove illegal actions (removes actions that would leave yourself in check) and an optional remove unsafe actions (used for the final depth of minimax).

## Minimax improvements

My first implementation of the minimax algorithm could only search to a depth of 4 without taking more than a few seconds. To increase this, I implemented [alpha beta pruning](http://www.chilton-computing.org.uk/acl/literature/books/gamesplaying/p004.htm#index01). This significantly improved the performance of the program and enabled me to easily reach a depth of 5. Sorting the order in which nodes on the search tree were evaluated could increase the benefit of alpha beta pruning further - so I wrote a "[quick evaluate function](https://github.com/Chris-F5/ChessEngine/blob/c839f11fb86962aa9d55e15b181c1c6953ffc6d0/src/best_action_finder/evaluator.rs#L82-L86)" which would be run for all board positions searched before the full evaluation was run. Then, the result of the quick evaluation would be used to sort the order in which the the full evaluation was run (best moves first). I underestimated the huge increase in performance this resulted in. Because my quick evaluate was so similar to my full evaluate, the number of board positions considered was almost square-rooted!

Another improvement I wanted to make was to search what initially seemed like better moves to a greater depth. This was implemented by selecting the first few board states in the lists that have been sorted by the quick evaluate and increasing their depth. This improvement did not change the ability of the chess engine significantly and after some experimentation, I found that using it to search to a max depth of 7 (only one more than was being searched previously) was best.

## Evaluation

My evaluation function almost exclusively uses [piece square tables](https://www.chessprogramming.org/Piece-Square_Tables). As an amateur chess player, I did not feel confident crafting my own evaluation function from the ground up. The evaluation function I have written is heavily based on one proposed by [Tomasz Michniewski](https://www.chessprogramming.org/Tomasz_Michniewski). [https://www.chessprogramming.org/Simplified_Evaluation_Function#Piece-Square_Tables]. My evaluation function however does not support multiple game phases so I modified the king's square table.

Initially, my engine played very weakly in the endgame. It could not achieve checkmate when it had a king and a queen against just a king. This was because it was using the same piece square tables as the rest of the game and could not see a forced checkmate within its searched depth. To solve this, I added end game tables. I chose to use a [3-4-5 piece syzygy end game table](https://chess.massimilianogoi.com/download/tablebases/). Uncompressed, this was almost 1 gigabyte! I did not want to use a larger table base as I wanted others to be able to download and try out the engine which would not be practical with a huge download. I probed the end game tables with [shakmaty-syzygy](https://github.com/niklasf/shakmaty-syzygy). This required me to convert my board states into Forsyth–Edwards Notation: a format [shakmaty](https://github.com/niklasf/shakmaty) could understand. The addition of endgame tables did not make a huge difference as most games it plays do not reach a board state with fewer than 6 pieces. However, it is comforting to know that if it gets that late into a game - it will not simply draw a winning position.

## Summary

I have created a chess engine that has an Elo rating of approximately 1300 - 1500. In the process, I have learnt: the Rust programming language, important skills related to writing fast code, and academic research skills. 

The program's features and limitations are as follows:

### Features

* Easy to use GUI
* Max depth of 7
* Endgame tables
* Alpha beta pruning
* Special moves (En passant, Castling, Queening)

### Limitations

* Only plays black
* Evaluation does not adapt according to game phase
* Evaluation only considers material and piece-square tables
* Does not adapt depth according to how long it is taking to play a move
* Pawns can be promoted only to queens
* No draw by repetition
* No fifty-move rule
* No draw by insufficient mating material

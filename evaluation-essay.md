# Project Evaluation

## Research

At the beginning of the project I did not appreciate the value of thorough research, this caused me to restart the project twice in different programming languages. If I had spared the time to do in depth research of available languages before starting development, I would have saved much time. Whilst midway through developing what would become the final program, I had partially realised the importance of research. However, I was not confident in reading many academic papers that seemed complex and daunting. Nearing the completion of the project, I began to peer into papers which I had once ignored and found many optimisations and improvements for algorithms I was using. Unfortunately, by then the project was almost done and I did not have the time to implement these improvements. Features I should have found earlier and therefore was not able to use include: iterative deepening, faster legal move finding, better evaluation and better board state representation.

A large portion of my research was spent learning the Rust programming language. This was quite difficult as Rust has some unique features that are notoriously hard to learn. Fortunately, the [official rust book](https://doc.rust-lang.org/book/) provided a great way to learn the language. I think learning Rust was one of the most successful aspects of the project as it is very useful to know a systems level language.

## Code

## Algorithms

The legal move finder used in the final project has some drawbacks. It is fairly inefficient because after each move it has to make sure its not in check which requires checking all the opponents moves to see if its king can be captured. If I had spared more thought into the legal move generator, it could have been made much faster.

The evaluation function is fast but it is very basic, only consisting of one piece square table for each piece. I should have realised that the evaluation function is more important than other features such as variable depth and end game tables, and therefore should have dedicated more time to improving it. Factors that could have been included in the evaluation function are: different piece square tables for different game phases, pawn structure, bishop pairs and king safety. I believe these features would have significantly improved the ability of the engine and could have been implemented within the same time it took to implement end game tables.

### Testing Code

I implemented a very simple unit test for the legal move finder. However, it did not catch any bugs and missed many that I found manually. The issue with the test is that it only checked if specific sequences of moves were legal. A better approach would be to calculate the total number of move sequences that could be played from a given position to a given depth and check this total matched the expected result. Although this would not tell me what was causing the unexpected result, at least it would catch almost all bugs that could occur in the legal move finder.

### Compatibility

My chess engine does not have good compatibility with other chess related programs. This is partly because of its limited support for standard chess notations such as FEN and PNG, but also because the program has no non-graphical interface that other programs can interact with. The limited support for FEN made querying end game tables difficult because other programs that read the table base usually take standard chess notation as a parameter. The lack of an API made it impossible to automatically play against other engines as the only way of playing it is through the GUI. Now I have realised the importance of a robust API and will consider creating one for future projects.

## Download Size

One of the biggest drawbacks of my engine is its large download size which makes it troublesome for users to play. This could have been solved by making the engine playable online with a browser, or to not use end game tables that are responsible for the large download size. I should have considered the implications on the download size of the engine before implementing end game tables.

## Dissertation

Before this project, I had never had to explain the motives behind the decisions made during software development. I have found that documenting my decisions helps me to think more critically about what would be best for the project. I have also learnt that images can be useful in assisting an explanation and I believe I should have used them.

## Independent Working

Working over lock-down due to COVID-19 and doing the EPQ while in year 11 rather than year 12 or 13 presented some challenges. It made communicating with my project supervisor more difficult, this meant that I had a unclear idea of what an EPQ consists of and was unable to plan as well as I should have been able to. I also missed many of the teaching sessions. However, I was still able to receive advice and guidance over phone calls with my project supervisor. Working independently over lock-down also meant I would have to motivate myself to work on the project and not to give up. Fortunately, I learnt to work independently and finished the project.


## Problem Solving

There were several problems that needed to be solved throughout the project: how should the computer pick the best move, how can the user input moves into the GUI, etc. Solving many of these problems was a difficult challenge and learning to overcome them improved my problem solving skills.

## Academic Writing

Initially my project essay was too informal. After some advice from my project supervisor, this was corrected. I also struggled to add good references to my essay that weren't simply links to wikis. This was improved by looking into the references that those wiki articles cited and referencing them directly. During the project, I have learnt how to write formally and how to clearly explain how a program works.

## Summary

If I were to do the project again I would make some changes. Firstly, I would do more research into what would be the most appropriate programing language to use. I am happy about my final decision of Rust but I wasted a lot of time making that decision because I started programing the engine in other, less suitable languages. I would also do more in depth research of current chess engines and the approaches they took to playing chess. By doing this I would have found many features and optimisations that could have improve the engine. When researching I should not have been afraid to read academic papers as nearing the end of the project I found many useful pieces of information in them. I should also have spent more time considering how each part of the program should work before implementing it in code. Once the first implementation had been made, I should have created better tests to ensure the algorithm is working properly. I should have also given the program an API so other programs can interact with it. After finishing my program, I found that its download size was around 2GB, I should have considered the implication of the end game tables on the size of the program. When creating my project essay, I could have included images to assist my explanation of the algorithms used to select the best move.



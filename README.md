# FreQ Sage

FreQ Sage is a easy to use but robust console application for frequency analysis of text, mostly for the purpose of language learning and, specifically, sentence mining.

# How does it work?

TBD: specific details about most used commands (once the API and the command line UI are settled on);

However, right now I can discuss the main ideas behind the application already.
The point is to have a tool for frequency analysis of texts in your chosen target languages. For now of course, the application is only going to work with European or at least left-aligned latin and Cyrillic based languages (Asian languages are excluded for now). However, within that scope, the app should work well.

# What is frequency analysis in this case?

By "frequency analysis" I mean:
- Analysing texts to create a database of knowledge on:
  1. Most frequent words in the language;
  2. Ranking of sentences on the "easiest to hardest" scale.

## What does it mean for a sentence to be "easy" or "difficult"?

In this specific definition, "easy-ness" of a sentence depends on two criteria:
- The number of words in the sentence (the fewer words, the easier the sentence);
- The easy-ness of all the words in the sentence, summed up.
- The easy-ness of a single word is defined by its ranking in the frequency database. If the sentence is the most frequently encountered, it's considered the easiest.

This simple premise allows me to use FreQ Sage in the future to assist me with sentence mining and adding cards to Anki.

# What features should the base version have:

The base 1.0.0 version should have these features implemented:
- A database to store frequency analysis results for individual words, as well as for sentences;
- Analysing a text (in a simple text file for now) to add frequencies (the number of times a word is encountered in a text) of words to the database;
- Based on this word frequency ranking, as well as the number of words in the text, calculate the easy-ness and ranking of all the sentences in the text;
- Store scores and rankings of both words and sentences;
- Command: show top N words or top N sentences;
  - Flag: for this command, there should be an option to start from ranking N rather than the very start.

# Project management

As of right now, I'm not planning to use formal structure and project managment strategies for this, such as Kanban boards, GitHub issues or any sort of git branch workflows. This is going to be just straight push-to-main type of situation. The notes, todos and other important info are going to just be done as TODO: items directly in the code comments. The reason for this choice is, I want this project to be fluid and flexible, and I want to be able to develop it quickly. Therefore any kind of rigid/formal project management structure is going to just waste my time unnecessarily.

# Long-term, short-term?

For now, this is a specific application with a specific set of features, and should be developed in just a few days. Once I do get the first full version (1.0.0) out, I will start thinking about whether this app can be potentially expanded to help me with my language learning in other ways as well.

# Licensing

This application is licensed under MIT. This means that, provided you give the necessary attribution/copyrights to my code, you can do whatever you want with it!

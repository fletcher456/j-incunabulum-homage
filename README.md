A live demo of this project is available:

http://replit.com/@fletcher456/HelloWorldServer

This project was a test to see what the current possibilities with Replit are for me. It fully works, although I've only run it inside Replit, for reasons that will be clear in a couple more paragraphs. I believe that it is in the spirit of the J incunabulum:

https://www.jsoftware.com/ioj/iojATW.htm

The majority of the work done on this project happened in one week.

A major way I interacted with the Replit agent was having it generate design documents and analysis documents, and providing feedback on them. It seems that making exhaustive or nearly exhaustive list of action items or steps helps the process. Also, it seems like markdown is the bread and butter of AI. I suppose it's all the GitHub readmes? 

It's incredibly cluttered, but I didn't develop a more structured, less ad-hoc workflow. There's at least one markdown document in a terrible place because it was easier in the moment to leave it there. This hampered progress.

One constraint I undertook as part of this project was to do the whole thing on my phone. This readme was created and edited in GitHub web ui. I would commit code from the Replit app on my phone, and then look at the source files and the markdown files on GitHub, which has a nicer UI _by far_ for looking at markdown, and marginally better than the Replit app for looking at code, especially in terms of busyness and scrolling to see long lines. Replit was better for editing. I made minimal manual edits. Because I let the agent commit from my user, the only way to tell which few commits in this project were me is to read the commit messages. The AI commit messages are awful slop. However, this phone-only constraint was possible. Absolutely wild. If at the beginning of the year you had told me that I could get any sort of app developed on my phone, I would not have believed you.

I feel like it would help to get the agent mind-mapping, showing the connections and dependencies of its process as a graph of dependencies, something like that. The visualizations are great for me as a means of condensing information. AI is really good at visualizations! Maybe org-mode is a good match for AI. Maybe GNU Info is? UML? It seems like it really can understand tree-structured data. My next experiment will focus on this.

There is a huge amount of technical debt in this project. From an architectural point of view, the agent only ever added layers of stuff, never removed, never refined, never understood the project holistically. Well, maybe that's a bit harsh. It half-assed a bunch of refactorings in a way where you might think that it cleaned up after itself... but it smells like the cleanup was not done. I think there's a lot of dead code.

I had a lot of fun doing this, and I hope you'll take in my tale with caution: the thing works, which is amazing, but the code is absolutely trash. I'm not happy with any of it. 🤷

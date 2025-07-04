# An homage to the J Incunabulum

## Optimized for phone screens

A live demo of this project is available:

https://fletcher456.github.io/j-incunabulum-homage/

This project was a test to see what the current possibilities with Replit are for me. I believe that it is in the spirit of the J incunabulum:

https://www.jsoftware.com/ioj/iojATW.htm

A major way I interacted with the Replit agent was having it generate design documents and analysis documents, and providing feedback on them. It seems that making exhaustive or nearly exhaustive list of action items or steps helps the process. Also, it seems like markdown is the bread and butter of AI. I suppose it's all the GitHub readmes? 

The project directory is incredibly cluttered because I didn't develop a more structured, less ad-hoc workflow. There's at least one markdown document in a terrible place because it was easier in the moment to leave it there. This hampered progress.

One constraint I undertook as part of this project was to do the whole thing on my phone. This readme was created and edited in GitHub web ui. I would commit code from the Replit app on my phone, and then look at the source files and the markdown files on GitHub, which has a nicer UI _by far_ for looking at markdown, and marginally better than the Replit app for looking at code, especially in terms of busyness and scrolling to see long lines. Replit was better for editing. I made minimal manual edits. All of the AI commits have extra metadata, some sort of project screenshot, and AI slop commit messages. However, even with the phone-only constraint, the project was possible to complete. Absolutely wild. If at the beginning of the year you had told me that I could get any sort of app developed on my phone, I would not have believed you.

I feel like it would help to get the agent mind-mapping, showing the connections and dependencies of its process as a graph of dependencies, something like that. The visualizations are great for me as a means of condensing information. AI is really good at visualizations! Maybe org-mode is a good match for AI. Maybe GNU Info is? UML? It seems like it really can understand tree-structured data. My next experiment will focus on this.

There is a huge amount of technical debt in this project. From an architectural point of view, the agent only ever added layers of stuff, never removed, never refined, never understood the project holistically. Well, maybe that's a bit harsh. It half-assed a bunch of refactorings in a way where you might think that it cleaned up after itself... but it smells like the cleanup was not done. I think there's a lot of dead code.

Analysis was far better than implementation. Asking for feasibility analyses seems to trigger extra truthfulness. It actually told me not to do certain things, which made me more confident when it claimed other things were feasible.

I had a lot of fun doing this, and I hope you'll take in my tale with caution: the thing works, which is amazing, but the code is in that terrifying superposition of incredibly cool and also ready to collapse if you look at it funny. I'm not happy with the process. 🤷

# Epilogue

When I started writing this readme I thought that Replit let people who weren't logged in users see the project. That's not the case, so I switched to GitHub Pages and immediately had to jump into wasm and a bunch of configuration stuff... or rather, the AI did.

It works now, after a bunch of stuff gone wrong. I had it rip out LALRPOP and serde to try and get wasm builds to complete on Replit itself, but that didn't happen, so the wasm builds only ever happened on GitHub. It might have been possible to leave in LALRPOP and still compile on GitHub, but I found out that Replit does not trust its agents to directly run git commands, so I would have had to use a keyboard and the command line to try to get LALRPOP building in wasm on GitHub. This was a definite L for on-phone development.

> I wouldn't trust AI to directly run git commands either. I barely trust myself to do that!

Also, after a particularly egregious unasked-for commit, I made the AI write a self-criticism. It's in there among the giant mass of self-soothing green checkmarks and affirmations, if you're looking for a chuckle.

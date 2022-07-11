# Glass Cockpit 
(_aka HumanInterfaceLab_)

A library of WebGL components, for building interfaces, apps, games, etc.

I had started doing this wrapped in a Yew app, but considered the Yew component to be dead-weight for use-cases not leveraging HTML.  Nevertheless, the hybrid HTML/WebGl app is still in play here, so will likely have some of that as well.


### Commentary
Complex applications reach full potential with active human cognitive input, requiring advanced high-bandwidth interface.  The human cognition is largely visual, so maximizing data-conveyance is largey a graphics problem.

The ubiquity of web-tooling and infrastructure sustains the relevance of distributed web-applications for many applications, while the jump to dedicated networks and hardware might leave the software implemtation largely unchanged, in general.

The paradigm explored here for tier-one web-human-interface: Components: (a) An actual web-server serving a single-page-app per route, whereas data-server is distinct from web-server and handles all distributed state/data flows. (b) Route/app: Many of these will be a simply WebGL on Wasm.  
Stuff that we want written in hypertext typesetting language will still be HTML.  The thinking is that web-apps, apps in general have to cover the space between literary medium (blogs, articles), and the proverbial glass-cockpit, which is not like a book / literary document at all.  A glass-cockpit needs to be a high-bandwidth cognitive interface between human and machine.  Symbolic text exist, but the overall structure is more geometric and generally abstractly graphical.  

We will build a library of interfaces, mostly implemented as games.  There will be a library of WebGl constructs and routines, usable as library of components for games/interfaces.

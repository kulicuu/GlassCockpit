# Glass Cockpit 
(_aka HumanInterfaceLab_)

A library of WebGL components, for building interfaces, apps, games, etc.

I had started doing this wrapped in a Yew app, but considered the Yew component to be dead-weight for use-cases not leveraging HTML.  When would HTML be dead weight? For most of my professional career [^1] I have held this opinionated paradigm of web-apps: the hypertext document legacy of the internet made it so that the tooling for apps in the social media and ecommerce space was in a type-setting language ill-suited to the implementation of a full expression of graphics structure capabilities.  Some of the web is still text, and HTML/CSS practitioners have created a myriad of impressive effects.  
ed
When do you need HTML? When you need typesetting you need a typesetting language.  In the future it could be XeLaTex to GL.  HTML is suitable for old style hypertext documents.  These are basically text documents, like a book.  But they have hyperlinks.  
Now the symbolic and logical structure of a glass cockpit is higher bandwidth functionally, in terms of control (input to the machine from the human) and instrumentation (output from the machine to the human).  There are text elements, but they layout is symbolic and structurally operative in terms of organization in space of elements.  For virtual and augmented-reality, the 3D workspace is something to be engineered, and WebGl, while very well suited to pure 2D GUI elements, is what is available on the browser. The ubiquity of the platform has protoping advantages.

### Commentary
Complex applications reach full potential with active human cognitive input, requiring advanced high-bandwidth interface.  The human cognition is largely visual, so maximizing data-conveyance is largey a graphics problem.

The ubiquity of web-tooling and infrastructure sustains the relevance of distributed web-applications for many applications, while the jump to dedicated networks and hardware might leave the software implemtation largely unchanged, in general.

The paradigm explored here for tier-one web-human-interface: Components: (a) An actual web-server serving a single-page-app per route, whereas data-server is distinct from web-server and handles all distributed state/data flows. (b) Route/app: Many of these will be a simply WebGL on Wasm.  
Stuff that we want written in hypertext typesetting language will still be HTML.  The thinking is that web-apps, apps in general have to cover the space between literary medium (blogs, articles), and the proverbial glass-cockpit, which is not like a book / literary document at all.  A glass-cockpit needs to be a high-bandwidth cognitive interface between human and machine.  Symbolic text exist, but the overall structure is more geometric and generally abstractly graphical.  

We will build a library of interfaces, mostly implemented as games.  There will be a library of WebGl constructs and routines, usable as library of components for games/interfaces.


[^1]: For a few years around 2015 I thought to do full apps in SVG, finding it to be a good graphics API for web-app development, also finding that it clogged up the main JS thread with too much linear algebraic operations. WebGl is a different story, though text requires some variety of approaches depending on the specific effect required.
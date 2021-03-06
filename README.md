## pngsneak
##### CLI tool for hiding messages in png files
##### encode, decode, remove and print to your heart's content 💖

<a href=https://picklenerd.github.io/pngme_book/chapter_1.html target="_blank" >Background</a>

`png` files are broken down into "chunks". A "chunk" is a series of grouped bytes.
```
length bytes   type bytes           data bytes                 crc bytes
[L,L,L,L,       T,T,T,T,    D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,     C,C,C,C]
```

We derive the data byte's length by converting the first four bytes (length bytes) to a 32 bit integer.
The type bytes are upper/lowercase ascii bytes that impart information about the chunk. See PNG file spec [section 3.3.](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html#PNG-file-signature) for more info about chunk type bytes.

By using type and data bytes we can create our own chunks and embed text in png files. This is something you will probably NEVER need to do 😝
___________________________________________________________________

Clone away if you want to engage in some pointless png manipulation

```
cargo run help
cargo run help <encode|decode|remove|print>
```

Or if you'd like to cut to the chase - you need 1 png file to ride this ride.

```
cargo run encode <path to png> rUST  "This is the hidden message" <optional output path>
cargo run decode <path to png> rUST
cargo run remove <path to png> rUST
cargo run print  <path to png>
```

This can be installed as a binary in your CWD (I doubt you want to do that 🤷) 
```
cargo install --path
pngsneak help
pngsneak help <encode|decode|remove|print>
```

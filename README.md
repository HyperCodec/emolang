# Emolang
A coding languages for emos that utilizes emojis

### Syntax
This language is composed of emojis to symbolize binary:
- 👌 = 1
- 💀 = 0
- 👀 = byte separator (bytes can have less than 8 bits, will fill missing ones with leading 0s when compiling)
- All forms of whitespace are ignored by the compiler. 
- Comments are not yet supported, though they are a planned feature.

### Compiling
This language compiles to binary files (.bin).
To compile, you simply run `emo path\to\emo\file.emo`.
The outputted binary file can then be used for whatever purposes you may have.

### Example
Simple python Hello World script (from [example.emo](example.emo)):
```
💀👌👌👌💀💀💀💀👀
💀👌👌👌💀💀👌💀👀
💀👌👌💀👌💀💀👌👀
💀👌👌💀👌👌👌💀👀
💀👌👌👌💀👌💀💀👀
💀💀👌💀👌💀💀💀👀
💀💀👌💀💀💀👌💀👀
💀👌👌💀👌💀💀💀👀
💀👌👌💀💀👌💀👌👀
💀👌👌💀👌👌💀💀👀
💀👌👌💀👌👌💀💀👀
💀👌👌💀👌👌👌👌👀
💀💀👌💀💀💀💀💀👀
💀👌👌👌💀👌👌👌👀
💀👌👌💀👌👌👌👌👀
💀👌👌👌💀💀👌💀👀
💀👌👌💀👌👌💀💀👀
💀👌👌💀💀👌💀💀👀
💀💀👌💀💀💀👌💀👀
💀💀👌💀👌💀💀👌
```

To run this, you would first compile by doing `emo example.emo` and then run it via the python interpreter with `python example.bin`

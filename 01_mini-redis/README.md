# Mini Redis

## Project Structure

There are some concepts:
1. Tcp bytes stream
2. Redis Frame
3. Redis Command
4. Store in memory

**connection** transfer tcp stream into Redis frame; **parse** help transfer Redis frame into Redis command

## Features

- [x] Support commands: `set`, `get` with ttl.
- [ ] Log
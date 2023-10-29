# Mini Redis

## Project Structure

There are some concepts:
1. Tcp bytes stream
2. Redis Frame
3. Redis Command
4. Store in memory

**connection** transfer tcp stream into Redis frame
**parse** transfer Redis frame into Redis command
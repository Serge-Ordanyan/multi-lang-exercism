# Exercism CLI Quick Guide

## Installation

```bash
brew install exercism
# Get token from: https://exercism.org/settings/api_cli
exercism configure --token=YOUR_TOKEN
First Time Setup
Join tracks on website:

bash
open https://exercism.org/tracks/python
open https://exercism.org/tracks/javascript
Download Exercises
bash
exercism download --track=python --exercise=two-fer
exercism download --track=python --exercise=leap
exercism download --track=javascript --exercise=hello-world
Solve & Test
bash
cd ~/Exercism/python/two-fer
cat README.md                    # Read instructions
code two_fer.py                  # Edit solution
exercism test                    # Run tests
exercism submit two_fer.py       # Submit solution
Example Solution
python
# two_fer.py
def two_fer(name="you"):
    return f"One for {name}, one for me."
Useful Commands
bash
exercism workspace               # Show workspace location
exercism download --force        # Overwrite existing exercise
ls ~/Exercism/python/            # List downloaded exercises
Troubleshooting
"Not joined track" → Join on website

"Directory exists" → Use --force flag

Tests fail → Check README.md instructions

Quick Reference
bash
# Complete workflow:
exercism download --track=python --exercise=NAME
cd ~/Exercism/python/NAME
code FILE.py
exercism test
exercism submit FILE.py





# after all here is submit and test commands exercises V
exercism submit two_fer.py
```

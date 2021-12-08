also i learned last year that you can script downloading your daily challenge input… here’s how i do it:
open a challenge on adventofcode.com, then open the network tab, and refresh. now look at the request header’s cookie value and look for session=<SESSION VALUE>
take that <SESSION VALUE> and put it in ~/.aocrc
add this bash function to your bash_profile (or whatever terminal you’re using):
  aocdl () {
    curl -b session=$(cat ${HOME}/.aocrc) https://adventofcode.com/2021/day/$1/input 2>/dev/null > input.txt
  }
4. call with with aocdl 5 to download day 5's input to the current directory

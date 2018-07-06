# git-browse

All it does is open the current repositories remote origin in a browser,
with some light transformations to accomodate for urls:

Start with: `"git@github.com:felipesere/git-browse.git"`

and then replace
  * ":" with "/"
  * "git@" with "https://"
  * ".git" with ""

and arrive at  "https://github.com/felipesere/git-browse"!


It will let you know if you are not in a git repo or there is no origin set.

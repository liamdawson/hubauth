# TODOs

## CLI Test Coverage

* [x] General invocation
  * [x] Subcommands shown if no subcommand
  * [x] Help works
  * [x] Version works
* [ ] Init
  * [ ] Username validation
  * [ ] Handles a file which is already configured
  * [ ] Handles a file which has comments to replace
  * [ ] Handles a file with no comments to replace
* [ ] Cached
  * [ ] Handles golden path
  * [ ] Validates username
  * [ ] Ignores old entries
* [ ] Fetch
  * [ ] Handles golden path
  * [ ] Validates username
  * [ ] Makes request as expected
  * [ ] :question: Possible to test the GitHub key source without MITMing it?
* [ ] List
  * [ ] Respects `min_age`
  * [ ] Respects `max_age`
  * [ ] Calls to remote server if over `min_age`
  * [ ] Falls back to cache if the connection fails transiently
  * [ ] Wipes cache if the connection fails with a 410
    * [ ] :question: Any point testing all of the codes?
* [ ] Sync
  * [ ] Fetches all
  * [ ] Doesn't fetch for `cache: false` user
  * [ ] Fetches key sources shared across cache/non-cache users


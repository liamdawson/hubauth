% hubauth.yml(5) hubauth config file format
% Liam Dawson
% April 5, 2019

# NAME

hubauth.yml - hubauth configuration file

# SYNOPSIS

`/etc/hubauth.yml`

# DESCRIPTION

The hubauth configuration file is a YAML-formatted file which defines
critical values, and maps users to key source URLs.

The possible keys and their meanings are as follows (note that all
values are case sensitive):

## caching

* destination
  Specifies the directory where cached keys are to be stored. Defaults
  to /var/cache/hubauth
* min_age
  Specifies the minimum age of a cached source before hubauth-list(1)
  will attempt to fetch it
* max_age
  Specifies the maximum allowed age of a cached key, after which
  hubauth-list(1) and hubauth-cached(1) will ignore it

## users

* `<username>`
  Specifies a username to configure
* `<username>`.cache
  Specifies whether the named user permits key caching--if false, keys
  are not written for this user via hubauth-sync(1) or hubauth-list(1)
  and cached keys are not read via hubauth-cached(1) or
  hubauth-list(1).

  Note that the key sources for this user may still be cached if used
  by another user.
* `<username>`.key_sources
  An array of key sources for the specified user. The following
  formats are recognised:
  
  * `- url: <url>`
    Read keys from the given URL, without transformation
  * `- github: <gh_handle>`
    Read keys for the given GitHub user `<gh_handle>` (that is, the
    url `https://github.com/<gh_handle>.keys`)

# SEE ALSO

hubauth(1) hubauth-list(1) hubauth-cached(1)

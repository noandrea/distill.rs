# Distill

Another url shortener

[![Build Status](https://travis-ci.com/noandrea/distill.svg?branch=master)](https://travis-ci.com/noandrea/distill) [![codecov](https://codecov.io/gh/noandrea/distill/branch/master/graph/badge.svg)](https://codecov.io/gh/noandrea/distill) [![GoDoc](https://godoc.org/github.com/noandrea/distill?status.svg)](https://godoc.org/github.com/noandrea/distill) [![Go Report Card](https://goreportcard.com/badge/github.com/noandrea/distill)](https://goreportcard.com/report/github.com/noandrea/distill)

[![Docker](https://img.shields.io/badge/docker-noandrea/distill-blue)]

## Motivations

Existing url shorteners are not suitable for a private deploy use or are too complex in terms of requirements.

_Distill_ aims to create a easy deployable short url service
that can be used for specific events.

## Features

- Choose the alphabet set for the generate short id
- Choose the length of the generate short id
- Load existing short id <-> url mappings\*
- Overwrite an existing short id with a different target url\*
- Set a time to live on short ids (globally or per id)
- Set a expiration date on short ids (globabbly or per id)
- Set a request limit on short ids (globally or per id)
- Set a redirect for the `/` path
- Set a redirect url for exhausted ids (request limit reached)
- Set a redirect url for expired ids (ttl/end date reached)
- Backup/restore urls in csv or binary format
- Import data via csv
- Get statistics both globally and for short id

\* the alphabet and lenght can be enforced

## Expiration strategy

There are 3 ways to set an expiration for a short id:

- TTL (seconds)
- Epiration date
- Max requests

The three options can be configured globally or per short id,
the value specified for the short id takes always precedence over the
global configuration.

For the _TTL_ and the _expiration date_ the actual expiration is selected as
`max ( creation_date + ttl, expiration_date)`

> !!! the expiration is set upon short id creation, changing global configuration
> will not affect the short ids already set !!!

For redirects, the expiration url redirect takes precedence over the exhaustion url redirect.

If no redirects are set for exhausted / expired url then a `404` is returned.

## Storage

The supported storage if for kv stores 

### Key schema

Keys should be grouped by domain, at startup the default domain 
is `default`. 

There are 2 type of objects for a domain, config and shortcodes:

There is always one **config** config key for a domain and the key is:

```
<HASH(domain):config>
```

**shortcodes** have cardinality `0...N` hand the keys are:

```
<HASH(domain):HASH(shortcode)>
```

## CLI Doc

TODO

## Api Doc

TODO


A [`docker-compose`](https://docs.docker.com/compose/) example is available in the [`examples/docker`](https://github.com/noandrea/distill/blob/master/examples/docker)

### Systemd 

Distill can be run via `systemd`, check the [example](https://github.com/noandrea/distill/blob/master/examples/systemd) configuration.






# udp53_lookup
Test the interception/filter of UDP 53 of your local networks or hotspots.

Inspired by [BennyThink/UDP53-Filter-Type](https://github.com/BennyThink/UDP53-Filter-Type) .

## What's the purpose?
If your hotspots or local networks do not filter the traffic via UDP 53, you may want to setup an OpenVPN or iodine dns tunnel to bypass web auth.

## Quick start
Download from Releases, start it with web auth logged out.

## Test passed, what should I do next?
OpenVPN, Shadowsocks (with KCPTUN), V2Ray (with mKCP) and anything utilizes UDP protocol should work.

In some rare networks, malformed DNS queries (e.g OpenVPN on UDP 53) would be blocked but traffic with correct DNS query structures would be allowed. In this situation, [iodine dns tunnel](https://github.com/yarrick/iodine) may help.

## How does it work?
udp53_lookup uses a special DNS server set up by me for the query. For some specific domain names, this DNS server will respond with special answers which should not to be found in other DNS services. udp53_lookup recognizes these answers and if the actual answers are not identical to the expected ones, your local networks are hijacking every DNS query, indicating it's impossible to use malformed DNS packets to transport.

For the exact DNS server currently using, see the source code for details.

## License
[MIT License](https://opensource.org/licenses/MIT)

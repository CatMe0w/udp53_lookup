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

## License
[MIT License](https://opensource.org/licenses/MIT)

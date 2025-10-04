# Keisteen
*Keisteen is an experimental Minecraft Server implementation written in Rust.*

![Server List](README/serverlist.png)

**Why?**
a) Why not?
b) For fun!

There are lots of Minecraft Server implementations written in Rust to be found, and I do not intend to challenge them. This project is mostly a side-project that started out of curiosity, so do not expect it to work, be fast or, God forbid, be stable.

## Checklist
- [ ] Handshaking
	- [x] Handle intended connection state
	- [ ] Kick player if protocol versions do not match
	- [ ] Handle legacy server list ping packet
- [ ] Status
	- [ ] Status response
		- [x] Version
		- [ ] Players
		- [ ] Description
			- [x] Send hardcoded description
			- [ ] Send customizable description
		- [ ] Favicon
		- [ ] Enforce secure chat
	- [x] Pong response
- [ ] Login
	- [ ] Handle disconnected packet
	- [ ] Encryption
		- [x] Encrypt packets
		- [ ] Authenticate
	- [ ] Compression
	- [ ] Login plugin request packet
	- [ ] Cookie packets
	- [ ] Send player properties
- [ ] Transfer
- [ ] Configuration
	- [ ] Use client information to store their settings
	- [ ] Cookie packets
	- [ ] Plugin messages
		- [ ] Store client brand
		- [x] Send server brand
		- [ ] Expose received plugin messages to API
	- [ ] Keep-alive packets
	- [ ] Ping-pong packets
	- [ ] Resource packs
	- [ ] Synchronize known packs
	- [ ] Handle custom click action
	- [ ] Reset chat packet
	- [ ] Registry packets
		- [x] Basic registry support
		- [ ] Send all registries to client
	- [ ] Send feature flags
	- [ ] Update tags
	- [ ] Dialogs
	- [ ] Server links
- [ ] Play
	- [ ] Allow player to log in

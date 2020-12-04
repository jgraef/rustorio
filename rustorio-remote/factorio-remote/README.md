
# Simulation Theory

What if *The Engineer* would research advanced technology that not only allows him to prove that he lives in a
simulation, it also allows him to hack into the simulation and uses it's computational resources to get more iron.

# Factorio Remote

This crate allows to send and receive circuit signals to and from a Factorio game.

The game must run the [Remote IO][https://github.com/jgraef/factorio-remote-mod] mod to accept connections.

## Features

For examples on how to use these features take a look at the `examples/` directory.

### Status Combinator

This combinator outputs a signal that indicates if a connection from the client was established. A connection is
considered established if the client sent a request in the last 10 seconds. There is currently no way to ensure that
the client sends at least one request every 10 seconds (see TODO).

The combinator will output `1`  on the *Connection Status* virtual signal, if there is a connection and `-1` otherwise. 

### Signal Sender

The signal sender is an entity that accepts a *channel* virtual signal and will send all other circuit signals to the
client when requested with `FactorioRemote::receive_signals(channel)`.

### Signal Receiver

The signal receiver is an entity that accepts a *channel* virtual signal and will receive other circuit signals from
the client when requested with `FactorioRemote::send_signals(channel, signals)`. 

### Blueprint Importer

The blueprint importer is a chest that can import a blueprint that is send by the client with
`FactorioRemote::import_blueprint(channel, blueprint_data)`. It also accepts a channel virtual signal.

## Example

The following example code connects to the mod and prints `Hello World!` in the players' consoles.

```rust
// Add `tokio = { version = "0.2", features = ["macros"] }` to your dependencies

use factorio_remote::{
    error::Error,
    FactorioRemote,
};


#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // Connect using environment variables `RCON_ADDRESS` and `RCON_PASSWORD`
    // Alternatively use `RemoteIO::connect(hostname, password)`.
    let mut remote = FactorioRemote::connect_env().await?;

    // Print `Hello World` to the players' consoles.
    remote.print("Hello World!", None).await?;

    Ok(())
}
```

You need to set the `RCON_ADDRESS` and `RCON_PASSWORD` variables accordingly. You can also write them into a `.env`
file and the example will load them:

```dotenv
# .env
RCON_ADDRESS=localhost:27015
RCON_PASSWORD=hunter2
```

Please choose a more secure password.

To configure your Factorio to run a RCON server, you need to set the following settings in
`~/.factorio/config/config.ini` (TODO: What is the Windows path?).

```ini
; Socket to host RCON on when lauching MP server from the menu.
local-rcon-socket=127.0.0.1:27015

; Password for RCON when launching MP server from the menu.
local-rcon-password=hunter2
```

## Concept

The [mod](https://github.com/jgraef/factorio-remote-mod) adds a console command to accept requests from this client.
The requests are sent via [RCON](https://github.com/panicbit/rust-rcon) and are JSON strings. The mod then answers by
printing to the RCON client with [`LuaRCON.print()`](https://lua-api.factorio.com/latest/LuaRCON.html#LuaRCON.print).

### Request format

```json
{
  "request_type": "<REQUEST_TYPE>",
  // ... Additional request parameters
}
```

The command executed then is:

```
/__remote-io-request {"request_type":"<REQUEST_TYPE>", ... }
```

For a list of supported request types, take a look at `src/request.rs`.
 
### Response format

The mod can print anything to the RCON client. This is used to e.g. relay debug information to the RCON client. The
last line printed for a request is the response line and always starts with a fixed prefix `__remote-io-response`

The JSON response for a successful request:

```json
{
  "status": "success",
  "value": "<RETURN_VALUE>"
}
```

The JSON response for a failed request:

```json
{
  "status": "error",
  "error": "<ERROR_MESSAGE>"
}
```

Note that the `error` field can contain anything that is a Lua error serialized to JSON, so it is not always a string.

## Mod interoperability

The mod exposes a [`LuaRemote`](https://lua-api.factorio.com/latest/LuaRemote.html) interface, that can be used by
other mods to register request handlers:

 - `send_request(request)`: Runs the given request. The request is passed in decoded form as table.
 - `register_request_handler(request_type, handler)`: Registers a handler for the given request type. Please make sure
   to avoid name collisions, i.e. by prefixing the name with your mod name.
 - `import_blueprint(channel, blueprint_data)`: Imports the given blueprint to blueprint importers on that channel.
 - `is_connected()`: Returns `true` if a client is connected, `false` otherwise.

# TODO

If you have ideas for other features, open an issue. I'd be happy to add more features (that are not alike to cheating).

 - [_] Send notification (like a speaker would, but with customizable text)
 - [_] Graphics (Help wanted)
 - [_] Set size of receiver slots to number of available signals in `data-final-fixes.lua`
 - [_] Better recipes
 - [_] Mod settingsim
 - [_] Accept other types than string for error values.
 - [_] Heartbeat thread
 - [_] Arbitrary Lua code execution, behind a settings value (off by default).
 - [_] Set train schedules.
 - [_] Split the remote instrumentation into a seperate mod. This can be used to implement other mods as well.
 - [_] Document message passing. Implement push semantics from mod to client by writing to file.
 - [_] Use mod to client calls for a debugger.

# Bugs

This is my first Factorio mod, so it likely contains some bugs. Please open an issue, if you find one.

If you have other suggestions on how to improve the mod, please also don't hesitate to open an issue.

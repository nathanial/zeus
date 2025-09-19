# Zeus Web Server Walkthrough

This scenario imagines how a developer could stand up a web server in
Zeus using the live runtime and functional core. It leans on the ideas in
`docs/` and demonstrates the kind of workflow the language aspires to
support.

## Goal

Expose a simple HTTP service that:

- responds to `GET /` with a welcome message retrieved from a live
  configuration record,
- records each request in an append-only event log, and
- streams runtime metrics to an inspector workspace for experimentation.

## 1. Model the Domain

Start in the Zeus workspace by creating a new module:

```zeus
module Web.Server where

import Prelude
import Zeus.Network.Http
import Zeus.Time
import Zeus.Runtime.Log

route root : Request -> Response
route root request =
    let greeting = Config.current.greeting
        timestamp = Time.now()
    in Response.ok "text/plain" (greeting ++ " @ " ++ show timestamp)
```

Notes:

- `Config.current` surfaces the live configuration object; edits to it in
  the inspector take effect immediately.
- `Time.now` is an explicit effect node; the type checker requires the
  module to declare the `Temporal` capability.

## 2. Wire Effects Explicitly

Declare which effects the module consumes so the runtime can schedule it
safely:

```zeus
effect Grants = { Temporal, Http, Log }
```

The runtime editor offers a capability palette that can be toggled on and
off while the module is running. Removing `Http` would freeze the server
until it is restored.

## 3. Register the Service

Open the `Runtime > World Graph` inspector and attach the module to the
HTTP dispatcher:

```zeus
Http.attach
    { path = "/"
    , method = Get
    , handler = Web.Server.route root
    }
```

Behind the scenes the dispatcher updates the world graph with a new
listener node. Because the runtime journals every mutation, you can roll
back the change if the handler misbehaves.

## 4. Observe in Real Time

With the service running, spin up a live metrics panel:

```zeus
Metrics.stream "web-server" |> Inspect.liveChart
```

Every request increments counters and logs structured events:

```zeus
Log.append
    (Event.new "web.request"
        { path = request.path
        , method = request.method
        , latency = timer.elapsed()
        })
```

The inspector lets you scrub through past events, fork the timeline, and
replay them against new code.

## 5. Hot Reload Handlers

Editing `route root` in the workspace instantly updates the live image.
If the change fails the type checker or raises a runtime guard, Zeus
rolls back to the previous version while keeping the error report pinned
in the inspector. You can then patch the code and retry without dropping
in-flight connections.

## 6. Snapshot and Share

Once satisfied, capture a snapshot so teammates can resume the same
state:

```zeus
Runtime.snapshot.save "web-server-alpha01"
```

Snapshots store both the code and the running world, including request
logs and configuration deltas. Loading the snapshot recreates the server
exactly as it was, ready for further exploration.

---

This example is aspirational—the current Rust prototype only sets the
stage. Use it as a design target while fleshing out runtime APIs, effect
handling, and tooling.

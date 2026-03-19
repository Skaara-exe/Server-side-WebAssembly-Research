# WebAssembly persistence demo
## Introduction
As you might know, a typical usecase of WebAssembly is its heavily sandboxed environment. Saving needs dedicated steps and is not straightforward.  
In this demo, you will see how you can set up a persistent wasm app yourself, using [Wasmcloud](https://wasmcloud.com/).

## Description
### Notes
- Setting up this project and installing the necessities takes **1 hour to 90 minutes**.
- This was tested on a Google Cloud VM, running Debian 12, with port 8000 open.


### Installing wasmcloud

Install wash with the following script. Make sure to move `wash` to somewhere in your `$PATH` (e.g. /usr/local/bin).
Option 1 is wash v0.39.0, while the second option is v2.0.0 (at 2026-03-10). v2.0.0 does not support a local deployment without kubernetes, while v0.39.0 does.  
**I recommend to use v0.39.0!**

**Option 1 - v0.39.0**  
Note: To use cargo, go to [Choosing your language](#choosing-your-language) first.  
Installing this took me about 45 minutes.
``` bash
cargo install --locked wash-cli
``` 
**Option 2 - v2.0.0**  
``` bash
curl -fsSL https://raw.githubusercontent.com/wasmcloud/wasmCloud/refs/heads/main/install.sh | bash
```
------------
Move `wash` to your `$PATH`.  
**Note**: If you installed v0.39.0, this is located in `$HOME/.cargo/bin/wash`
``` bash 
sudo mv wash /usr/local/bin
```
Verify that it's running with 
``` bash
wash --version
```

### Choosing your language
wasmCloud supports building WebAssembly components from any language that supports the WASI 0.2 target, including Go, Rust, TypeScript, and others.  
To install **Rust** on Ubuntu/Debian, use the following script.
``` bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Once Rust is installed, **restart the terminal** and install the wasm32-wasip2 target:
``` bash
rustup target add wasm32-wasip2
```
Install **git** to use the wasmcloud templates and install the cc linker.
``` bash
sudo apt-get install git-all -y
sudo apt-get install build-essential -y
```
If you want to install option 1, go [back](#installing-wasmcloud) to install it.

### Demo
#### No configuration
If you are on v0.39.0 and want the full project without configuring the persistence layer yourself, clone the http-hello-world folder in this project.  

Start up the NATS lattice with the following command
```bash
wash up -d
```

To deploy the app, run the deploy command
```bash
wash app deploy manifests/workloaddeployment.yaml
```

Visit the site at `localhost:8000`

#### v0.39.0
Clone the project and extract the `http-hello-world` demo from the [wasmcloud git repository](https://github.com/wasmCloud/wasmCloud/tree/main/examples/http-hello-world).  
```bash 
git clone https://github.com/wasmCloud/wasmCloud
mv wasmCloud/examples/http-hello-world/ ./
sudo rm -r wasmCloud
cd http-hello-world
```

Start the NATS server with the following command.
``` bash
wash up -d
```

In `Cargo.toml`, Change the `edition` to `2021` instead of `2024`:
```toml
[package]
name = "hello-world"
edition = "2021"
version = "0.1.0"
```
Create `wasmcloud.toml`
```bash
echo 'name = "hello-world"
language = "rust"
type = "component"

[component]
wit_world = "hello"
wasm_target = "wasm32-wasip2"' | tee wasmcloud.toml
```

Build the app with `wash build`. This should return a .wasm app, which should be used in the `manifests/workloaddeployment.yaml` file.  
Then, replace the content of the workloaddeployment file with the following:
```yaml 
apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  annotations:
    version: v0.0.1
    description: "Hello world"
  name: hello-world
spec:
  components:
    - name: hello-world
      type: component
      properties:
        image: file:///home/user/http-hello-world/build/hello_world_s.wasm
      traits:
        - type: spreadscaler
          properties:
            instances: 1
            
    - name: httpserver
      type: capability
      properties:
        image: ghcr.io/wasmcloud/http-server:0.23.2
      traits:
        - type: spreadscaler
          properties:
            instances: 1
        - type: link
          properties:
            target: hello-world
            namespace: wasi
            package: http
            interfaces: [incoming-handler]
            source_config:
              - name: default-http
                properties:
                  address: 0.0.0.0:8000
...
```
Deploy the program with the following command:
```bash
wash app deploy manifests/workloaddeployment.yaml 
```

#### v2.0.0
To make a new component, use `wash new`
``` bash
wash new https://github.com/wasmCloud/wasmCloud.git --subfolder examples/http-hello-world
```
Build the component with `wash -C (folder) build`
``` bash
wash -C ./http-hello-world build
```
Start a development loop with the `wash -C (folder) dev`
``` bash
wash -C ./http-hello-world dev
```

-----------------

The app should now be running, and return `Hello from wasmCloud!` when accessed. (Running at localhost:8000)

### Adding the persistence layer
In the `wit/world.wit` file, import `wasi:keyvalue/store@0.2.0-draft` and `wasi:keyvalue/atomics@0.2.0-draft`.
``` wit
package wasmcloud:hello;

world hello {
   export wasi:http/incoming-handler@0.2.2;
   import wasi:keyvalue/atomics@0.2.0-draft;
   import wasi:keyvalue/store@0.2.0-draft;
}
```
This grants the app access to the wasi:keyvalue capability. The store interface provides access to a bucket resource, while the atomics interface is useful for counters.

Adjust the host file (`src/lib.rs`) by importing the keyvalue library and modifying the `home` function.
``` rust
use wasmcloud_component::wasi::keyvalue::*;

//...

async fn home(_req: Request<Body>) -> Result<Response<Body>, wstd::http::Error> {
    // Return a simple response with a string body
    let bucket = store::open("default").unwrap();
    let count = atomics::increment(&bucket, "counter", 1).unwrap();
    Ok(Response::new(format!("Hello from wasmCloud! I was called {count} times\n").into()))
}
```

And rebuild the app with the build command
```bash 
wash build
```

#### v0.39.0
Adjust the `manifest/workloaddeployment.yaml` by adding the kvnats component and linking it to the wasm component.
```yaml
apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  annotations:
    version: v0.0.1
    description: "Hello world with persistent key-value counter"
  name: hello-world
spec:
  components:
    - name: hello-world
      type: component
      properties:
        image: file:///home/user/http-hello-world/build/hello_world_s.wasm
      traits:
        - type: spreadscaler
          properties:
            instances: 1
        - type: link
          properties:
            namespace: wasi
            package: keyvalue
            interfaces: [store, atomics]
            target:
              name: kvnats
              config:
                - name: wasi-keyvalue-config
                  properties:
                    bucket: default
                    enable_bucket_auto_create: "true"

    - name: kvnats
      type: capability
      properties:
        image: ghcr.io/wasmcloud/keyvalue-nats:0.3.0
      traits:
        - type: spreadscaler
          properties:
            instances: 1

    - name: httpserver
      type: capability
      properties:
        image: ghcr.io/wasmcloud/http-server:0.23.2
      traits:
        - type: spreadscaler
          properties:
            instances: 1
        - type: link
          properties:
            target: hello-world
            namespace: wasi
            package: http
            interfaces: [incoming-handler]
            source_config:
              - name: default-http
                properties:
                  address: 0.0.0.0:8000

```
Add `wasmcloud-component` as a dependency in the `Cargo.toml` file.
```toml
[package]
name = "hello-world"
edition = "2021"
version = "0.1.0"

[workspace]

[lib]
crate-type = ["cdylib"]

[dependencies]
wstd = "0.6.3"
wasmcloud-component = "0.2.0"
```
Delete the previous deployment with the following commands
```bash
wash app undeploy hello-world 
wash app delete hello-world
```
And redeploy the app 
```bash
wash app deploy manifests/workloaddeployment.yaml
```


#### v2.0.0
Modify the `manifests/workloaddeployement.yaml` file by linking the wasi:keyvalue interfaces to the kvnats capability.
```yaml
apiVersion: runtime.wasmcloud.dev/v1alpha1
kind: WorkloadDeployment
metadata:
  name: hello-world
spec:
  replicas: 1
  template:
    spec:
      hostSelector:
        hostgroup: default
      components:
        - name: hello-world
          image: ghcr.io/wasmcloud/components/hello-world:0.1.0
          traits:
            - type: link
              properties:
                namespace: wasi
                package: keyvalue
                interfaces: [store, atomics]
                target:
                  name: kvnats
                  config:
                    - name: wasi-keyvalue-config
                      properties:
                        bucket: default
                        enable_bucket_auto_create: 'true'
        - name: kvnats
          type: capability
          properties:
            image: ghcr.io/wasmcloud/keyvalue-nats:6c67eb2
      hostInterfaces:
        - namespace: wasi
          package: http
          interfaces:
            - incoming-handler
          config:
            host: localhost
```


Add `wasmcloud-component` as a dependency in the Cargo.toml file.
``` toml
[package]
name = "hello-world"
edition = "2024"
version = "0.1.0"

[workspace]

[lib]
crate-type = ["cdylib"]

[dependencies]
wstd = "0.6.3"
wasmcloud-component = "0.2.0"
```
Rebuild the application with `wash -C (folder) build`, and start with `wash -C (folder) dev`:
``` bash
wash -C ./http-hello-world build
wash -C ./http-hello-world dev
```
-----
## Conclusion
You'll see that when you visit `localhost:8000`, the counter increments. For v0.39.0, the data will persist, even if you redeploy the manifest or shut the NATS lattice down with `wash down`.    
In v2.0.0, the data only exists in-memory. Should you want to persist this, you'll need to deploy it to kubernetes and attach it to a NATS-lattice.  

As can be seen, wasm apps can easily configured so their data is persisted across sessions.

## Documentation
- https://wasmcloud.com/docs/tour/hello-world/?os=debian
- https://github.com/wasmCloud/wasmCloud/tree/main
- https://crates.io/crates/wasmcloud-component/0.2.0
- https://github.com/WebAssembly/wasi-keyvalue/blob/main/watch-service.md
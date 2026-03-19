# containers
## Introduction
This project is made specifically to test wasm (WebAssembly) containers. Docker Desktop support is deprecated, so it's focused on kubernetes.  

**Findings:** 
- Wasm is able to run on a Linux-OS based container. The OS needs to have glibc (included in debian:bookworm-slim, but not in alpine).
- To run it on alpine (or scratch), wasmtime must be compiled statically.
  To compile wasmtime yourself with static binaries, execute the following commands **in linux**.  
  **Note:** you need to install rust **and** musl-tools.
```
rustup target add x86_64-unknown-linux-musl
cargo build --release --target=x86_64-unknown-linux-musl
```
- Wasmtime is not able to run wasm binaries in kubernetes. ~~Might have to look into alternatives; with [k3d](https://github.com/deislabs/containerd-wasm-shims/blob/main/deployments/k3d/README.md#how-to-run-the-example) or [krustlet](https://docs.krustlet.dev/howto/wasm/).~~
- WasmEdge is not able to run wasm binaries either on kubernetes. 
- Alternatives for running wasm binaries on kubernetes such as k3d and krustlet have their own issues. K3d depends on docker, so support is not a given. Meanwhile, Krustlet hasn't been updated in four years.
- Wasmcloud and SpinKube might still prove useful.


## Requirements
- Kubernetes
  - [Kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/)
  - [Kubeadm](https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/install-kubeadm/) - make sure you have at least 2GB RAM!
  - A container runtime. I installed [containerd](https://github.com/containerd/containerd/blob/main/docs/getting-started.md) to make sure I'm not dependent on Docker support.
  - [A running cluster](https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm/)
  - Install a CNI plugin. I chose [flannel](https://github.com/flannel-io/flannel)
  - Load kernel module (not always necessary)
- [Wasmtime containerd shim](https://github.com/containerd/runwasi/releases/tag/containerd-shim-wasmtime%2Fv0.6.0)

## Tree view
```
/usr/local/bin/
├── containerd
├── containerd-shim-runc-v2
├── containerd-shim-wasmtime-v1
├── containerd-stress
├── ctr
└── kubectl

/etc/containerd/
└── config.toml

$HOME/.kube
├── cache
└── config
```


## Extra configuration
**NOTE:** Make sure that whenever you change the configuration file, you restart the containerd service.
- Create the configuration file at /etc/containerd/config.toml.   
``` bash
  sudo mkdir -p /etc/containerd
  sudo containerd config default | sudo tee /etc/containerd/config.toml
```
- Enable systemd cgroup driver in the config file.
``` toml
[plugins."io.containerd.grpc.v1.cri".containerd.runtimes.runc.options]
  SystemdCgroup = true
```

- I added wasmtime as a runtime in containerd, in the same config file as before. 
``` toml
[plugins]
 ...
  [plugins.'io.containerd.cri.v1.runtime']
    ...
    [plugins.'io.containerd.cri.v1.runtime'.containerd]
    ...
        [plugins.'io.containerd.cri.v1.runtime'.containerd.runtimes]: 
          [plugins.'io.containerd.cri.v1.runtime'.containerd.runtimes.wasmtime]
            runtime_type = "io.containerd.wasmtime.v1"
```
- Set ipv4 forward content to 1
```
echo "1" | sudo tee /proc/sys/net/ipv4/ip_forward
``` 

If you want a runtime other than wasmtime, replace "wasmtime" with your preferred runtime.  
Example:  
Install wasmedge with `curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- -p /usr/local` and put the underlying code snippet in /etc/containerd/config.toml.
``` toml
          [plugins.'io.containerd.cri.v1.runtime'.containerd.runtimes.wasmedge]
            runtime_type = "io.containerd.wasmedge.v1"
```
- Add a .yaml file to add wasmtime to kubernetes as a runtimeClass. (Or an alternative!)
``` yaml
apiVersion: node.k8s.io/v1
kind: RuntimeClass
metadata:
  name: wasmtime
handler: wasmtime
```
To execute the file, type `kubectl apply -f runtime.yaml`.

- Initialize the cluster with a pod network cidr
``` bash
kubeadm init --pod-network-cidr=10.244.0.0/16
```

- Initialize kubectl for the user
```
mkdir -p $HOME/.kube
sudo cp -i /etc/kubernetes/admin.conf $HOME/.kube/config
sudo chown $(id -u):$(id -g) $HOME/.kube/config
```



- ? - Load the kernel module necessary [(since kubeadm v1.30)](https://github.com/flannel-io/flannel/issues/2068)
``` bash
sudo modprobe br_netfilter
```
To check if it loaded, execute `lsmod | grep br_netfilter`. You should see 
```
br_netfilter            24576  0
bridge                 155648  1 br_netfilter
```
Also enable bridge-nf-call-iptables
``` bash
sudo sysctl net.bridge.bridge-nf-call-iptables=1
sudo sysctl net.bridge.bridge-nf-call-ip6tables=1
``` 
and add it to /etc/sysctl.conf for persistence across reboots
```
net.bridge.bridge-nf-call-iptables = 1
net.bridge.bridge-nf-call-ip6tables = 1
```
reload with `sudo sysctl -p`  

And finally, restart the flannel pods with `kubectl delete pod -n kube-flannel -l app=flannel`
- Remove the constraint that pods can't run on the main node
```
kubectl taint nodes --all node-role.kubernetes.io/control-plane-
```

- Add a .yaml file to run wasm in a pod
``` yaml
apiVersion: v1
kind: Pod
metadata:
  name: wasm-app
spec:
  runtimeClassName: wasmtime
  containers:
  - name: app
    image: stroempell/my-wasm-app:latest
```

# Sources
- https://runwasi.dev/getting-started/quickstart.html
- https://kubernetes.io/docs/setup/
  https://seifrajhi.github.io/blog/k8s-wasm-runtimes-part1/#:~:text=Wasm%20is%20a%20portable%2C%20lightweight,deploying%20and%20managing%20containerized%20applications.
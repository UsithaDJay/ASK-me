Installed tools and toolchains

sudo apt install rustc

rustup toolchain install nightly --allow-downgrade
rustup default nightly

rustup target add wasm32-unknown-unknown

cargo install trunk cargo-leptos

failed to run custom build command for `openssl-sys v0.9.91`
Package openssl was not found in the pkg-config search path.
  Perhaps you should add the directory containing `openssl.pc'
  to the PKG_CONFIG_PATH environment variable
  No package 'openssl' found
---->(sudo apt-get install libssl-dev)

The pkg-config command could not be found.
  Most likely, you need to install a pkg-config package for your OS.
  Try `apt install pkg-config`, or `yum install pkg-config`,
  or `pkg install pkg-config`, or `apk add pkgconfig` depending on your distribution.
---->(sudo apt install pkg-config)

re-run
cargo install trunk cargo-leptos

cargo install cargo-generate

Got the template from a git hub repository
cargo leptos new --git https://github.com/leptos-rs/start
give a project name
cd {projectname}
cargo leptos watch

SerDe
cd {projectname} (inside the project directory)
cargo add serde -F derive

TailwindCSS
sudo apt install npm
npm install -D tailwindcss
npx tailwindcss -i ./input.css -o ./style/output.css --watch - errored


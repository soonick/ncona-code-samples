# Neovim as ESP32 IDE with clangd LSP

This shows an example of Neovim configured for ESP development inside a docker image.

To build the image:

```
docker build -t esp32-nvim-image .
```

To start a shell to the docker container:

```
docker run --rm -it \
    -v $(pwd)/blink:/blink \
    -v $(pwd)/nvim:/root/.config/nvim \
    -w /blink/ \
    esp32-nvim-image \
    sh -c " \
        . /esp-idf/export.sh && \
        mkdir -p build && \
        cd build && \
        cmake .. && \
        cd .. && \
        bash
    "
```

Inside the container, run:

```
nvim
```

The first time Neovim is started, it will need to download and install the plugins and `clangd` LSP, you can verify the installation of the LSP with `:LspInfo`, `:MasonLog` and `:messages`.

Once the LSP is installed, you can open main.c to see it in action:

```
:tabnew main/main.c
```

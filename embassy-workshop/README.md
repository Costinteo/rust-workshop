# Embedded Embassy RS

Welcome to the Embedded track. At this workshop, we will make our first steps with `Rust` and `embassy-rs`.

## Prerequisites

1) The Rust toolchain. In order to install it, follow the tutorial [here](https://www.rust-lang.org/tools/install).

    > If you are using `vscode`, install the `rust-analyzer` extension in the **Marketplace**.

    > Open the `.vscode/settings.json` and familiarize yourself with the analyzer settings.

2) Install `elf2uf2-rs` using `cargo`. Run

```shell
cargo install elf2uf2-rs
```

in terminal.

## How to flash

In the `embassy` directory, go to the provided examples.
We will compile the `wifi_blinky.rs` example and flash it on the boards.

* Go to `examples/rp`.

* Build the appropriate example. Run

```shell
cargo build --bin wifi_blinky
```

* Connect the board to your PC (while connecting the USB, press the **BOOTSEL** button on the microcontroller), then run `elf2uf2-rs -d target/thumbv6m-none-eabi/debug/wifi_blinky` in order to flash it. (In case it fails, dragging and dropping the `wifi_blinky.uf2` file in the `RPI-RPI2` should work)

## Tasks

### Task 1 - Hello Embassy! 🚀

For this task, you will need to use the builtin breadboard on the **Pico Explorer** to connect the two provided LEDs the following way:

* The *RED* LED should be connected to `GP0`
* The *GREEN* LED should be connected to `GP1`

Go to `embassy/embassy-workshop/src/bin/hello_embassy.rs`.
You will need to modify the source file so that the builtin buttons will control the LEDs in the following way:

* Button `A` should toggle the *RED* LED.
* Button `B` should toggle the *GREEN* LED.

### Task 2 - Echo echo echo... 📣

For this task, we want to build a small **echo** server using `TCP Sockets`.

Go to `embassy/embassy-workshop/src/bin/echo.rs`.
You will need to modify the source file so that the **Pi** will listen on port `123{your_team's_number}`. When a client connects, the server should respond to every message by sending it back on the socket.

In order to test the application, you can use `netcat`.

### Task 3 - The semaphore 🚥

For this task, you will need to pair with another team. One team will be the **semaphore** and the other one will be the pedestrian's **controller**. They should work together like this:

* The semaphore will be always *RED* until a pedestrian interacts with the controller.
* When the pedestrian wants to cross the road, he will press the button, and after a delay, the semaphore will turn *GREEN*. (To signal that the semaphore is waiting to turn *GREEN*, the debug LED present on the board should be blinking during this delay period). When the pedestrian should cross the semaphore should send a message to the pedestrian controller.
* To signal that the pedestrian should cross, the debug LED on the pedestrian controller should also blink.

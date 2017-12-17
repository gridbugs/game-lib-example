fetch("web_frontend.wasm").then(response =>
    response.arrayBuffer()
).then(bytes =>
    WebAssembly.instantiate(bytes, {})
).then(results => {
    let game = results.instance.exports;
    const STATE = game.alloc_state();

    const NUM_KEY_CODES = 256;
    const NO_KEY = 255;
    const NUM_INPUTS = 8;

    const INPUT_BUFFER = game.alloc_buffer(NUM_INPUTS);
    const input_buffer = new Uint8ClampedArray(game.memory.buffer,
                                               INPUT_BUFFER,
                                               NUM_INPUTS);

    const input_state = new Uint8Array(NUM_INPUTS);
    function clear_input_state() {
        for (let i = 0; i < NUM_INPUTS; i++) {
            input_state[i] = 0;
        }
    }
    clear_input_state();

    const key_codes = {
        UP: 38,
        DOWN: 40,
        LEFT: 37,
        RIGHT: 39,
        RETURN: 13,
    };

    const input = {
        UP: 0,
        DOWN: 1,
        LEFT: 2,
        RIGHT: 3,
        RETURN: 4,
    };

    const KEY_MAP = new Uint8Array(NUM_KEY_CODES);
    for (let i = 0; i < NUM_KEY_CODES; i++) {
        KEY_MAP[i] = NO_KEY;
    }
    for (key in key_codes) {
        KEY_MAP[key_codes[key]] = input[key];
    }

    const WIDTH = game.width(STATE);
    const HEIGHT = game.height(STATE);
    const SIZE = WIDTH * HEIGHT;

    const ASCII_BUFFER = game.alloc_buffer(SIZE);
    const ascii_buffer = new Uint8ClampedArray(game.memory.buffer,
                                               ASCII_BUFFER,
                                               SIZE);

    let block_inputs = false;

    window.onkeydown = function(e) {
        input_state[KEY_MAP[e.keyCode]] = 1;
    };

    window.onkeyup = function(e) {
        block_inputs = false;
        input_state[KEY_MAP[e.keyCode]] = 0;
    };

    function render() {
        let count = 0;
        let s = "";
        for (let i = 0; i < HEIGHT; i++) {
            for (let j = 0; j < WIDTH; j++) {
                s += String.fromCharCode(ascii_buffer[count]);
                count += 1;
            }
            s += "</br>";
        }
        display.innerHTML = s;
    }

    function buffer_inputs() {
        let count = 0;
        for (let i = 0; i < NUM_INPUTS; i++) {
            if (input_state[i] == 1) {
                input_buffer[count] = i;
                count += 1;
            }
        }
        return count;
    }

    let previous_instant = Date.now();

    function frame() {
        let now = Date.now();
        let period = now - previous_instant;
        previous_instant = now;

        game.buffer_ascii(STATE, ASCII_BUFFER, SIZE);
        render();

        let num_inputs = buffer_inputs();
        clear_input_state();
        game.tick(STATE, INPUT_BUFFER, num_inputs, period);

        requestAnimationFrame(frame);
    }

    requestAnimationFrame(frame);
});

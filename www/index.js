import init, { greet } from 'snake_game';

init().then(res => {
    greet('wasm extern function is working truly')
    console.log('hi')
})
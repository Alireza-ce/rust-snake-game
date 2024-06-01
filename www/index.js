import init, { World } from 'snake_game';

init().then(res => {
    const world = World.new();
    console.log(world.width());
})
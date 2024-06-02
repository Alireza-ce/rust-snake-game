import init, { World } from 'snake_game';



function drawWorld(width, cellSize, snakeIdx){
    const canvas = document.getElementById('board-game-canvas');
    const ctx = canvas.getContext('2d');
    canvas.height = width * cellSize;
    canvas.width = width * cellSize;

    ctx.beginPath();

    for (let x = 0 ; x < width + 1; x++){
        ctx.moveTo(cellSize * x,0);
        ctx.lineTo(cellSize * x , width  * cellSize)
    }

    for (let y = 0 ; y < width + 1; y++){
       ctx.moveTo( 0,cellSize * y);
       ctx.lineTo(width  * cellSize ,cellSize * y )
    }

    ctx.stroke();

    const col = snakeIdx % width;
    const row = Math.floor(snakeIdx / width);

    ctx.beginPath();
    ctx.fillRect(
        col * cellSize,
        row * cellSize,
        cellSize,
        cellSize
    )

}


function main(){
    const cellSize = 20;
    init().then(res => {
        const world = World.new();
        console.log(world.width());
        drawWorld(world.width(),cellSize, world.snake_head());
    })
}


main();
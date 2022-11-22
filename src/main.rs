use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use rand::Rng;
//A snake and a Tail
//A part of the snake's tail
struct Tail {
    pos: Vector2,
    velocity: Vector2,
}
//Allows the tail to move and return the velocity vector
impl Tail{
    pub fn move_tail(&mut self, new_velocity: Vector2, ) {
        self.pos += self.velocity;
        self.velocity = new_velocity;
        //Allows the tail to wrap around the stage
        if self.pos.x >= 1920.0 {
            self.pos.x = 0.0;
        }
        if self.pos.x < 0.0 {
            self.pos.x = 1900.0;
        }
        if self.pos.y >= 1080.0 {
            self.pos.y = 0.0;
        }
        if self.pos.y < 0.0 {
            self.pos.y = 1060.0;
        }
    }
    pub fn get_velocity(&self) -> &Vector2
    {
        return &self.velocity;
    }
    pub fn get_position(&self) -> &Vector2
    {
        return &self.pos;
    }
}
//A snake that has a head and a tail
struct Snake {
    head: Vector2,
    tail: Vec<Tail>,
}
//Move the snake and return the positions
impl Snake {
    pub fn move_snake(&mut self, mut velocity: Vector2)
    {
        self.head += velocity;
        //Move each segment of the tail
        for segment in self.tail.iter_mut() {
            let store = *segment.get_velocity();
            segment.move_tail(velocity);
            velocity = store;
        }
        //Allows the head to wrap around the stage
        if self.head.x >= 1920.0 {
            self.head.x = 0.0;
        }
        if self.head.x < 0.0 {
            self.head.x = 1900.0;
        }
        if self.head.y >= 1080.0 {
            self.head.y = 0.0;
        }
        if self.head.y < 0.0 {
            self.head.y = 1060.0;
        }
    }

    //Add a segment to the tail
    pub fn add_segment(&mut self)
    {
        self.tail.push(Tail{pos: self.tail.last().unwrap().pos, velocity: Vector2::new(0.0, 0.0)})
    }
    pub fn get_head(&self) -> &Vector2
    {
        return &self.head;
    }

    pub fn get_tail(&self) -> &Vec<Tail>
    {
        return &self.tail;
    }
}
//An Apple to be eaten
struct Apple {
    pos: Vector2,
}

//Main contains the initialization and the game loop.
fn main()
{
    //Create an rng object
    let mut rng = rand::thread_rng();
    //Create a snake
    let mut snake = Snake{head: Vector2::new(240.0, 240.0),
        tail: vec![Tail{pos: Vector2::new(240.0, 260.0), velocity: Vector2::new(0.0, -20.0)}]};
    //Create the current velocity vector of the snake
    let mut current_velocity = Vector2::new(0.0, -20.0);
    //Create an apple in a random location
    let mut apple = Apple{pos: Vector2::new((rng.gen_range(0..96) * 20) as f32, (rng.gen_range(0..54) * 20) as f32)};
    //Initialize the window
    let (mut rl, thread) = init()
        .size(1920, 1080)
        .title("Snake")
        .build();
    //Set the fps
    rl.set_target_fps(60);
    //Boolean to store if the snake is dead or not
    let mut dead: bool = false;
    //Game loop
    while !rl.window_should_close() {
        //Get a new velocity
        let new_velocity = velocity(current_velocity, &mut rl);
        //Make sure the new velocity is not the reverse of the old velocity
        //so that the snake isn't able to go directly backwards, ending the game
        if current_velocity.x != -new_velocity.x ||  current_velocity.y != -new_velocity.y
        {
            current_velocity = new_velocity;
        }


        //Check if the snake's head is colliding with the apple
        if snake.get_head().x == apple.pos.x && snake.get_head().y == apple.pos.y
        {
            snake.add_segment();
            apple = Apple{pos: Vector2::new((rng.gen_range(0..96) * 20) as f32, (rng.gen_range(0..54) * 20) as f32)};
        }

        //Check if the snake's head is colliding with any segment of the tail
        for segment in snake.get_tail()
        {
            if snake.get_head().x == segment.pos.x && snake.get_head().y == segment.pos.y
            {
                dead = true;
            }
        }

        //Check if the apple is colliding with the tail (to move it elsewhere)
        for segment in snake.get_tail()
        {
            if apple.pos.x == segment.pos.x && apple.pos.y == segment.pos.y
            {
                apple = Apple{pos: Vector2::new((rng.gen_range(0..96) * 20) as f32, (rng.gen_range(0..54) * 20) as f32)};
            }
        }

        //If the snake is dead, set the current velocity to 0
        if dead == true
        {
            current_velocity = Vector2::new(0.0, 0.0);
        }

        //Move the snake's head and tail
        snake.move_snake(current_velocity);

        //Start drawing stuff
        let mut d = rl.begin_drawing(&thread);
        //Clear the background
        d.clear_background(Color::BLACK);
        //Draw the snake's head
        d.draw_text("0", snake.get_head().x as i32, snake.get_head().y as i32, 20, Color::GREEN );
        //Draw the apple
        d.draw_text("@", apple.pos.x as i32, apple.pos.y as i32, 20, Color::RED);
        //Draw the tail
        for tail in snake.get_tail()
        {
            d.draw_text("#", tail.get_position().x as i32, tail.get_position().y as i32, 20, Color::BLUE);
        }
        //Draw the current score
        d.draw_text(&format!("Score: {}", snake.get_tail().len() - 1), 10, 10, 40, Color::YELLOW);

        //Draw the game over message if the snake is dead
        if dead
        {
            d.draw_text("Game Over", 650, 10, 40, Color::RED);
        }
    }
}

//Checks the keyboard input and returns a velocity vector corresponding to it
fn velocity(last_vector: Vector2, rl: &mut RaylibHandle) -> Vector2
{
    //Return match returns a vector2 based on what key is being pressed
    return match rl.get_key_pressed()
    {
        Some(KEY_UP) => Vector2::new(0.0, -20.0),
        Some(KEY_DOWN) => Vector2::new(0.0, 20.0),
        Some(KEY_LEFT) => Vector2::new(-20.0, 0.0),
        Some(KEY_RIGHT) => Vector2::new(20.0, 0.0),
        //If no key is being pressed, return the previous velocity vector (so the snake continues
        //moving in a direction)
        _ => last_vector,
    }
}